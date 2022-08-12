#include "Audio/SoundRecorderStruct.h"
#include <cstddef>
#include <cstdint>
#include <vector>

extern "C" sfSoundRecorder *sfSoundRecorder_create(sfSoundRecorderStartCallback onStart,
                                                   sfSoundRecorderProcessCallback onProcess,
                                                   sfSoundRecorderStopCallback onStop,
                                                   void *userData) {
    return new sfSoundRecorder(onStart, onProcess, onStop, userData);
}

extern "C" void sfSoundRecorder_destroy(sfSoundRecorder *soundRecorder) {
    delete soundRecorder;
}

extern "C" bool sfSoundRecorder_start(sfSoundRecorder *soundRecorder, unsigned int sampleRate) {
    return soundRecorder->start(sampleRate);
}

extern "C" void sfSoundRecorder_stop(sfSoundRecorder *soundRecorder) {
    soundRecorder->stop();
}

extern "C" unsigned int sfSoundRecorder_getSampleRate(const sfSoundRecorder *soundRecorder) {
    return soundRecorder->getSampleRate();
}

extern "C" bool sfSoundRecorder_isAvailable(void) {
    return sf::SoundRecorder::isAvailable();
}

extern "C" void sfSoundRecorder_setProcessingInterval(sfSoundRecorder *soundRecorder, int64_t interval) {
    soundRecorder->setProcessingInterval(interval);
}

extern "C" std::vector<std::string> *sfSoundRecorder_getAvailableDevices() {
    return new std::vector<std::string>(sf::SoundRecorder::getAvailableDevices());
}

extern "C" std::string *sfSoundRecorder_getDefaultDevice() {
    return new std::string(sf::SoundRecorder::getDefaultDevice());
}

extern "C" bool sfSoundRecorder_setDevice(sfSoundRecorder *soundRecorder, const char *name) {
    return soundRecorder->setDevice(name);
}

extern "C" const std::string *sfSoundRecorder_getDevice(sfSoundRecorder *soundRecorder) {
    return &soundRecorder->getDevice();
}

extern "C" void sfSoundRecorder_setChannelCount(sfSoundRecorder *soundRecorder, unsigned int channelCount) {
    soundRecorder->setChannelCount(channelCount);
}

extern "C" unsigned int sfSoundRecorder_getChannelCount(const sfSoundRecorder *soundRecorder) {
    return soundRecorder->getChannelCount();
}
