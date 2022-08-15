#include <SFML/Audio/SoundBufferRecorder.hpp>
#include <cstddef>

extern "C" sf::SoundBufferRecorder *sfSoundBufferRecorder_create(void) {
    return new sf::SoundBufferRecorder;
}

extern "C" void sfSoundBufferRecorder_destroy(sf::SoundBufferRecorder *soundBufferRecorder) {
    delete soundBufferRecorder;
}

extern "C" bool sfSoundBufferRecorder_start(sf::SoundBufferRecorder *soundBufferRecorder, unsigned int sampleRate) {
    return soundBufferRecorder->start(sampleRate);
}

extern "C" void sfSoundBufferRecorder_stop(sf::SoundBufferRecorder *soundBufferRecorder) {
    soundBufferRecorder->stop();
}

extern "C" unsigned int sfSoundBufferRecorder_getSampleRate(const sf::SoundBufferRecorder *soundBufferRecorder) {
    return soundBufferRecorder->getSampleRate();
}

extern "C" const sf::SoundBuffer *sfSoundBufferRecorder_getBuffer(const sf::SoundBufferRecorder *soundBufferRecorder) {
    return &soundBufferRecorder->getBuffer();
}

extern "C" bool sfSoundBufferRecorder_setDevice(sf::SoundBufferRecorder *soundBufferRecorder, const char *name) {
    return soundBufferRecorder->setDevice(name);
}

extern "C" const std::string *sfSoundBufferRecorder_getDevice(sf::SoundBufferRecorder *soundBufferRecorder) {
    return &soundBufferRecorder->getDevice();
}
