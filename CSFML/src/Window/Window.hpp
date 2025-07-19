#pragma once

#include <SFML/Window/WindowEnums.hpp>

enum sfState {
    Windowed,
    Fullscreen
};

sf::State to_state(const sfState state);
