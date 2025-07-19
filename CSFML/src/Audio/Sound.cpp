#include "System/Vector3.hpp"
#include "Audio/SoundSourceCone.hpp"
#include "Audio/EffectProcessor.hpp"
#include "Audio/SoundStatus.hpp"
#include <SFML/Audio/Sound.hpp>
#include <SFML/System/Time.hpp>
#include <chrono>
#include <cstdint>
#include <map>
#include <mutex>

extern "C" sf::Sound *sfSound_new(const sf::SoundBuffer *buffer) {
    return new sf::Sound(*buffer);
}

extern "C" sf::Sound *sfSound_cpy(const sf::Sound *sound) {
    return new sf::Sound(*sound);
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
    return &sound->getBuffer();
}

extern "C" void sfSound_setLooping(sf::Sound *sound, bool loop) {
    sound->setLooping(loop);
}

extern "C" bool sfSound_isLooping(const sf::Sound *sound) {
    return sound->isLooping();
}

extern "C" sfSoundStatus sfSound_getStatus(const sf::Sound *sound) {
    return static_cast<sfSoundStatus>(sound->getStatus());
}

extern "C" void sfSound_setPitch(sf::Sound *sound, float pitch) {
    sound->setPitch(pitch);
}

extern "C" void sfSound_setPan(sf::Sound *sound, float pan) {
    sound->setPan(pan);
}

extern "C" void sfSound_setVolume(sf::Sound *sound, float volume) {
    sound->setVolume(volume);
}

extern "C" void sfSound_setSpatializationEnabled(sf::Sound *sound, bool enabled) {
    sound->setSpatializationEnabled(enabled);
}

extern "C" void sfSound_setPosition(sf::Sound *sound, sfVector3f position) {
    sound->setPosition(sf::Vector3f(position.x, position.y, position.z));
}

extern "C" void sfSound_setDirection(sf::Sound *sound, sfVector3f direction) {
    sound->setDirection(convertVector3(direction));
}

extern "C" void sfSound_setCone(sf::Sound *sound, sfSoundSourceCone cone) {
    sound->setCone(convertCone(cone));
}

extern "C" void sfSound_setVelocity(sf::Sound *sound, sfVector3f velocity) {
    sound->setVelocity(convertVector3(velocity));
}

extern "C" void sfSound_setDopplerFactor(sf::Sound *sound, float factor) {
    sound->setDopplerFactor(factor);
}

extern "C" void sfSound_setDirectionalAttenuationFactor(sf::Sound *sound, float factor) {
    sound->setDirectionalAttenuationFactor(factor);
}

extern "C" void sfSound_setRelativeToListener(sf::Sound *sound, bool relative) {
    sound->setRelativeToListener(relative);
}

extern "C" void sfSound_setMinDistance(sf::Sound *sound, float distance) {
    sound->setMinDistance(distance);
}

extern "C" void sfSound_setMaxDistance(sf::Sound *sound, float distance) {
    assert(sound);
    sound->setMaxDistance(distance);
}

extern "C" void sfSound_setMinGain(sf::Sound *sound, float gain) {
    sound->setMinGain(gain);
}

extern "C" void sfSound_setMaxGain(sf::Sound *sound, float gain) {
    sound->setMaxGain(gain);
}

extern "C" void sfSound_setAttenuation(sf::Sound *sound, float attenuation) {
    sound->setAttenuation(attenuation);
}

extern "C" void sfSound_setPlayingOffset(sf::Sound *sound, int64_t timeOffset) {
    sound->setPlayingOffset(std::chrono::microseconds(timeOffset));
}

extern "C" float sfSound_getPitch(const sf::Sound *sound) {
    return sound->getPitch();
}

extern "C" float sfSound_getPan(const sf::Sound *sound) {
    return sound->getPan();
}

extern "C" float sfSound_getVolume(const sf::Sound *sound) {
    return sound->getVolume();
}

extern "C" bool sfSound_isSpatializationEnabled(const sf::Sound *sound) {
    return sound->isSpatializationEnabled();
}

extern "C" sfVector3f sfSound_getPosition(const sf::Sound *sound) {
    return convertVector3(sound->getPosition());
}

extern "C" sfVector3f sfSound_getDirection(const sf::Sound *sound) {
    return convertVector3(sound->getPosition());
}

extern "C" sfSoundSourceCone sfSound_getCone(const sf::Sound *sound) {
    return convertCone(sound->getCone());
}

extern "C" sfVector3f sfSound_getVelocity(const sf::Sound *sound) {
    return convertVector3(sound->getVelocity());
}

extern "C" float sfSound_getDopplerFactor(const sf::Sound *sound) {
    return sound->getDopplerFactor();
}

extern "C" float sfSound_getDirectionalAttenuationFactor(const sf::Sound *sound) {
    return sound->getDopplerFactor();
}

extern "C" float sfSound_getMaxDistance(const sf::Sound *sound) {
    return sound->getMaxDistance();
}

extern "C" float sfSound_getMinGain(const sf::Sound *sound) {
    return sound->getMinGain();
}

extern "C" float sfSound_getMaxGain(const sf::Sound *sound) {
    return sound->getMaxGain();
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

static std::map<sf::Sound *, std::pair<sfEffectProcessor, void *>> processors;
static std::mutex processorMutex;

extern "C" void sfSound_setEffectProcessor(sf::Sound *sound, sfEffectProcessor effectProcessor, void *userData) {
    std::unique_lock<std::mutex> lock(processorMutex);
    if (!effectProcessor) {
        processors.erase(sound);
        sound->setEffectProcessor(nullptr);
    } else {
        processors[sound] = {effectProcessor, userData};
        sound->setEffectProcessor(
            [sound](const float *inputFrames,
                    unsigned int &inputFrameCount,
                    float *outputFrames,
                    unsigned int &outputFrameCount,
                    unsigned int frameChannelCount) {
                std::unique_lock<std::mutex> lock(processorMutex);
                auto it = processors.find(sound);
                if (it != processors.end()) {
                    it->second.first(inputFrames, &inputFrameCount, outputFrames, &outputFrameCount, frameChannelCount, it->second.second);
                }
            });
    }
}

extern "C" void sfSound_del(sf::Sound *music) {
    sfSound_setEffectProcessor(music, nullptr, nullptr);
    delete music;
}
