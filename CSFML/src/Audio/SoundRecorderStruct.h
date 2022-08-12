#ifndef SFML_SOUNDRECORDERSTRUCT_H
#define SFML_SOUNDRECORDERSTRUCT_H

#include <SFML/Audio/SoundRecorder.hpp>
#include <cstdint>

typedef bool (*sfSoundRecorderStartCallback)(void *);                            ///< Type of the callback used when starting a capture
typedef bool (*sfSoundRecorderProcessCallback)(const int16_t *, size_t, void *); ///< Type of the callback used to process audio data
typedef void (*sfSoundRecorderStopCallback)(void *);                             ///< Type of the callback used when stopping a capture

// Helper class implementing the callback forwarding from
// C++ to C in sfSoundRecorder

class sfSoundRecorder : public sf::SoundRecorder {
  public:
    sfSoundRecorder(sfSoundRecorderStartCallback onStart,
                    sfSoundRecorderProcessCallback onProcess,
                    sfSoundRecorderStopCallback onStop,
                    void *userData) : myStartCallback(onStart),
                                      myProcessCallback(onProcess),
                                      myStopCallback(onStop),
                                      myUserData(userData) {
    }

    void setProcessingInterval(int64_t interval) {
        sf::SoundRecorder::setProcessingInterval(sf::microseconds(interval));
    }

  private:
    virtual bool onStart() {
        if (myStartCallback)
            return myStartCallback(myUserData);
        else
            return true;
    }

    virtual bool onProcessSamples(const sf::Int16 *samples, std::size_t sampleCount) {
        if (myProcessCallback)
            return myProcessCallback(samples, sampleCount, myUserData);
        else
            return true;
    }

    virtual void onStop() {
        if (myStopCallback)
            myStopCallback(myUserData);
    }

    sfSoundRecorderStartCallback myStartCallback;
    sfSoundRecorderProcessCallback myProcessCallback;
    sfSoundRecorderStopCallback myStopCallback;
    void *myUserData;
};

#endif // SFML_SOUNDRECORDERSTRUCT_H
