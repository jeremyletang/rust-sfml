use sfml::{
    SfResult,
    window::{
        Event, Key, Style, Window,
        vulkan::{self, VkAllocationCallbacks, VkInstance, VkSurfaceKHR, VulkanFunction},
    },
};
use std::{
    ffi::{CStr, c_char, c_void},
    mem, ptr,
};

// Colors that the window clears to. Each key press moves to the next one.
const COLORS: [[f32; 4]; 3] = [
    [0.9, 0.2, 0.2, 1.0],
    [0.2, 0.8, 0.3, 1.0],
    [0.2, 0.4, 0.95, 1.0],
];

// The Vulkan types, constants, and function signatures used here. A Vulkan binding crate would
// normally provide them. Non-dispatchable handles are 64-bit on every platform, and dispatchable
// handles are pointers.
type VkPhysicalDevice = *mut c_void;
type VkDevice = *mut c_void;
type VkQueue = *mut c_void;
type VkCommandBuffer = *mut c_void;
type VkSwapchainKHR = u64;
type VkImage = u64;
type VkSemaphore = u64;
type VkFence = u64;
type VkCommandPool = u64;

const STRUCTURE_TYPE_APPLICATION_INFO: i32 = 0;
const STRUCTURE_TYPE_INSTANCE_CREATE_INFO: i32 = 1;
const STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO: i32 = 2;
const STRUCTURE_TYPE_DEVICE_CREATE_INFO: i32 = 3;
const STRUCTURE_TYPE_SUBMIT_INFO: i32 = 4;
const STRUCTURE_TYPE_FENCE_CREATE_INFO: i32 = 8;
const STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO: i32 = 9;
const STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO: i32 = 39;
const STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO: i32 = 40;
const STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO: i32 = 42;
const STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER: i32 = 45;
const STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR: i32 = 1000001000;
const STRUCTURE_TYPE_PRESENT_INFO_KHR: i32 = 1000001001;
const QUEUE_GRAPHICS_BIT: u32 = 0x1;
const IMAGE_USAGE_TRANSFER_DST_BIT: u32 = 0x2;
const IMAGE_ASPECT_COLOR_BIT: u32 = 0x1;
const IMAGE_LAYOUT_UNDEFINED: i32 = 0;
const IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL: i32 = 7;
const IMAGE_LAYOUT_PRESENT_SRC_KHR: i32 = 1000001002;
const PIPELINE_STAGE_TOP_OF_PIPE_BIT: u32 = 0x1;
const PIPELINE_STAGE_TRANSFER_BIT: u32 = 0x1000;
const PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT: u32 = 0x2000;
const ACCESS_TRANSFER_WRITE_BIT: u32 = 0x1000;
const SHARING_MODE_EXCLUSIVE: i32 = 0;
const PRESENT_MODE_FIFO_KHR: i32 = 2;
const COMMAND_BUFFER_LEVEL_PRIMARY: i32 = 0;
const COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: u32 = 0x1;
const COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT: u32 = 0x2;
const FENCE_CREATE_SIGNALED_BIT: u32 = 0x1;
const QUEUE_FAMILY_IGNORED: u32 = u32::MAX;

#[repr(C)]
struct ApplicationInfo {
    s_type: i32,
    p_next: *const c_void,
    application_name: *const c_char,
    application_version: u32,
    engine_name: *const c_char,
    engine_version: u32,
    api_version: u32,
}

