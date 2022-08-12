#include <SFML/Window/Keyboard.hpp>

extern "C" bool sfKeyboard_isKeyPressed(sf::Keyboard::Key key) {
    return sf::Keyboard::isKeyPressed(key);
}

extern "C" void sfKeyboard_setVirtualKeyboardVisible(bool visible) {
    sf::Keyboard::setVirtualKeyboardVisible(visible);
}
