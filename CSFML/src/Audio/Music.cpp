#include "System/InputStreamHelper.hpp"
#include "System/Vector3.h"
#include <SFML/Audio/Music.hpp>
#include <cstddef>

typedef struct
{
    int64_t offset; ///< The beginning offset of the time range
    int64_t length; ///< The length of the time range
} sfTimeSpan;

extern "C" sf::Music *sfMusic_new() {
    return new sf::Music;
}

extern "C" void sfMusic_del(sf::Music *music) {
    delete music;
}

extern "C" bool sfMusic_openFromFile(sf::Music *music, const char *filename) {
    return music->openFromFile(filename);
}

extern "C" bool sfMusic_openFromMemory(sf::Music *music, const uint8_t *data, size_t sizeInBytes) {
    return music->openFromMemory(data, sizeInBytes);
}

extern "C" bool sfMusic_openFromStream(sf::Music *music, sfInputStreamHelper *stream) {
    return music->openFromStream(*stream);
}

extern "C" void sfMusic_setLoop(sf::Music *music, bool loop) {
    music->setLoop(loop != 0);
}

extern "C" bool sfMusic_getLoop(const sf::Music *music) {
    return music->getLoop();
}

extern "C" int64_t sfMusic_getDuration(const sf::Music *music) {
    return music->getDuration().asMicroseconds();
}

extern "C" sfTimeSpan sfMusic_getLoopPoints(const sf::Music *music) {
    sf::Music::TimeSpan span = music->getLoopPoints();
    return {span.offset.asMicroseconds(), span.length.asMicroseconds()};
}

extern "C" void sfMusic_setLoopPoints(sf::Music *music, sfTimeSpan timePoints) {
    music->setLoopPoints(sf::Music::TimeSpan(sf::microseconds(timePoints.offset),
                                             sf::microseconds(timePoints.length)));
}

extern "C" void sfMusic_play(sf::Music *music) {
    music->play();
}

extern "C" void sfMusic_pause(sf::Music *music) {
    music->pause();
}

extern "C" void sfMusic_stop(sf::Music *music) {
    music->stop();
}

extern "C" unsigned int sfMusic_getChannelCount(const sf::Music *music) {
    return music->getChannelCount();
}

extern "C" unsigned int sfMusic_getSampleRate(const sf::Music *music) {
    return music->getSampleRate();
}

extern "C" sf::Music::Status sfMusic_getStatus(const sf::Music *music) {

    return music->getStatus();
}

extern "C" int64_t sfMusic_getPlayingOffset(const sf::Music *music) {
    return music->getPlayingOffset().asMicroseconds();
}

extern "C" void sfMusic_setPitch(sf::Music *music, float pitch) {
    music->setPitch(pitch);
}

extern "C" void sfMusic_setVolume(sf::Music *music, float volume) {
    music->setVolume(volume);
}

extern "C" void sfMusic_setPosition(sf::Music *music, sfVector3f position) {
    music->setPosition(sf::Vector3f(position.x, position.y, position.z));
}

extern "C" void sfMusic_setRelativeToListener(sf::Music *music, bool relative) {
    music->setRelativeToListener(relative);
}

extern "C" void sfMusic_setMinDistance(sf::Music *music, float distance) {
    music->setMinDistance(distance);
}

extern "C" void sfMusic_setAttenuation(sf::Music *music, float attenuation) {
    music->setAttenuation(attenuation);
}

extern "C" void sfMusic_setPlayingOffset(sf::Music *music, int64_t timeOffset) {
    music->setPlayingOffset(sf::microseconds(timeOffset));
}

extern "C" float sfMusic_getPitch(const sf::Music *music) {
    return music->getPitch();
}

extern "C" float sfMusic_getVolume(const sf::Music *music) {
    return music->getVolume();
}

extern "C" sfVector3f sfMusic_getPosition(const sf::Music *music) {
    sf::Vector3f pos = music->getPosition();
    return {pos.x, pos.y, pos.z};
}

extern "C" bool sfMusic_isRelativeToListener(const sf::Music *music) {
    return music->isRelativeToListener();
}

extern "C" float sfMusic_getMinDistance(const sf::Music *music) {
    return music->getMinDistance();
}

extern "C" float sfMusic_getAttenuation(const sf::Music *music) {
    return music->getAttenuation();
}
