#include "System/InputStreamStruct.h"
#include <SFML/Audio/SoundBuffer.hpp>
#include <cstddef>

extern "C" sf::SoundBuffer *sfSoundBuffer_createFromFile(const char *filename) {
    sf::SoundBuffer *buffer = new sf::SoundBuffer;

    if (!buffer->loadFromFile(filename)) {
        delete buffer;
        buffer = NULL;
    }

    return buffer;
}

extern "C" sf::SoundBuffer *sfSoundBuffer_createFromMemory(const void *data, size_t sizeInBytes) {
    sf::SoundBuffer *buffer = new sf::SoundBuffer;

    if (!buffer->loadFromMemory(data, sizeInBytes)) {
        delete buffer;
        buffer = NULL;
    }

    return buffer;
}

extern "C" sf::SoundBuffer *sfSoundBuffer_createFromStream(sfInputStream *stream) {

    sf::SoundBuffer *buffer = new sf::SoundBuffer;
    if (!buffer->loadFromStream(*stream)) {
        delete buffer;
        buffer = NULL;
    }

    return buffer;
}

extern "C" sf::SoundBuffer *sfSoundBuffer_createFromSamples(const int16_t *samples, uint64_t sampleCount, unsigned int channelCount, unsigned int sampleRate) {
    sf::SoundBuffer *buffer = new sf::SoundBuffer;

    if (!buffer->loadFromSamples(samples, sampleCount, channelCount, sampleRate)) {
        delete buffer;
        buffer = NULL;
    }

    return buffer;
}

extern "C" sf::SoundBuffer *sfSoundBuffer_copy(const sf::SoundBuffer *soundBuffer) {
    return new sf::SoundBuffer(*soundBuffer);
}

extern "C" void sfSoundBuffer_destroy(sf::SoundBuffer *soundBuffer) {
    delete soundBuffer;
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

extern "C" int64_t sfSoundBuffer_getDuration(const sf::SoundBuffer *soundBuffer) {
    return soundBuffer->getDuration().asMicroseconds();
}
