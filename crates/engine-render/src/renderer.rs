use glam::Mat4;
use vulkano::buffer::allocator::{SubbufferAllocator, SubbufferAllocatorCreateInfo};
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassBeginInfo,
    SubpassContents,
};
use vulkano::format::{ClearValue, Format};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter};
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::render_pass::Subpass;
use vulkano::swapchain::{self, SwapchainPresentInfo};
use vulkano::sync::GpuFuture;

use crate::context::VulkanContext;
use crate::overlay::build_text_overlay;
use crate::pipeline::RenderPipelines;
use crate::shaders::vs;
use crate::swapchain::SwapchainTarget;
use crate::vertex::Vertex3D;

pub struct EngineRenderer {
    pub context: VulkanContext,
    pub target: SwapchainTarget,
    pub pipelines: RenderPipelines,
    pub cube_vbuf: Subbuffer<[Vertex3D]>,
    pub cube_ibuf: Subbuffer<[u32]>,
    pub overlay_allocator: SubbufferAllocator,
    pub previous_frame_end: Option<Box<dyn GpuFuture>>,
    pub recreate_needed: bool,
}

impl EngineRenderer {
    pub fn new(
        context: VulkanContext,
        target: SwapchainTarget,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let subpass = Subpass::from(target.render_pass.clone(), 0).ok_or("subpass 0 not found")?;
        let pipelines = RenderPipelines::new(context.device.clone(), subpass)?;

        let (cube_verts, cube_inds) = create_cube_mesh();
        let cube_vbuf = Buffer::from_iter(
            context.memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            cube_verts,
        )?;

        let cube_ibuf = Buffer::from_iter(
            context.memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::INDEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            cube_inds,
        )?;

        let overlay_allocator = SubbufferAllocator::new(
            context.memory_allocator.clone(),
            SubbufferAllocatorCreateInfo {
                buffer_usage: BufferUsage::VERTEX_BUFFER | BufferUsage::INDEX_BUFFER,
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
        );

        let previous_frame_end = Some(vulkano::sync::now(context.device.clone()).boxed());

        Ok(Self {
            context,
            target,
            pipelines,
            cube_vbuf,
            cube_ibuf,
            overlay_allocator,
            previous_frame_end,
            recreate_needed: false,
        })
    }

    pub fn resize(&mut self, dimensions: [u32; 2]) {
        if dimensions[0] == 0 || dimensions[1] == 0 {
            return;
        }
        if let Err(e) = self.target.recreate(&self.context, dimensions) {
            log::error!("Swapchain recreate error: {:?}", e);
        }
    }

    pub fn draw_frame(
        &mut self,
        mvp: Mat4,
        hud_text: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(text) = hud_text {
            let (v_overlay, i_overlay) = build_text_overlay(text);
            self.draw_frame_with_overlay(mvp, &v_overlay, &i_overlay)
        } else {
            self.draw_frame_with_overlay(mvp, &[], &[])
        }
    }

    pub fn draw_frame_with_overlay(
        &mut self,
        mvp: Mat4,
        overlay_verts: &[Vertex3D],
        overlay_inds: &[u32],
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.recreate_needed {
            self.recreate_needed = false;
            let extent = self.target.swapchain.image_extent();
            self.resize(extent);
        }

        if let Some(mut prev) = self.previous_frame_end.take() {
            prev.cleanup_finished();
            self.previous_frame_end = Some(prev);
        }

        let (image_index, suboptimal, acquire_future) = match swapchain::acquire_next_image(
            self.target.swapchain.clone(),
            None,
        ) {
            Ok(r) => r,
            Err(vulkano::Validated::Error(vulkano::VulkanError::OutOfDate)) => {
                self.recreate_needed = true;
                return Ok(());
            }
            Err(e) => return Err(e.into()),
        };
        let _ = suboptimal;

        let extent = self.target.swapchain.image_extent();

        let mut builder = AutoCommandBufferBuilder::primary(
            self.context.cb_allocator.clone(),
            self.context.queue_family_index,
            CommandBufferUsage::OneTimeSubmit,
        )?;

        let depth_clear = if self.target.depth_format == Format::D24_UNORM_S8_UINT {
            ClearValue::DepthStencil((1.0, 0))
        } else {
            ClearValue::Depth(1.0)
        };

        builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![
                        Some([0.05, 0.06, 0.08, 1.0].into()),
                        Some(depth_clear),
                    ],
                    ..RenderPassBeginInfo::framebuffer(
                        self.target.framebuffers[image_index as usize].clone(),
                    )
                },
                SubpassBeginInfo {
                    contents: SubpassContents::Inline,
                    ..Default::default()
                },
            )?
            .set_viewport(
                0,
                [Viewport {
                    offset: [0.0, 0.0],
                    extent: [extent[0] as f32, extent[1] as f32],
                    depth_range: 0.0..=1.0,
                }]
                .into_iter()
                .collect(),
            )?;

