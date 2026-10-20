use sfml::window::vulkan;

fn main() {
    println!("Vulkan available: {}", vulkan::is_available(true));
    println!("Graphics instance extensions:");
    for name in vulkan::graphics_required_instance_extensions() {
        println!("  {}", name.to_string_lossy());
    }
}
