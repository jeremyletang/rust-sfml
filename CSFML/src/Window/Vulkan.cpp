#include <SFML/Window/Vulkan.hpp>

#include <cstddef>

extern "C" bool sfVulkan_isAvailable(bool requireGraphics) {
    return sf::Vulkan::isAvailable(requireGraphics);
}

extern "C" void *sfVulkan_getFunction(const char *name) {
    return reinterpret_cast<void *>(sf::Vulkan::getFunction(name));
}

extern "C" const char *const *sfVulkan_getGraphicsRequiredInstanceExtensions(size_t *count) {
    const std::vector<const char *> &extensions = sf::Vulkan::getGraphicsRequiredInstanceExtensions();
    *count = extensions.size();
    return extensions.data();
}
