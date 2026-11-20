use std::sync::Arc;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferToImageInfo,
    PrimaryCommandBufferAbstract,
};
use vulkano::descriptor_set::layout::DescriptorSetLayout;
use vulkano::descriptor_set::{DescriptorSet, WriteDescriptorSet};
use vulkano::format::Format;
use vulkano::image::sampler::{Filter, Sampler, SamplerAddressMode, SamplerCreateInfo};
use vulkano::image::view::ImageView;
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter};
use vulkano::sync::GpuFuture;

use crate::context::VulkanContext;

pub fn create_texture_from_png(
    context: &VulkanContext,
    set_layout: Arc<DescriptorSetLayout>,
    png_bytes: &[u8],
) -> Result<Arc<DescriptorSet>, Box<dyn std::error::Error>> {
    let img = image::load_from_memory(png_bytes)?.to_rgba8();
    let (width, height) = img.dimensions();
    let raw_bytes = img.into_raw();

    let image = Image::new(
        context.memory_allocator.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_UNORM,
            extent: [width, height, 1],
            usage: ImageUsage::SAMPLED | ImageUsage::TRANSFER_DST,
            ..Default::default()
        },
        AllocationCreateInfo::default(),
    )?;

    let staging_buffer = Buffer::from_iter(
        context.memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_HOST
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        raw_bytes,
    )?;

    let mut builder = AutoCommandBufferBuilder::primary(
        context.cb_allocator.clone(),
        context.queue_family_index,
        CommandBufferUsage::OneTimeSubmit,
    )?;

    builder.copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(
        staging_buffer,
        image.clone(),
    ))?;

    let command_buffer = builder.build()?;
    command_buffer
        .execute(context.queue.clone())?
        .then_signal_fence_and_flush()?
        .wait(None)?;

    let view = ImageView::new_default(image)?;
    let sampler = Sampler::new(
        context.device.clone(),
        SamplerCreateInfo {
            mag_filter: Filter::Linear,
            min_filter: Filter::Linear,
            address_mode: [SamplerAddressMode::ClampToEdge; 3],
            ..Default::default()
        },
    )?;

    let descriptor_set = DescriptorSet::new(
        context.desc_allocator.clone(),
        set_layout,
        [WriteDescriptorSet::image_view_sampler(0, view, sampler)],
        [],
    )?;

    Ok(descriptor_set)
}
