#include "Window/VideoMode.hpp"
#include <SFML/Window/VideoMode.hpp>
#include <cstddef>
#include <vector>

extern "C" sfVideoMode sfVideoMode_getDesktopMode(void) {
    return convertVideoMode(sf::VideoMode::getDesktopMode());
}

extern "C" const std::vector<sf::VideoMode> *sfVideoMode_getFullscreenModes() {
    return &sf::VideoMode::getFullscreenModes();
}

extern "C" bool sfVideoMode_isValid(sfVideoMode mode) {
    return convertVideoMode(mode).isValid();
}

extern "C" std::size_t sfVideoModeVector_getLength(const std::vector<sf::VideoMode> *vec) {
    return vec->size();
}

extern "C" const sf::VideoMode *sfVideoModeVector_getData(const std::vector<sf::VideoMode> *vec) {
    return vec->data();
}
