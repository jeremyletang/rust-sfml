#include "SFML/Audio/SoundChannel.hpp"
#include "System/InputStreamHelper.hpp"
#include "Audio/SoundChannel.hpp"
#include <SFML/Audio/SoundBuffer.hpp>
#include <cstddef>

extern "C" sf::SoundBuffer *sfSoundBuffer_new() {
    return new sf::SoundBuffer();
}

extern "C" void sfSoundBuffer_del(sf::SoundBuffer *soundBuffer) {
    delete soundBuffer;
}

extern "C" sf::SoundBuffer *sfSoundBuffer_cpy(const sf::SoundBuffer *soundBuffer) {
    return new sf::SoundBuffer(*soundBuffer);
}

extern "C" bool sfSoundBuffer_loadFromFile(sf::SoundBuffer *buffer, const char *filename) {
    return buffer->loadFromFile(filename);
}

extern "C" bool sfSoundBuffer_loadFromMemory(sf::SoundBuffer *buffer, const uint8_t *data, size_t sizeInBytes) {
    return buffer->loadFromMemory(data, sizeInBytes);
}

extern "C" bool sfSoundBuffer_loadFromStream(sf::SoundBuffer *buffer, sfInputStreamHelper *stream) {
    return buffer->loadFromStream(*stream);
}

extern "C" bool sfSoundBuffer_loadFromSamples(sf::SoundBuffer *buffer, const int16_t *samples, uint64_t sampleCount, unsigned int channelCount, unsigned int sampleRate, const sfSoundChannel *channelMap, size_t channelMapLen) {
    std::vector<sf::SoundChannel> castedChannelMap(channelMapLen);
    for (size_t i = 0; i < channelMapLen; i++) {
        castedChannelMap.push_back(static_cast<sf::SoundChannel>(channelMap[i]));
    }
    return buffer->loadFromSamples(samples, sampleCount, channelCount, sampleRate, castedChannelMap);
}

extern "C" bool sfSoundBuffer_saveToFile(const sf::SoundBuffer *soundBuffer, const char *filename) {
    return soundBuffer->saveToFile(filename);
}

extern "C" const int16_t *sfSoundBuffer_getSamples(const sf::SoundBuffer *soundBuffer) {
    return soundBuffer->getSamples();
}

extern "C" uint64_t sfSoundBuffer_getSampleCount(const sf::SoundBuffer *soundBuffer) {
    return soundBuffer->getSampleCount();
}

extern "C" unsigned int sfSoundBuffer_getSampleRate(const sf::SoundBuffer *soundBuffer) {
    return soundBuffer->getSampleRate();
}

extern "C" unsigned int sfSoundBuffer_getChannelCount(const sf::SoundBuffer *soundBuffer) {
    return soundBuffer->getChannelCount();
}

extern "C" const std::vector<sf::SoundChannel> *sfSoundBuffer_getChannelMap(const sf::SoundBuffer *soundStream) {
    return new std::vector(soundStream->getChannelMap());
}

extern "C" int64_t sfSoundBuffer_getDuration(const sf::SoundBuffer *soundBuffer) {
    return soundBuffer->getDuration().asMicroseconds();
}
