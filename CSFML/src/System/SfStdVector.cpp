#include <SFML/Window/VideoMode.hpp>
#include <string>
#include <vector>

extern "C" void sfStdStringVector_destroy(std::vector<std::string> *vec) {
    delete vec;
}

extern "C" std::size_t sfStdStringVector_getLength(const std::vector<std::string> *vec) {
    return vec->size();
}

extern "C" const std::string *sfStdStringVector_index(const std::vector<std::string> *vec, std::size_t index) {
    return &(*vec)[index];
}

extern "C" void sfVideoModeVector_destroy(std::vector<sf::VideoMode> *vec) {
    delete vec;
}

extern "C" std::size_t sfVideoModeVector_getLength(const std::vector<sf::VideoMode> *vec) {
    return vec->size();
}

extern "C" const sf::VideoMode *sfVideoModeVector_index(const std::vector<sf::VideoMode> *vec, std::size_t index) {
    return &(*vec)[index];
}
