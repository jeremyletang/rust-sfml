#include "Audio/SoundChannel.hpp"
#include <SFML/Audio/SoundChannel.hpp>
#include <vector>

extern "C" std::size_t sfSoundChannelVector_getLength(const std::vector<sf::SoundChannel> *vec) {
    return vec->size();
}

extern "C" const sf::SoundChannel *sfSoundChannelVector_getData(const std::vector<sf::SoundChannel> *vec) {
    return vec->data();
}

extern "C" void sfSoundChannelVector_del(const std::vector<sf::SoundChannel> *vec) {
    delete vec;
}
