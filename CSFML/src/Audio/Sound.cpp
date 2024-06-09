#include "System/Vector3.h"
#include <SFML/Audio.hpp>
#include <cstddef>
#include <cstdint>

extern "C" sf::Sound *sfSound_create(void) {
    return new sf::Sound;
}

extern "C" sf::Sound *sfSound_copy(const sf::Sound *sound) {
    return new sf::Sound(*sound);
}

extern "C" void sfSound_destroy(sf::Sound *sound) {
    delete sound;
}

extern "C" void sfSound_play(sf::Sound *sound) {
    sound->play();
}

extern "C" void sfSound_pause(sf::Sound *sound) {
    sound->pause();
}

extern "C" void sfSound_stop(sf::Sound *sound) {
    sound->stop();
}

extern "C" void sfSound_setBuffer(sf::Sound *sound, const sf::SoundBuffer *buffer) {
    sound->setBuffer(*buffer);
}

extern "C" const sf::SoundBuffer *sfSound_getBuffer(const sf::Sound *sound) {
    const sf::Sound *s = sound;
    return s->getBuffer();
}

extern "C" void sfSound_setLoop(sf::Sound *sound, bool loop) {
    sound->setLoop(loop);
}

extern "C" bool sfSound_getLoop(const sf::Sound *sound) {
    return sound->getLoop();
}

extern "C" sf::Sound::Status sfSound_getStatus(const sf::Sound *sound) {
    return sound->getStatus();
}

extern "C" void sfSound_setPitch(sf::Sound *sound, float pitch) {
    sound->setPitch(pitch);
}

extern "C" void sfSound_setVolume(sf::Sound *sound, float volume) {
    sound->setVolume(volume);
}

extern "C" void sfSound_setPosition(sf::Sound *sound, sfVector3f position) {
    sound->setPosition(sf::Vector3f(position.x, position.y, position.z));
}

extern "C" void sfSound_setRelativeToListener(sf::Sound *sound, bool relative) {
    sound->setRelativeToListener(relative);
}

extern "C" void sfSound_setMinDistance(sf::Sound *sound, float distance) {
    sound->setMinDistance(distance);
}

extern "C" void sfSound_setAttenuation(sf::Sound *sound, float attenuation) {
    sound->setAttenuation(attenuation);
}

extern "C" void sfSound_setPlayingOffset(sf::Sound *sound, int64_t timeOffset) {
    sound->setPlayingOffset(sf::microseconds(timeOffset));
}

extern "C" float sfSound_getPitch(const sf::Sound *sound) {
    return sound->getPitch();
}

extern "C" float sfSound_getVolume(const sf::Sound *sound) {
    return sound->getVolume();
}

extern "C" sfVector3f sfSound_getPosition(const sf::Sound *sound) {
    sf::Vector3f pos = sound->getPosition();
    return {pos.x, pos.y, pos.z};
}

extern "C" bool sfSound_isRelativeToListener(const sf::Sound *sound) {
    return sound->isRelativeToListener();
}

extern "C" float sfSound_getMinDistance(const sf::Sound *sound) {
    return sound->getMinDistance();
}

extern "C" float sfSound_getAttenuation(const sf::Sound *sound) {
    return sound->getAttenuation();
}

extern "C" int64_t sfSound_getPlayingOffset(const sf::Sound *sound) {
    int64_t time = sound->getPlayingOffset().asMicroseconds();
    return time;
}
