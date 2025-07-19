#include <SFML/System/String.hpp>

extern "C" std::size_t sfString_getLength(const sf::String *string) {
    return string->getSize();
}

extern "C" const uint32_t *sfString_getData(const sf::String *string) {
    return (uint32_t *)string->getData();
}

extern "C" void sfString_delete(sf::String *string) {
    delete string;
}
