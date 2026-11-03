#include <SFML/System/Version.hpp>

extern "C" const sf::Version *sfVersion_get(void) {
    return &sf::version();
}

extern "C" int8_t sfVersion_getMajor(const sf::Version *version) {
    return version->major;
}

extern "C" int8_t sfVersion_getMinor(const sf::Version *version) {
    return version->minor;
}

extern "C" int8_t sfVersion_getPatch(const sf::Version *version) {
    return version->patch;
}

extern "C" bool sfVersion_getIsRelease(const sf::Version *version) {
    return version->isRelease;
}

extern "C" const char *sfVersion_getString(const sf::Version *version) {
    return version->string.data();
}

extern "C" size_t sfVersion_getStringLength(const sf::Version *version) {
    return version->string.size();
}
