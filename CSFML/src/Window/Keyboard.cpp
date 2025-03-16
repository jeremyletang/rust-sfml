#include "Window/Keyboard.hpp"
#include "SFML/System/String.hpp"
#include <SFML/Window/Keyboard.hpp>

extern "C" bool sfKeyboard_isKeyPressed(sfKeyCode key) {
    return sf::Keyboard::isKeyPressed(static_cast<sf::Keyboard::Key>(key));
}

extern "C" bool sfKeyboard_isScancodePressed(sfScancode code) {
    return sf::Keyboard::isKeyPressed(static_cast<sf::Keyboard::Scancode>(code));
}

extern "C" sfKeyCode sfKeyboard_localize(sfScancode code) {
    return static_cast<sfKeyCode>(sf::Keyboard::localize(static_cast<sf::Keyboard::Scancode>(code)));
}

extern "C" sfScancode sfKeyboard_delocalize(sfKeyCode code) {
    return static_cast<sfScancode>(sf::Keyboard::localize(static_cast<sf::Keyboard::Scancode>(code)));
}

extern "C" sf::String *sfKeyboard_getDescription(sfScancode code) {
    return new sf::String(sf::Keyboard::getDescription(static_cast<sf::Keyboard::Scancode>(code)));
}

extern "C" void sfKeyboard_setVirtualKeyboardVisible(bool visible) {
    sf::Keyboard::setVirtualKeyboardVisible(visible);
}
