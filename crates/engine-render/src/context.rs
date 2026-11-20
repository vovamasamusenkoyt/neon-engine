use std::sync::Arc;
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags};
use vulkano::format::{Format, FormatFeatures};
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo, InstanceExtensions};
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::VulkanLibrary;

#[derive(Clone)]
pub struct VulkanContext {
    pub instance: Arc<Instance>,
    pub physical_device: Arc<PhysicalDevice>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub queue_family_index: u32,
    pub memory_allocator: Arc<StandardMemoryAllocator>,
    pub cb_allocator: Arc<StandardCommandBufferAllocator>,
    pub desc_allocator: Arc<StandardDescriptorSetAllocator>,
    pub depth_format: Format,
}

impl VulkanContext {
    pub fn new(instance_extensions: InstanceExtensions) -> Result<Self, Box<dyn std::error::Error>> {
        let library = VulkanLibrary::new()?;
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                enabled_extensions: instance_extensions,
                ..Default::default()
            },
        )?;

        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };

        let (physical_device, queue_family_index) = instance
            .enumerate_physical_devices()?
            .filter(|p| p.supported_extensions().contains(&device_extensions))
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(_, q)| q.queue_flags.intersects(QueueFlags::GRAPHICS))
                    .map(|i| (p, i as u32))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                _ => 2,
            })
            .ok_or("No suitable Vulkan physical device found")?;

        let depth_format = [
            Format::D24_UNORM_S8_UINT,
            Format::D32_SFLOAT,
            Format::D16_UNORM,
        ]
        .into_iter()
        .find(|&f| {
            physical_device
                .format_properties(f)
                .map(|p| p.optimal_tiling_features.contains(FormatFeatures::DEPTH_STENCIL_ATTACHMENT))
                .unwrap_or(false)
        })
        .unwrap_or(Format::D16_UNORM);

        log::info!("GPU: {}", physical_device.properties().device_name);
        log::info!("Depth format: {:?}", depth_format);

        let (device, mut queues) = Device::new(
            physical_device.clone(),
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                enabled_extensions: device_extensions,
                ..Default::default()
            },
        )?;

        let queue = queues.next().ok_or("No graphics queue available")?;
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        let cb_allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            Default::default(),
        ));
        let desc_allocator = Arc::new(StandardDescriptorSetAllocator::new(
            device.clone(),
            Default::default(),
        ));

        Ok(Self {
            instance,
            physical_device,
            device,
            queue,
            queue_family_index,
            memory_allocator,
            cb_allocator,
            desc_allocator,
            depth_format,
        })
    }
}
