#include <SFML/System/String.hpp>

extern "C" std::size_t sfString_getLength(const sf::String *string) {
    return string->getSize();
}

extern "C" const sf::Uint32 *sfString_getData(const sf::String *string) {
    return string->getData();
}

extern "C" void sfString_delete(sf::String *string) {
    delete string;
}
