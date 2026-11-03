//! Vulkan support: checks that the system can use Vulkan, looks up Vulkan functions, and lists the
//! instance extensions that graphics rendering requires.

use crate::ffi::window as ffi;
use std::ffi::{CStr, c_void};

pub use crate::ffi::window::{VkAllocationCallbacks, VkInstance, VkInstanceT, VkSurfaceKHR};

/// Address of a Vulkan function, as returned by [`get_function`].
pub type VulkanFunction = unsafe extern "C" fn();

/// Tells whether the system supports Vulkan.
///
/// Call this before using Vulkan. If it returns `false`, any attempt to use Vulkan will fail.
/// Pass `false` for `require_graphics` if only compute is needed, which skips checking for the
/// extensions that graphics rendering requires.
#[must_use]
pub fn is_available(require_graphics: bool) -> bool {
    unsafe { ffi::sfVulkan_isAvailable(require_graphics) }
}

/// Gets the address of a Vulkan function by name.
///
/// This only finds functions that the Vulkan library exports. Other functions, such as most
/// extension functions, must be loaded with `vkGetInstanceProcAddr`, which this function can
/// find.
///
/// Returns `None` if the function could not be found, for example because Vulkan is not
/// available.
///
/// # Usage example
/// ```
/// # use sfml::window::vulkan;
/// if vulkan::is_available(true) {
///     let function = vulkan::get_function(c"vkGetInstanceProcAddr");
///     assert!(function.is_some());
/// }
/// ```
#[must_use]
pub fn get_function(name: &CStr) -> Option<VulkanFunction> {
    let address = unsafe { ffi::sfVulkan_getFunction(name.as_ptr()) };
    // SAFETY: SFML returns either null or the address of a Vulkan function. A function pointer and
    // a data pointer have the same size on every platform SFML supports, and `Option` of a function
    // pointer uses null for `None`, so this conversion keeps the null check.
    unsafe { std::mem::transmute::<*mut c_void, Option<VulkanFunction>>(address) }
}

/// Gets the Vulkan instance extensions that graphics rendering requires.
///
/// Enable these when creating the Vulkan instance, before creating a surface with
/// [`Window::create_vulkan_surface`](crate::window::Window::create_vulkan_surface).
#[must_use]
pub fn graphics_required_instance_extensions() -> Vec<&'static CStr> {
    let mut count = 0;
    let names = unsafe { ffi::sfVulkan_getGraphicsRequiredInstanceExtensions(&mut count) };
    if count == 0 {
        return Vec::new();
    }
    // SAFETY: when it is not empty, SFML returns a pointer to a static array of `count` names.
    unsafe { std::slice::from_raw_parts(names, count) }
        .iter()
        // SAFETY: each name is a static, NUL-terminated string.
        .map(|&name| unsafe { CStr::from_ptr(name) })
        .collect()
}
