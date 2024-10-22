#include "Window/VideoMode.hpp"
#include <SFML/Window/VideoMode.hpp>
#include <cstddef>
#include <vector>

extern "C" sfVideoMode sfVideoMode_getDesktopMode(void) {
    sf::VideoMode vm = sf::VideoMode::getDesktopMode();
    return {vm.width, vm.height, vm.bitsPerPixel};
}

extern "C" const std::vector<sf::VideoMode> *sfVideoMode_getFullscreenModes() {
    return &sf::VideoMode::getFullscreenModes();
}

extern "C" bool sfVideoMode_isValid(sfVideoMode mode) {
    return sf::VideoMode(mode.width, mode.height, mode.bitsPerPixel).isValid();
}

extern "C" std::size_t sfVideoModeVector_getLength(const std::vector<sf::VideoMode> *vec) {
    return vec->size();
}

extern "C" const sf::VideoMode *sfVideoModeVector_getData(const std::vector<sf::VideoMode> *vec) {
    return vec->data();
}