#[repr(C)]
struct InstanceCreateInfo {
    s_type: i32,
    p_next: *const c_void,
    flags: u32,
    application_info: *const ApplicationInfo,
    enabled_layer_count: u32,
    enabled_layer_names: *const *const c_char,
    enabled_extension_count: u32,
    enabled_extension_names: *const *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct QueueFamilyProperties {
    queue_flags: u32,
    queue_count: u32,
    timestamp_valid_bits: u32,
    min_image_transfer_granularity: [u32; 3],
}

#[repr(C)]
struct SurfaceCapabilities {
    min_image_count: u32,
    max_image_count: u32,
    current_extent: [u32; 2],
    min_image_extent: [u32; 2],
    max_image_extent: [u32; 2],
    max_image_array_layers: u32,
    supported_transforms: u32,
    current_transform: u32,
    supported_composite_alpha: u32,
    supported_usage_flags: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SurfaceFormat {
    format: i32,
    color_space: i32,
}

#[repr(C)]
struct DeviceQueueCreateInfo {
    s_type: i32,
    p_next: *const c_void,
    flags: u32,
    queue_family_index: u32,
    queue_count: u32,
    queue_priorities: *const f32,
}

#[repr(C)]
struct DeviceCreateInfo {
    s_type: i32,
    p_next: *const c_void,
    flags: u32,
    queue_create_info_count: u32,
    queue_create_infos: *const DeviceQueueCreateInfo,
    enabled_layer_count: u32,
    enabled_layer_names: *const *const c_char,
    enabled_extension_count: u32,
    enabled_extension_names: *const *const c_char,
    enabled_features: *const c_void,
}

#[repr(C)]
struct SwapchainCreateInfo {
    s_type: i32,
    p_next: *const c_void,
    flags: u32,
    surface: VkSurfaceKHR,
    min_image_count: u32,
    image_format: i32,
    image_color_space: i32,
    image_extent: [u32; 2],
    image_array_layers: u32,
    image_usage: u32,
    image_sharing_mode: i32,
    queue_family_index_count: u32,
    queue_family_indices: *const u32,
    pre_transform: u32,
    composite_alpha: u32,
    present_mode: i32,
    clipped: u32,
    old_swapchain: VkSwapchainKHR,
}

#[repr(C)]
struct CommandPoolCreateInfo {
    s_type: i32,
    p_next: *const c_void,
    flags: u32,
    queue_family_index: u32,
}

#[repr(C)]
struct CommandBufferAllocateInfo {
    s_type: i32,
    p_next: *const c_void,
    command_pool: VkCommandPool,
    level: i32,
    command_buffer_count: u32,
}

#[repr(C)]
struct CommandBufferBeginInfo {
    s_type: i32,
    p_next: *const c_void,
    flags: u32,
    inheritance_info: *const c_void,
}

// Used for both semaphores and fences, which have the same layout.
#[repr(C)]
struct SyncCreateInfo {
    s_type: i32,
    p_next: *const c_void,
    flags: u32,
}

#[repr(C)]
struct ImageSubresourceRange {
    aspect_mask: u32,
    base_mip_level: u32,
    level_count: u32,
    base_array_layer: u32,
    layer_count: u32,
}

#[repr(C)]
struct ImageMemoryBarrier {
    s_type: i32,
    p_next: *const c_void,
    src_access_mask: u32,
    dst_access_mask: u32,
    old_layout: i32,
    new_layout: i32,
    src_queue_family_index: u32,
    dst_queue_family_index: u32,
    image: VkImage,
    subresource_range: ImageSubresourceRange,
}

#[repr(C)]
struct SubmitInfo {
    s_type: i32,
    p_next: *const c_void,
    wait_semaphore_count: u32,
    wait_semaphores: *const VkSemaphore,
    wait_dst_stage_mask: *const u32,
    command_buffer_count: u32,
    command_buffers: *const VkCommandBuffer,
    signal_semaphore_count: u32,
    signal_semaphores: *const VkSemaphore,
}

#[repr(C)]
struct PresentInfo {
    s_type: i32,
    p_next: *const c_void,
    wait_semaphore_count: u32,
    wait_semaphores: *const VkSemaphore,
    swapchain_count: u32,
    swapchains: *const VkSwapchainKHR,
    image_indices: *const u32,
    results: *mut i32,
}

type GetInstanceProcAddr =
    unsafe extern "C" fn(VkInstance, *const c_char) -> Option<VulkanFunction>;
type CreateInstance = unsafe extern "C" fn(
    *const InstanceCreateInfo,
    *const VkAllocationCallbacks,
    *mut VkInstance,
) -> i32;
type EnumeratePhysicalDevices =
    unsafe extern "C" fn(VkInstance, *mut u32, *mut VkPhysicalDevice) -> i32;
type GetQueueFamilyProperties =
    unsafe extern "C" fn(VkPhysicalDevice, *mut u32, *mut QueueFamilyProperties);
type GetSurfaceSupport = unsafe extern "C" fn(VkPhysicalDevice, u32, VkSurfaceKHR, *mut u32) -> i32;
type GetSurfaceCapabilities =
    unsafe extern "C" fn(VkPhysicalDevice, VkSurfaceKHR, *mut SurfaceCapabilities) -> i32;
type GetSurfaceFormats =
    unsafe extern "C" fn(VkPhysicalDevice, VkSurfaceKHR, *mut u32, *mut SurfaceFormat) -> i32;
type CreateDevice = unsafe extern "C" fn(
    VkPhysicalDevice,
    *const DeviceCreateInfo,
    *const VkAllocationCallbacks,
    *mut VkDevice,
) -> i32;
type GetDeviceQueue = unsafe extern "C" fn(VkDevice, u32, u32, *mut VkQueue);
type CreateSwapchain = unsafe extern "C" fn(
    VkDevice,
    *const SwapchainCreateInfo,
    *const VkAllocationCallbacks,
    *mut VkSwapchainKHR,
) -> i32;
type GetSwapchainImages =
    unsafe extern "C" fn(VkDevice, VkSwapchainKHR, *mut u32, *mut VkImage) -> i32;
type CreateCommandPool = unsafe extern "C" fn(
    VkDevice,
    *const CommandPoolCreateInfo,
    *const VkAllocationCallbacks,
    *mut VkCommandPool,
) -> i32;
type AllocateCommandBuffers =
    unsafe extern "C" fn(VkDevice, *const CommandBufferAllocateInfo, *mut VkCommandBuffer) -> i32;
type CreateSync = unsafe extern "C" fn(
    VkDevice,
    *const SyncCreateInfo,
    *const VkAllocationCallbacks,
    *mut u64,
) -> i32;
type WaitForFences = unsafe extern "C" fn(VkDevice, u32, *const VkFence, u32, u64) -> i32;
type ResetFences = unsafe extern "C" fn(VkDevice, u32, *const VkFence) -> i32;
type AcquireNextImage =
    unsafe extern "C" fn(VkDevice, VkSwapchainKHR, u64, VkSemaphore, VkFence, *mut u32) -> i32;
type ResetCommandBuffer = unsafe extern "C" fn(VkCommandBuffer, u32) -> i32;
type BeginCommandBuffer =
    unsafe extern "C" fn(VkCommandBuffer, *const CommandBufferBeginInfo) -> i32;
type CmdPipelineBarrier = unsafe extern "C" fn(
    VkCommandBuffer,
    u32,
    u32,
    u32,
    u32,
    *const c_void,
    u32,
    *const c_void,
    u32,
    *const ImageMemoryBarrier,
);
type CmdClearColorImage = unsafe extern "C" fn(
    VkCommandBuffer,
    VkImage,
    i32,
    *const [f32; 4],
    u32,
    *const ImageSubresourceRange,
);
type EndCommandBuffer = unsafe extern "C" fn(VkCommandBuffer) -> i32;
type QueueSubmit = unsafe extern "C" fn(VkQueue, u32, *const SubmitInfo, VkFence) -> i32;
type QueuePresent = unsafe extern "C" fn(VkQueue, *const PresentInfo) -> i32;
type DeviceWaitIdle = unsafe extern "C" fn(VkDevice) -> i32;
type DestroySync = unsafe extern "C" fn(VkDevice, u64, *const VkAllocationCallbacks);
type DestroyCommandPool =
    unsafe extern "C" fn(VkDevice, VkCommandPool, *const VkAllocationCallbacks);
type DestroySwapchain =
    unsafe extern "C" fn(VkDevice, VkSwapchainKHR, *const VkAllocationCallbacks);
type DestroyDevice = unsafe extern "C" fn(VkDevice, *const VkAllocationCallbacks);
type DestroySurface = unsafe extern "C" fn(VkInstance, VkSurfaceKHR, *const VkAllocationCallbacks);
type DestroyInstance = unsafe extern "C" fn(VkInstance, *const VkAllocationCallbacks);

macro_rules! vulkan_api {
    ($($field:ident: $name:expr => $ty:ty),* $(,)?) => {
        struct Api {
            $($field: $ty),*
        }
        impl Api {
            fn load(get_instance_proc_addr: GetInstanceProcAddr, instance: VkInstance) -> Self {
                Self {
                    $($field: unsafe {
                        mem::transmute::<VulkanFunction, $ty>(require(
                            get_instance_proc_addr,
                            instance,
                            $name,
                        ))
                    }),*
                }
            }
        }
    };
}

vulkan_api! {
    enumerate_physical_devices: c"vkEnumeratePhysicalDevices" => EnumeratePhysicalDevices,
    get_queue_family_properties: c"vkGetPhysicalDeviceQueueFamilyProperties" => GetQueueFamilyProperties,
    get_surface_support: c"vkGetPhysicalDeviceSurfaceSupportKHR" => GetSurfaceSupport,
    get_surface_capabilities: c"vkGetPhysicalDeviceSurfaceCapabilitiesKHR" => GetSurfaceCapabilities,
    get_surface_formats: c"vkGetPhysicalDeviceSurfaceFormatsKHR" => GetSurfaceFormats,
    create_device: c"vkCreateDevice" => CreateDevice,
    get_device_queue: c"vkGetDeviceQueue" => GetDeviceQueue,
    create_swapchain: c"vkCreateSwapchainKHR" => CreateSwapchain,
    get_swapchain_images: c"vkGetSwapchainImagesKHR" => GetSwapchainImages,
    create_command_pool: c"vkCreateCommandPool" => CreateCommandPool,
    allocate_command_buffers: c"vkAllocateCommandBuffers" => AllocateCommandBuffers,
    create_semaphore: c"vkCreateSemaphore" => CreateSync,
    create_fence: c"vkCreateFence" => CreateSync,
    wait_for_fences: c"vkWaitForFences" => WaitForFences,
    reset_fences: c"vkResetFences" => ResetFences,
    acquire_next_image: c"vkAcquireNextImageKHR" => AcquireNextImage,
    reset_command_buffer: c"vkResetCommandBuffer" => ResetCommandBuffer,
    begin_command_buffer: c"vkBeginCommandBuffer" => BeginCommandBuffer,
    cmd_pipeline_barrier: c"vkCmdPipelineBarrier" => CmdPipelineBarrier,
    cmd_clear_color_image: c"vkCmdClearColorImage" => CmdClearColorImage,
    end_command_buffer: c"vkEndCommandBuffer" => EndCommandBuffer,
    queue_submit: c"vkQueueSubmit" => QueueSubmit,
    queue_present: c"vkQueuePresentKHR" => QueuePresent,
    device_wait_idle: c"vkDeviceWaitIdle" => DeviceWaitIdle,
    destroy_semaphore: c"vkDestroySemaphore" => DestroySync,
    destroy_fence: c"vkDestroyFence" => DestroySync,
    destroy_command_pool: c"vkDestroyCommandPool" => DestroyCommandPool,
    destroy_swapchain: c"vkDestroySwapchainKHR" => DestroySwapchain,
    destroy_device: c"vkDestroyDevice" => DestroyDevice,
    destroy_surface: c"vkDestroySurfaceKHR" => DestroySurface,
    destroy_instance: c"vkDestroyInstance" => DestroyInstance,
}

fn require(
    get_instance_proc_addr: GetInstanceProcAddr,
    instance: VkInstance,
    name: &CStr,
) -> VulkanFunction {
    unsafe { get_instance_proc_addr(instance, name.as_ptr()) }
        .unwrap_or_else(|| panic!("{name:?} was not found"))
}

fn check(result: i32, what: &str) {
    assert!(result >= 0, "{what} failed with VkResult {result}");
}

// Calls a Vulkan function twice, first to get the count and then to get the items.
fn query_list<T: Copy>(mut query: impl FnMut(*mut u32, *mut T) -> i32, what: &str) -> Vec<T> {
    let mut count = 0;
    check(query(&mut count, ptr::null_mut()), what);
    // SAFETY: the items are raw pointers or plain C structs, for which all-zero bytes are valid.
    let zeroed: T = unsafe { mem::zeroed() };
    let mut items = vec![zeroed; count as usize];
    check(query(&mut count, items.as_mut_ptr()), what);
    items.truncate(count as usize);
    items
}

// Finds a physical device with a graphics queue family that can present to the surface.
fn pick_device(
    api: &Api,
    instance: VkInstance,
    surface: VkSurfaceKHR,
) -> Option<(VkPhysicalDevice, u32)> {
    let devices: Vec<VkPhysicalDevice> = query_list(
        |count, devices| unsafe { (api.enumerate_physical_devices)(instance, count, devices) },
        "vkEnumeratePhysicalDevices",
    );
    for device in devices {
        let families: Vec<QueueFamilyProperties> = query_list(
            |count, families| {
                unsafe { (api.get_queue_family_properties)(device, count, families) };
                0
            },
            "vkGetPhysicalDeviceQueueFamilyProperties",
        );
        for (index, family) in families.iter().enumerate() {
            let index = index as u32;
            let mut supported = 0u32;
            check(
                unsafe { (api.get_surface_support)(device, index, surface, &mut supported) },
                "vkGetPhysicalDeviceSurfaceSupportKHR",
            );
            if family.queue_flags & QUEUE_GRAPHICS_BIT != 0 && supported != 0 {
                return Some((device, index));
            }
        }
    }
    None
}

fn color_range() -> ImageSubresourceRange {
    ImageSubresourceRange {
        aspect_mask: IMAGE_ASPECT_COLOR_BIT,
        base_mip_level: 0,
        level_count: 1,
        base_array_layer: 0,
        layer_count: 1,
    }
}

fn image_barrier(
    image: VkImage,
    old_layout: i32,
    new_layout: i32,
    src_access: u32,
    dst_access: u32,
) -> ImageMemoryBarrier {
    ImageMemoryBarrier {
        s_type: STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
        p_next: ptr::null(),
        src_access_mask: src_access,
        dst_access_mask: dst_access,
        old_layout,
        new_layout,
        src_queue_family_index: QUEUE_FAMILY_IGNORED,
        dst_queue_family_index: QUEUE_FAMILY_IGNORED,
        image,
        subresource_range: color_range(),
    }
}

fn main() -> SfResult<()> {
    if !vulkan::is_available(true) {
        println!("Vulkan is not available on this system");
        return Ok(());
    }
    let Some(address) = vulkan::get_function(c"vkGetInstanceProcAddr") else {
        println!("vkGetInstanceProcAddr was not found");
        return Ok(());
    };
    // SAFETY: vkGetInstanceProcAddr has this signature in the Vulkan specification.
    let get_instance_proc_addr: GetInstanceProcAddr = unsafe { mem::transmute(address) };

    // A null instance looks up the global functions, such as vkCreateInstance.
    // SAFETY: vkCreateInstance has this signature in the Vulkan specification.
    let create_instance: CreateInstance = unsafe {
        mem::transmute(require(
            get_instance_proc_addr,
            ptr::null_mut(),
            c"vkCreateInstance",
        ))
    };

    let extensions = vulkan::graphics_required_instance_extensions();
    let extension_names: Vec<*const c_char> = extensions.iter().map(|name| name.as_ptr()).collect();
    let app_info = ApplicationInfo {
        s_type: STRUCTURE_TYPE_APPLICATION_INFO,
        p_next: ptr::null(),
        application_name: c"sfml-vulkan".as_ptr(),
        application_version: 0,
        engine_name: ptr::null(),
        engine_version: 0,
        api_version: 0,
    };
    let create_info = InstanceCreateInfo {
        s_type: STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        p_next: ptr::null(),
        flags: 0,
        application_info: &app_info,
        enabled_layer_count: 0,
        enabled_layer_names: ptr::null(),
        enabled_extension_count: extension_names.len() as u32,
        enabled_extension_names: extension_names.as_ptr(),
    };
    let mut instance: VkInstance = ptr::null_mut();
    let result = unsafe { create_instance(&create_info, ptr::null(), &mut instance) };
    if result != 0 {
        println!("vkCreateInstance failed with VkResult {result}");
        return Ok(());
    }
    println!("Created Vulkan instance {instance:?} with extensions {extensions:?}");

    let mut window = Window::new(
        (800, 600),
        "Vulkan window",
        Style::CLOSE,
        Default::default(),
        &Default::default(),
    )?;
    let surface = unsafe { window.create_vulkan_surface(instance, None) }?;
    println!("Created Vulkan surface {surface:?} for the window");
    let api = Api::load(get_instance_proc_addr, instance);

    let Some((physical_device, family)) = pick_device(&api, instance, surface) else {
        println!("No physical device can draw to and present on the window");
        unsafe {
            (api.destroy_surface)(instance, surface, ptr::null());
            (api.destroy_instance)(instance, ptr::null());
        }
        return Ok(());
    };
    let mut caps: SurfaceCapabilities = unsafe { mem::zeroed() };
    check(
        unsafe { (api.get_surface_capabilities)(physical_device, surface, &mut caps) },
        "vkGetPhysicalDeviceSurfaceCapabilitiesKHR",
    );
    assert!(
        caps.supported_usage_flags & IMAGE_USAGE_TRANSFER_DST_BIT != 0,
        "the swapchain images can't be cleared on this system"
    );
    let formats: Vec<SurfaceFormat> = query_list(
        |count, formats| unsafe {
            (api.get_surface_formats)(physical_device, surface, count, formats)
        },
        "vkGetPhysicalDeviceSurfaceFormatsKHR",
    );
    let format = formats[0];

    let priority = 1.0f32;
    let queue_info = DeviceQueueCreateInfo {
        s_type: STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
        p_next: ptr::null(),
        flags: 0,
        queue_family_index: family,
        queue_count: 1,
        queue_priorities: &priority,
    };
    let swapchain_extension = [c"VK_KHR_swapchain".as_ptr()];
    let device_info = DeviceCreateInfo {
        s_type: STRUCTURE_TYPE_DEVICE_CREATE_INFO,
        p_next: ptr::null(),
        flags: 0,
        queue_create_info_count: 1,
        queue_create_infos: &queue_info,
        enabled_layer_count: 0,
        enabled_layer_names: ptr::null(),
        enabled_extension_count: 1,
        enabled_extension_names: swapchain_extension.as_ptr(),
        enabled_features: ptr::null(),
    };
    let mut device: VkDevice = ptr::null_mut();
    check(
        unsafe { (api.create_device)(physical_device, &device_info, ptr::null(), &mut device) },
        "vkCreateDevice",
    );
    let mut queue: VkQueue = ptr::null_mut();
    unsafe { (api.get_device_queue)(device, family, 0, &mut queue) };

    // Keep one spare image when the surface allows it. A current extent of u32::MAX means the
    // application picks the size. The window isn't resizable, so use its initial size.
    let mut image_count = caps.min_image_count + 1;
    if caps.max_image_count > 0 {
        image_count = image_count.min(caps.max_image_count);
    }
    let extent = if caps.current_extent[0] == u32::MAX {
        [800, 600]
    } else {
        caps.current_extent
    };
    // Keeps the lowest composite alpha mode that the surface supports.
    let composite_alpha =
        caps.supported_composite_alpha & caps.supported_composite_alpha.wrapping_neg();
    let swapchain_info = SwapchainCreateInfo {
        s_type: STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
        p_next: ptr::null(),
        flags: 0,
        surface,
        min_image_count: image_count,
        image_format: format.format,
        image_color_space: format.color_space,
        image_extent: extent,
        image_array_layers: 1,
        image_usage: IMAGE_USAGE_TRANSFER_DST_BIT,
        image_sharing_mode: SHARING_MODE_EXCLUSIVE,
        queue_family_index_count: 0,
        queue_family_indices: ptr::null(),
        pre_transform: caps.current_transform,
        composite_alpha,
        present_mode: PRESENT_MODE_FIFO_KHR,
        clipped: 1,
        old_swapchain: 0,
    };
    let mut swapchain: VkSwapchainKHR = 0;
    check(
        unsafe { (api.create_swapchain)(device, &swapchain_info, ptr::null(), &mut swapchain) },
        "vkCreateSwapchainKHR",
    );
    let images: Vec<VkImage> = query_list(
        |count, images| unsafe { (api.get_swapchain_images)(device, swapchain, count, images) },
        "vkGetSwapchainImagesKHR",
    );

    let pool_info = CommandPoolCreateInfo {
        s_type: STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
        p_next: ptr::null(),
        flags: COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
        queue_family_index: family,
    };
    let mut command_pool: VkCommandPool = 0;
    check(
        unsafe { (api.create_command_pool)(device, &pool_info, ptr::null(), &mut command_pool) },
        "vkCreateCommandPool",
    );
    let allocate_info = CommandBufferAllocateInfo {
        s_type: STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
        p_next: ptr::null(),
        command_pool,
        level: COMMAND_BUFFER_LEVEL_PRIMARY,
        command_buffer_count: 1,
    };
    let mut command_buffer: VkCommandBuffer = ptr::null_mut();
    check(
        unsafe { (api.allocate_command_buffers)(device, &allocate_info, &mut command_buffer) },
        "vkAllocateCommandBuffers",
    );

    let semaphore_info = SyncCreateInfo {
        s_type: STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
        p_next: ptr::null(),
        flags: 0,
    };
    let fence_info = SyncCreateInfo {
        s_type: STRUCTURE_TYPE_FENCE_CREATE_INFO,
        p_next: ptr::null(),
        flags: FENCE_CREATE_SIGNALED_BIT,
    };
    let mut image_available: VkSemaphore = 0;
    let mut fence: VkFence = 0;
    check(
        unsafe {
            (api.create_semaphore)(device, &semaphore_info, ptr::null(), &mut image_available)
        },
        "vkCreateSemaphore",
    );
    // One semaphore per swapchain image, so that a presentation can't still be waiting on the
    // semaphore that the next submission signals.
    let render_finished: Vec<VkSemaphore> = (0..images.len())
        .map(|_| {
            let mut semaphore: VkSemaphore = 0;
            check(
                unsafe {
                    (api.create_semaphore)(device, &semaphore_info, ptr::null(), &mut semaphore)
                },
                "vkCreateSemaphore",
            );
            semaphore
        })
        .collect();
    check(
        unsafe { (api.create_fence)(device, &fence_info, ptr::null(), &mut fence) },
        "vkCreateFence",
    );

    println!("Press any key to change the color. Close the window or press Escape to exit");
    let begin_info = CommandBufferBeginInfo {
        s_type: STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
        p_next: ptr::null(),
        flags: COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
        inheritance_info: ptr::null(),
    };
    let wait_stage = PIPELINE_STAGE_TOP_OF_PIPE_BIT;
    let mut color_index = 0;
    let mut running = true;
    while running {
        window.handle_events(|event| match event {
            Event::Closed
            | Event::KeyPressed {
                code: Key::Escape, ..
            } => running = false,
            Event::KeyPressed { .. } => color_index = (color_index + 1) % COLORS.len(),
            _ => {}
        });

        check(
            unsafe { (api.wait_for_fences)(device, 1, &fence, 1, u64::MAX) },
            "vkWaitForFences",
        );
        check(
            unsafe { (api.reset_fences)(device, 1, &fence) },
            "vkResetFences",
        );
        let mut index = 0u32;
        check(
            unsafe {
                (api.acquire_next_image)(
                    device,
                    swapchain,
                    u64::MAX,
                    image_available,
                    0,
                    &mut index,
                )
            },
            "vkAcquireNextImageKHR",
        );
        let image = images[index as usize];

        let to_clear = image_barrier(
            image,
            IMAGE_LAYOUT_UNDEFINED,
            IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
            0,
            ACCESS_TRANSFER_WRITE_BIT,
        );
        let to_present = image_barrier(
            image,
            IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
            IMAGE_LAYOUT_PRESENT_SRC_KHR,
            ACCESS_TRANSFER_WRITE_BIT,
            0,
        );
        let range = color_range();
        check(
            unsafe { (api.reset_command_buffer)(command_buffer, 0) },
            "vkResetCommandBuffer",
        );
        check(
            unsafe { (api.begin_command_buffer)(command_buffer, &begin_info) },
            "vkBeginCommandBuffer",
        );
        unsafe {
            (api.cmd_pipeline_barrier)(
                command_buffer,
                PIPELINE_STAGE_TOP_OF_PIPE_BIT,
                PIPELINE_STAGE_TRANSFER_BIT,
                0,
                0,
                ptr::null(),
                0,
                ptr::null(),
                1,
                &to_clear,
            );
            (api.cmd_clear_color_image)(
                command_buffer,
                image,
                IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                &COLORS[color_index],
                1,
                &range,
            );
            (api.cmd_pipeline_barrier)(
                command_buffer,
                PIPELINE_STAGE_TRANSFER_BIT,
                PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT,
                0,
                0,
                ptr::null(),
                0,
                ptr::null(),
                1,
                &to_present,
            );
        }
        check(
            unsafe { (api.end_command_buffer)(command_buffer) },
            "vkEndCommandBuffer",
        );

        let submit = SubmitInfo {
            s_type: STRUCTURE_TYPE_SUBMIT_INFO,
            p_next: ptr::null(),
            wait_semaphore_count: 1,
            wait_semaphores: &image_available,
            wait_dst_stage_mask: &wait_stage,
            command_buffer_count: 1,
            command_buffers: &command_buffer,
            signal_semaphore_count: 1,
            signal_semaphores: &render_finished[index as usize],
        };
        check(
            unsafe { (api.queue_submit)(queue, 1, &submit, fence) },
            "vkQueueSubmit",
        );
        let present = PresentInfo {
            s_type: STRUCTURE_TYPE_PRESENT_INFO_KHR,
            p_next: ptr::null(),
            wait_semaphore_count: 1,
            wait_semaphores: &render_finished[index as usize],
            swapchain_count: 1,
            swapchains: &swapchain,
            image_indices: &index,
            results: ptr::null_mut(),
        };
        check(
            unsafe { (api.queue_present)(queue, &present) },
            "vkQueuePresentKHR",
        );
    }

    check(
        unsafe { (api.device_wait_idle)(device) },
        "vkDeviceWaitIdle",
    );
    unsafe {
        (api.destroy_fence)(device, fence, ptr::null());
        for &semaphore in &render_finished {
            (api.destroy_semaphore)(device, semaphore, ptr::null());
        }
        (api.destroy_semaphore)(device, image_available, ptr::null());
        (api.destroy_command_pool)(device, command_pool, ptr::null());
        (api.destroy_swapchain)(device, swapchain, ptr::null());
        (api.destroy_device)(device, ptr::null());
        (api.destroy_surface)(instance, surface, ptr::null());
        (api.destroy_instance)(instance, ptr::null());
    }
    println!("Destroyed the Vulkan objects");
    Ok(())
}
