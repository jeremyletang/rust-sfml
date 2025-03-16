#include <SFML/Audio/SoundRecorder.hpp>
#include <cstddef>
#include <cstdint>

typedef bool (*sfCustomSoundRecorderStartCb)(void *);
typedef bool (*sfCustomSoundRecorderProcessCb)(const int16_t *, size_t, void *);
typedef void (*sfCustomSoundRecorderStopCb)(void *);

// Helper class implementing the callback forwarding from
// C++ to C in sfSoundRecorder
class sfCustomSoundRecorder final : public sf::SoundRecorder {
  public:
    sfCustomSoundRecorder(sfCustomSoundRecorderStartCb onStart,
                          sfCustomSoundRecorderProcessCb onProcess,
                          sfCustomSoundRecorderStopCb onStop,
                          void *userData) : myStartCb(onStart),
                                            myProcessCb(onProcess),
                                            myStopCb(onStop),
                                            myUserData(userData) {
    }

  private:
    virtual bool onStart() final {
        return myStartCb(myUserData);
    }

    virtual bool onProcessSamples(const std::int16_t *samples, std::size_t sampleCount) final {
        return myProcessCb(samples, sampleCount, myUserData);
    }

    virtual void onStop() final {
        myStopCb(myUserData);
    }

    sfCustomSoundRecorderStartCb myStartCb;
    sfCustomSoundRecorderProcessCb myProcessCb;
    sfCustomSoundRecorderStopCb myStopCb;
    void *myUserData;
};

extern "C" sfCustomSoundRecorder *sfCustomSoundRecorder_new(sfCustomSoundRecorderStartCb onStart,
                                                            sfCustomSoundRecorderProcessCb onProcess,
                                                            sfCustomSoundRecorderStopCb onStop,
                                                            void *userData) {
    return new sfCustomSoundRecorder(onStart, onProcess, onStop, userData);
}

extern "C" void sfCustomSoundRecorder_del(sfCustomSoundRecorder *soundRecorder) {
    delete soundRecorder;
}

extern "C" bool sfCustomSoundRecorder_start(sfCustomSoundRecorder *soundRecorder, unsigned int sampleRate) {
    return soundRecorder->start(sampleRate);
}

extern "C" void sfCustomSoundRecorder_stop(sfCustomSoundRecorder *soundRecorder) {
    soundRecorder->stop();
}

extern "C" unsigned int sfCustomSoundRecorder_getSampleRate(const sfCustomSoundRecorder *soundRecorder) {
    return soundRecorder->getSampleRate();
}

extern "C" bool sfCustomSoundRecorder_setDevice(sfCustomSoundRecorder *soundRecorder, const char *name) {
    return soundRecorder->setDevice(name);
}

extern "C" const std::string *sfCustomSoundRecorder_getDevice(sfCustomSoundRecorder *soundRecorder) {
    return &soundRecorder->getDevice();
}

extern "C" void sfCustomSoundRecorder_setChannelCount(sfCustomSoundRecorder *soundRecorder, unsigned int channelCount) {
    soundRecorder->setChannelCount(channelCount);
}

extern "C" unsigned int sfCustomSoundRecorder_getChannelCount(const sfCustomSoundRecorder *soundRecorder) {
    return soundRecorder->getChannelCount();
}
