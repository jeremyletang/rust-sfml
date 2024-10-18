#include <SFML/Window/Clipboard.hpp>
#include <cstdint>

extern "C" sf::String *sfClipboard_getUnicodeString() {
    return new sf::String(sf::Clipboard::getString());
}

extern "C" void sfClipboard_setUnicodeString(const uint32_t *text) {
    sf::Clipboard::setString(text);
}
