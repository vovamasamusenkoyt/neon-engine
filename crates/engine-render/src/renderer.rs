use std::sync::Arc;
use glam::Mat4;
use vulkano::buffer::allocator::{SubbufferAllocator, SubbufferAllocatorCreateInfo};
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassBeginInfo,
    SubpassContents,
};
use vulkano::descriptor_set::DescriptorSet;
use vulkano::format::{ClearValue, Format};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter};
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::pipeline::PipelineBindPoint;
use vulkano::render_pass::Subpass;
use vulkano::swapchain::{self, SwapchainPresentInfo};
use vulkano::sync::GpuFuture;

use crate::context::VulkanContext;
use crate::overlay::build_text_overlay;
use crate::pipeline::RenderPipelines;
use crate::shaders::vs;
use crate::swapchain::SwapchainTarget;
use crate::texture::create_texture_from_png;
use crate::vertex::{Vertex2D, Vertex3D};

pub struct EngineRenderer {
    pub context: VulkanContext,
    pub target: SwapchainTarget,
    pub pipelines: RenderPipelines,
    pub cube_vbuf: Subbuffer<[Vertex3D]>,
    pub cube_ibuf: Subbuffer<[u32]>,
    pub overlay_allocator: SubbufferAllocator,
    pub menu_quad_vbuf: Subbuffer<[Vertex2D]>,
    pub menu_quad_ibuf: Subbuffer<[u32]>,
    pub mouse_allocator: SubbufferAllocator,
    pub menu_descriptor_sets: [Arc<DescriptorSet>; 3],
    pub mouse_descriptor_set: Arc<DescriptorSet>,
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

        let menu_verts = [
            Vertex2D::new([-1.0, -1.0], [0.0, 0.0]),
            Vertex2D::new([ 1.0, -1.0], [1.0, 0.0]),
            Vertex2D::new([ 1.0,  1.0], [1.0, 1.0]),
            Vertex2D::new([-1.0,  1.0], [0.0, 1.0]),
        ];
        let menu_inds = [0u32, 1, 2, 0, 2, 3];

        let menu_quad_vbuf = Buffer::from_iter(
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
            menu_verts,
        )?;

        let menu_quad_ibuf = Buffer::from_iter(
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
            menu_inds,
        )?;

