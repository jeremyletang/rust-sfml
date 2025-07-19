#include "Audio/EffectProcessor.hpp"
#include "Audio/SoundSourceCone.hpp"
#include "Audio/SoundStatus.hpp"
#include "SFML/Audio/SoundChannel.hpp"
#include "System/InputStreamHelper.hpp"
#include "System/Vector3.hpp"
#include <SFML/Audio/Music.hpp>
#include <cstddef>
#include <map>
#include <mutex>

struct sfTimeSpan {
    int64_t offset;
    int64_t length;
};

extern "C" sf::Music *sfMusic_new() {
    return new sf::Music;
}

extern "C" bool sfMusic_openFromFile(sf::Music *music, const char *filename) {
    return music->openFromFile(std::filesystem::path(filename));
}

extern "C" bool sfMusic_openFromMemory(sf::Music *music, const uint8_t *data, size_t sizeInBytes) {
    return music->openFromMemory(data, sizeInBytes);
}

extern "C" bool sfMusic_openFromStream(sf::Music *music, sfInputStreamHelper *stream) {
    return music->openFromStream(*stream);
}

extern "C" void sfMusic_setLooping(sf::Music *music, bool loop) {
    music->setLooping(loop != 0);
}

extern "C" bool sfMusic_isLooping(const sf::Music *music) {
    return music->isLooping();
}

extern "C" int64_t sfMusic_getDuration(const sf::Music *music) {
    return music->getDuration().asMicroseconds();
}

extern "C" sfTimeSpan sfMusic_getLoopPoints(const sf::Music *music) {
    sf::Music::TimeSpan span = music->getLoopPoints();
    return {span.offset.asMicroseconds(), span.length.asMicroseconds()};
}

extern "C" void sfMusic_setLoopPoints(sf::Music *music, sfTimeSpan timePoints) {
    music->setLoopPoints({sf::microseconds(timePoints.offset),
                          sf::microseconds(timePoints.length)});
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

extern "C" const std::vector<sf::SoundChannel> *sfMusic_getChannelMap(const sf::Music *music) {
    return new std::vector(music->getChannelMap());
}

extern "C" sfSoundStatus sfMusic_getStatus(const sf::Music *music) {
    return static_cast<sfSoundStatus>(music->getStatus());
}

extern "C" int64_t sfMusic_getPlayingOffset(const sf::Music *music) {
    return music->getPlayingOffset().asMicroseconds();
}

extern "C" void sfMusic_setPitch(sf::Music *music, float pitch) {
    music->setPitch(pitch);
}

extern "C" void sfMusic_setPan(sf::Music *music, float pan) {
    music->setPan(pan);
}

extern "C" void sfMusic_setVolume(sf::Music *music, float volume) {
    music->setVolume(volume);
}

extern "C" void sfMusic_setSpatializationEnabled(sf::Music *music, bool enabled) {
    music->setSpatializationEnabled(enabled);
}

extern "C" void sfMusic_setDirection(sf::Music *music, sfVector3f direction) {
    music->setDirection({direction.x, direction.y, direction.z});
}

extern "C" void sfMusic_setCone(sf::Music *music, sfSoundSourceCone cone) {
    music->setCone(convertCone(cone));
}

extern "C" void sfMusic_setVelocity(sf::Music *music, sfVector3f velocity) {
    music->setVelocity(convertVector3(velocity));
}

extern "C" void sfMusic_setDopplerFactor(sf::Music *music, float factor) {
    music->setDopplerFactor(factor);
}

extern "C" void sfMusic_setDirectionalAttenuationFactor(sf::Music *music, float factor) {
    music->setDirectionalAttenuationFactor(factor);
}

extern "C" void sfMusic_setPosition(sf::Music *music, sf::Vector3f position) {
    music->setPosition(position);
}

extern "C" void sfMusic_setRelativeToListener(sf::Music *music, bool relative) {
    music->setRelativeToListener(relative);
}

extern "C" void sfMusic_setMinDistance(sf::Music *music, float distance) {
    music->setMinDistance(distance);
}

extern "C" void sfMusic_setMaxDistance(sf::Music *music, float distance) {
    music->setMaxDistance(distance);
}

extern "C" void sfMusic_setMinGain(sf::Music *music, float gain) {
    music->setMinGain(gain);
}

extern "C" void sfMusic_setMaxGain(sf::Music *music, float gain) {
    music->setMaxGain(gain);
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

extern "C" float sfMusic_getPan(const sf::Music *music) {
    return music->getPan();
}

extern "C" float sfMusic_getVolume(const sf::Music *music) {
    return music->getVolume();
}

extern "C" bool sfMusic_isSpatializationEnabled(const sf::Music *music) {
    return music->isSpatializationEnabled();
}

extern "C" sfVector3f sfMusic_getPosition(const sf::Music *music) {
    sf::Vector3f pos = music->getPosition();
    return {pos.x, pos.y, pos.z};
}

extern "C" sfVector3f sfMusic_getDirection(const sf::Music *music) {
    return convertVector3(music->getDirection());
}

extern "C" sfSoundSourceCone sfMusic_getCone(const sf::Music *music) {
    return convertCone(music->getCone());
}

extern "C" sfVector3f sfMusic_getVelocity(const sf::Music *music) {
    return convertVector3(music->getVelocity());
}

extern "C" float sfMusic_getDopplerFactor(const sf::Music *music) {
    return music->getDopplerFactor();
}

extern "C" float sfMusic_getDirectionalAttenuationFactor(const sf::Music *music) {
    return music->getDirectionalAttenuationFactor();
}

extern "C" bool sfMusic_isRelativeToListener(const sf::Music *music) {
    return music->isRelativeToListener();
}

extern "C" float sfMusic_getMinDistance(const sf::Music *music) {
    return music->getMinDistance();
}

extern "C" float sfMusic_getMaxDistance(const sf::Music *music) {
    return music->getMaxDistance();
}

extern "C" float sfMusic_getMinGain(const sf::Music *music) {
    return music->getMinGain();
}

extern "C" float sfMusic_getMaxGain(const sf::Music *music) {
    return music->getMinGain();
}

extern "C" float sfMusic_getAttenuation(const sf::Music *music) {
    return music->getAttenuation();
}

static std::map<sf::Music *, std::pair<sfEffectProcessor, void *>> processors;
static std::mutex processorMutex;

extern "C" void sfMusic_setEffectProcessor(sf::Music *music, sfEffectProcessor effectProcessor, void *userData) {
    std::unique_lock<std::mutex> lock(processorMutex);
    if (!effectProcessor) {
        processors.erase(music);
        music->setEffectProcessor(nullptr);
    } else {
        processors[music] = {effectProcessor, userData};
        music->setEffectProcessor(
            [music](const float *inputFrames,
                    unsigned int &inputFrameCount,
                    float *outputFrames,
                    unsigned int &outputFrameCount,
                    unsigned int frameChannelCount) {
                std::unique_lock<std::mutex> lock(processorMutex);
                auto it = processors.find(music);
                if (it != processors.end()) {
                    it->second.first(inputFrames, &inputFrameCount, outputFrames, &outputFrameCount, frameChannelCount, it->second.second);
                }
            });
    }
}

extern "C" void sfMusic_del(sf::Music *music) {
    sfMusic_setEffectProcessor(music, nullptr, nullptr);
    delete music;
}
