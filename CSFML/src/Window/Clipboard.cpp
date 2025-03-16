#include <SFML/System/String.hpp>
#include <SFML/Window/Clipboard.hpp>

extern "C" sf::String *sfClipboard_getUnicodeString() {
    return new sf::String(sf::Clipboard::getString());
}

extern "C" void sfClipboard_setUnicodeString(const uint32_t *text) {
    sf::Clipboard::setString((char32_t *)text);
}