        let mouse_allocator = SubbufferAllocator::new(
            context.memory_allocator.clone(),
            SubbufferAllocatorCreateInfo {
                buffer_usage: BufferUsage::VERTEX_BUFFER | BufferUsage::INDEX_BUFFER,
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
        );

        let set_layout = pipelines
            .textured_layout
            .set_layouts()
            .get(0)
            .ok_or("no set 0 in textured_layout")?
            .clone();

        let set_0 = create_texture_from_png(
            &context,
            set_layout.clone(),
            include_bytes!("../../../assets/generated/menu_0.png"),
        )?;
        let set_1 = create_texture_from_png(
            &context,
            set_layout.clone(),
            include_bytes!("../../../assets/generated/menu_1.png"),
        )?;
        let set_2 = create_texture_from_png(
            &context,
            set_layout.clone(),
            include_bytes!("../../../assets/generated/menu_2.png"),
        )?;
        let mouse_set = create_texture_from_png(
            &context,
            set_layout,
            include_bytes!("../../../assets/mouse.png"),
        )?;

        let previous_frame_end = Some(vulkano::sync::now(context.device.clone()).boxed());

        Ok(Self {
            context,
            target,
            pipelines,
            cube_vbuf,
            cube_ibuf,
            overlay_allocator,
            menu_quad_vbuf,
            menu_quad_ibuf,
            mouse_allocator,
            menu_descriptor_sets: [set_0, set_1, set_2],
            mouse_descriptor_set: mouse_set,
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
            self.draw_frame_with_overlay(mvp, false, 0, None, &v_overlay, &i_overlay)
        } else {
            self.draw_frame_with_overlay(mvp, false, 0, None, &[], &[])
        }
    }

    pub fn draw_frame_with_overlay(
        &mut self,
        mvp: Mat4,
        menu_active: bool,
        selected_option: usize,
        cursor_pos: Option<[f32; 2]>,
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
                        Some([0.0, 0.0, 0.0, 1.0].into()),
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

        if menu_active {
            // Draw Authentic 1:1 reVC Menu Quad
            let opt_idx = selected_option.min(2);
            let desc_set = self.menu_descriptor_sets[opt_idx].clone();

            builder.bind_pipeline_graphics(self.pipelines.textured_pipeline.clone())?;
            builder.bind_descriptor_sets(
                PipelineBindPoint::Graphics,
                self.pipelines.textured_layout.clone(),
                0,
                desc_set,
            )?;
            builder
                .bind_vertex_buffers(0, self.menu_quad_vbuf.clone())?
                .bind_index_buffer(self.menu_quad_ibuf.clone())?;

            unsafe {
                builder.draw_indexed(6, 1, 0, 0, 0)?;
            }

            // Draw Authentic Cursor if present
            if let Some(pos) = cursor_pos {
                let win_w = extent[0] as f32;
                let win_h = extent[1] as f32;
                let ndc_x = (pos[0] / win_w) * 2.0 - 1.0;
                let ndc_y = (pos[1] / win_h) * 2.0 - 1.0;
                let c_w = (32.0 / win_w) * 2.0;
                let c_h = (32.0 / win_h) * 2.0;

                let c_verts = [
                    Vertex2D::new([ndc_x, ndc_y], [0.0, 0.0]),
                    Vertex2D::new([ndc_x + c_w, ndc_y], [1.0, 0.0]),
                    Vertex2D::new([ndc_x + c_w, ndc_y + c_h], [1.0, 1.0]),
                    Vertex2D::new([ndc_x, ndc_y + c_h], [0.0, 1.0]),
                ];
                let c_inds = [0u32, 1, 2, 0, 2, 3];

                if let (Ok(vb), Ok(ib)) = (
                    self.mouse_allocator.allocate_slice::<Vertex2D>(4),
                    self.mouse_allocator.allocate_slice::<u32>(6),
                ) {
                    vb.write()?.copy_from_slice(&c_verts);
                    ib.write()?.copy_from_slice(&c_inds);

                    builder.bind_descriptor_sets(
                        PipelineBindPoint::Graphics,
                        self.pipelines.textured_layout.clone(),
                        0,
                        self.mouse_descriptor_set.clone(),
                    )?;
                    builder
                        .bind_vertex_buffers(0, vb)?
                        .bind_index_buffer(ib)?;
                    unsafe {
                        builder.draw_indexed(6, 1, 0, 0, 0)?;
                    }
                }
            }
        } else {
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
        }

        // Draw 2D HUD overlay (FPS / hints) on top
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
            Err(e) => return Err(e.into()),
        }

        Ok(())
    }
}

fn create_cube_mesh() -> (Vec<Vertex3D>, Vec<u32>) {
    let vertices = vec![
        Vertex3D::new([-0.5, -0.5, 0.5], [1.0, 0.2, 0.2]),
        Vertex3D::new([0.5, -0.5, 0.5], [0.2, 1.0, 0.2]),
        Vertex3D::new([0.5, 0.5, 0.5], [0.2, 0.2, 1.0]),
        Vertex3D::new([-0.5, 0.5, 0.5], [1.0, 1.0, 0.2]),
        Vertex3D::new([-0.5, -0.5, -0.5], [1.0, 0.2, 1.0]),
        Vertex3D::new([0.5, -0.5, -0.5], [0.2, 1.0, 1.0]),
        Vertex3D::new([0.5, 0.5, -0.5], [1.0, 1.0, 1.0]),
        Vertex3D::new([-0.5, 0.5, -0.5], [0.5, 0.5, 0.5]),
    ];

    let indices = vec![
        0, 1, 2, 2, 3, 0,
        1, 5, 6, 6, 2, 1,
        5, 4, 7, 7, 6, 5,
        4, 0, 3, 3, 7, 4,
        3, 2, 6, 6, 7, 3,
        4, 5, 1, 1, 0, 4,
    ];

    (vertices, indices)
}
