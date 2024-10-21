#include <string>
#include <vector>
#include "SFML/Audio/SoundRecorder.hpp"

extern "C" bool sfSoundRecorder_isAvailable(void) {
    return sf::SoundRecorder::isAvailable();
}

extern "C" std::string *sfSoundRecorder_getDefaultDevice() {
    return new std::string(sf::SoundRecorder::getDefaultDevice());
}

extern "C" std::vector<std::string> *sfSoundRecorder_getAvailableDevices() {
    return new std::vector<std::string>(sf::SoundRecorder::getAvailableDevices());
}
