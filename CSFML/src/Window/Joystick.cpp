#include <SFML/Window/Joystick.hpp>
#include <cstddef>

extern "C" bool sfJoystick_isConnected(unsigned int joystick) {
    return sf::Joystick::isConnected(joystick);
}

extern "C" unsigned int sfJoystick_getButtonCount(unsigned int joystick) {
    return sf::Joystick::getButtonCount(joystick);
}

extern "C" bool sfJoystick_hasAxis(unsigned int joystick, sf::Joystick::Axis axis) {
    return sf::Joystick::hasAxis(joystick, axis);
}

extern "C" bool sfJoystick_isButtonPressed(unsigned int joystick, unsigned int button) {
    return sf::Joystick::isButtonPressed(joystick, button);
}

extern "C" float sfJoystick_getAxisPosition(unsigned int joystick, sf::Joystick::Axis axis) {
    return sf::Joystick::getAxisPosition(joystick, axis);
}

extern "C" sf::Joystick::Identification *sfJoystick_getIdentification(unsigned int joystick) {
    sf::Joystick::Identification ident = sf::Joystick::getIdentification(joystick);
    sf::Joystick::Identification *copy = new sf::Joystick::Identification;
    copy->name = ident.name;
    copy->vendorId = ident.vendorId;
    copy->productId = ident.productId;
    return copy;
}

extern "C" void sfJoystickIdentification_destroy(sf::Joystick::Identification *ident) {
    delete ident;
}

extern "C" unsigned int sfJoystickIdentification_getVendorId(const sf::Joystick::Identification *ident) {
    return ident->vendorId;
}

extern "C" unsigned int sfJoystickIdentification_getProductId(const sf::Joystick::Identification *ident) {
    return ident->productId;
}

extern "C" const sf::String *sfJoystickIdentification_getName(const sf::Joystick::Identification *ident) {
    return &ident->name;
}

extern "C" void sfJoystick_update(void) {
    sf::Joystick::update();
}
