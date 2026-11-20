use std::sync::Arc;
use vulkano::format::Format;
use vulkano::image::view::ImageView;
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter};
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass};
use vulkano::swapchain::{PresentMode, Surface, Swapchain, SwapchainCreateInfo};

use crate::context::VulkanContext;

pub struct SwapchainTarget {
    pub surface: Arc<Surface>,
    pub swapchain: Arc<Swapchain>,
    pub images: Vec<Arc<Image>>,
    pub render_pass: Arc<RenderPass>,
    pub framebuffers: Vec<Arc<Framebuffer>>,
    pub depth_format: Format,
}

impl SwapchainTarget {
    pub fn new(
        ctx: &VulkanContext,
        surface: Arc<Surface>,
        initial_extent: Option<[u32; 2]>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let surface_caps = ctx.physical_device.surface_capabilities(&surface, Default::default())?;

        let dimensions = initial_extent
            .or(surface_caps.current_extent)
            .unwrap_or([1280, 720]);

        let (image_format, color_space) = ctx
            .physical_device
            .surface_formats(&surface, Default::default())?
            .first()
            .copied()
            .unwrap_or((Format::R8G8B8A8_UNORM, vulkano::swapchain::ColorSpace::SrgbNonLinear));

        let supported_modes = ctx
            .physical_device
            .surface_present_modes(&surface, Default::default())?;

        let present_mode = if supported_modes.contains(&PresentMode::Mailbox) {
            PresentMode::Mailbox
        } else if supported_modes.contains(&PresentMode::Immediate) {
            PresentMode::Immediate
        } else {
            PresentMode::Fifo
        };

        let (swapchain, images) = Swapchain::new(
            ctx.device.clone(),
            surface.clone(),
            SwapchainCreateInfo {
                min_image_count: surface_caps.min_image_count.max(3),
                image_format,
                image_color_space: color_space,
                image_extent: dimensions,
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                composite_alpha: surface_caps.supported_composite_alpha.into_iter().next().unwrap(),
                present_mode,
                ..Default::default()
            },
        )?;

        let render_pass = vulkano::single_pass_renderpass!(
            ctx.device.clone(),
            attachments: {
                color: {
                    format: image_format,
                    samples: 1,
                    load_op: Clear,
                    store_op: Store,
                },
                depth: {
                    format: ctx.depth_format,
                    samples: 1,
                    load_op: Clear,
                    store_op: DontCare,
                },
            },
            pass: {
                color: [color],
                depth_stencil: {depth},
            },
        )?;

        let depth_image = Image::new(
            ctx.memory_allocator.clone(),
            ImageCreateInfo {
                image_type: ImageType::Dim2d,
                format: ctx.depth_format,
                extent: [dimensions[0], dimensions[1], 1],
                usage: ImageUsage::DEPTH_STENCIL_ATTACHMENT,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                ..Default::default()
            },
        )?;
        let depth_view = ImageView::new_default(depth_image)?;

        let framebuffers = images
            .iter()
            .map(|img| {
                let view = ImageView::new_default(img.clone()).unwrap();
                Framebuffer::new(
                    render_pass.clone(),
                    FramebufferCreateInfo {
                        attachments: vec![view, depth_view.clone()],
                        ..Default::default()
                    },
                )
                .unwrap()
            })
            .collect();

        Ok(Self {
            surface,
            swapchain,
            images,
            render_pass,
            framebuffers,
            depth_format: ctx.depth_format,
        })
    }

    pub fn recreate(
        &mut self,
        ctx: &VulkanContext,
        dimensions: [u32; 2],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let surface_caps = ctx.physical_device.surface_capabilities(&self.surface, Default::default())?;
        let min_image_count = surface_caps.min_image_count.max(3);

        let (new_swapchain, new_images) = self.swapchain.recreate(SwapchainCreateInfo {
            image_extent: dimensions,
            min_image_count,
            ..self.swapchain.create_info()
        })?;

        let depth_image = Image::new(
            ctx.memory_allocator.clone(),
            ImageCreateInfo {
                image_type: ImageType::Dim2d,
                format: self.depth_format,
                extent: [dimensions[0], dimensions[1], 1],
                usage: ImageUsage::DEPTH_STENCIL_ATTACHMENT,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                ..Default::default()
            },
        )?;
        let depth_view = ImageView::new_default(depth_image)?;

        self.framebuffers = new_images
            .iter()
            .map(|img| {
                let view = ImageView::new_default(img.clone()).unwrap();
                Framebuffer::new(
                    self.render_pass.clone(),
                    FramebufferCreateInfo {
                        attachments: vec![view, depth_view.clone()],
                        ..Default::default()
                    },
                )
                .unwrap()
            })
            .collect();

        self.swapchain = new_swapchain;
        self.images = new_images;
        Ok(())
    }
}