        // Draw 3D Scene with Depth Test
        builder.bind_pipeline_graphics(self.pipelines.cube_pipeline.clone())?;
        let cube_push = vs::PushConstants {
            mvp: mvp.to_cols_array_2d(),
        };
        builder
            .push_constants(self.pipelines.pipeline_layout.clone(), 0, cube_push)?
            .bind_vertex_buffers(0, self.cube_vbuf.clone())?
            .bind_index_buffer(self.cube_ibuf.clone())?;

        unsafe {
            builder.draw_indexed(36, 1, 0, 0, 0)?;
        }

        // Draw 2D HUD / Menu overlay
        if !overlay_inds.is_empty() {
            if let (Ok(vb), Ok(ib)) = (
                self.overlay_allocator.allocate_slice::<Vertex3D>(overlay_verts.len() as u64),
                self.overlay_allocator.allocate_slice::<u32>(overlay_inds.len() as u64),
            ) {
                vb.write()?.copy_from_slice(overlay_verts);
                ib.write()?.copy_from_slice(overlay_inds);

                builder.bind_pipeline_graphics(self.pipelines.overlay_pipeline.clone())?;
                let overlay_push = vs::PushConstants {
                    mvp: Mat4::IDENTITY.to_cols_array_2d(),
                };
                builder
                    .push_constants(self.pipelines.pipeline_layout.clone(), 0, overlay_push)?
                    .bind_vertex_buffers(0, vb)?
                    .bind_index_buffer(ib)?;

                unsafe {
                    builder.draw_indexed(overlay_inds.len() as u32, 1, 0, 0, 0)?;
                }
            }
        }

        builder.end_render_pass(Default::default())?;
        let command_buffer = builder.build()?;

        let future = self
            .previous_frame_end
            .take()
            .unwrap()
            .join(acquire_future)
            .then_execute(self.context.queue.clone(), command_buffer)?
            .then_swapchain_present(
                self.context.queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(
                    self.target.swapchain.clone(),
                    image_index,
                ),
            )
            .then_signal_fence_and_flush();

        match future {
            Ok(f) => {
                self.previous_frame_end = Some(f.boxed());
            }
            Err(vulkano::Validated::Error(vulkano::VulkanError::OutOfDate)) => {
                self.recreate_needed = true;
                self.previous_frame_end = Some(vulkano::sync::now(self.context.device.clone()).boxed());
            }
            Err(e) => {
                log::error!("Flush error: {:?}", e);
                self.previous_frame_end = Some(vulkano::sync::now(self.context.device.clone()).boxed());
            }
        }

        Ok(())
    }
}

pub fn create_cube_mesh() -> (Vec<Vertex3D>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let faces = [
        // Front (+Z): Cyan
        ([-1.0, -1.0, 1.0], [1.0, -1.0, 1.0], [1.0, 1.0, 1.0], [-1.0, 1.0, 1.0], [0.0, 0.95, 0.95]),
        // Back (-Z): Magenta
        ([1.0, -1.0, -1.0], [-1.0, -1.0, -1.0], [-1.0, 1.0, -1.0], [1.0, 1.0, -1.0], [0.95, 0.05, 0.95]),
        // Top (+Y): Yellow
        ([-1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, -1.0], [-1.0, 1.0, -1.0], [0.95, 0.95, 0.1]),
        // Bottom (-Y): Royal Blue
        ([-1.0, -1.0, -1.0], [1.0, -1.0, -1.0], [1.0, -1.0, 1.0], [-1.0, -1.0, 1.0], [0.1, 0.35, 0.95]),
        // Right (+X): Bright Orange
        ([1.0, -1.0, 1.0], [1.0, -1.0, -1.0], [1.0, 1.0, -1.0], [1.0, 1.0, 1.0], [1.0, 0.55, 0.05]),
        // Left (-X): Lime Green
        ([-1.0, -1.0, -1.0], [-1.0, -1.0, 1.0], [-1.0, 1.0, 1.0], [-1.0, 1.0, -1.0], [0.2, 0.95, 0.25]),
    ];

    for (p0, p1, p2, p3, c) in faces {
        let base_idx = vertices.len() as u32;
        vertices.push(Vertex3D::new(p0, c));
        vertices.push(Vertex3D::new(p1, c));
        vertices.push(Vertex3D::new(p2, c));
        vertices.push(Vertex3D::new(p3, c));

        indices.extend_from_slice(&[
            base_idx,
            base_idx + 1,
            base_idx + 2,
            base_idx,
            base_idx + 2,
            base_idx + 3,
        ]);
    }

    (vertices, indices)
}
