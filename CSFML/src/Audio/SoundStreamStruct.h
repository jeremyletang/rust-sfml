#ifndef SFML_SOUNDSTREAMSTRUCT_H
#define SFML_SOUNDSTREAMSTRUCT_H

#include <SFML/Audio/SoundStream.hpp>
#include <cstdint>

typedef struct
{
    int16_t *samples;         ///< Pointer to the audio samples
    unsigned int sampleCount; ///< Number of samples pointed by Samples
} sfSoundStreamChunk;

typedef bool (*sfSoundStreamGetDataCallback)(sfSoundStreamChunk *, void *); ///< Type of the callback used to get a sound stream data
typedef void (*sfSoundStreamSeekCallback)(int64_t, void *);                 ///< Type of the callback used to seek in a sound stream

// Helper class implementing the callback forwarding from
// C++ to C in sfSoundStream

class sfSoundStream : public sf::SoundStream {
  public:
    sfSoundStream(sfSoundStreamGetDataCallback onGetData,
                  sfSoundStreamSeekCallback onSeek,
                  unsigned int channelCount,
                  unsigned int sampleRate,
                  void *userData) : myGetDataCallback(onGetData),
                                    mySeekCallback(onSeek),
                                    myUserData(userData) {
        initialize(channelCount, sampleRate);
    }

  private:
    virtual bool onGetData(Chunk &data) {
        sfSoundStreamChunk chunk = {NULL, 0};
        bool ok = (myGetDataCallback(&chunk, myUserData));

        data.samples = chunk.samples;
        data.sampleCount = chunk.sampleCount;

        return ok;
    }

    virtual void onSeek(sf::Time timeOffset) {
        if (mySeekCallback) {
            int64_t time = {timeOffset.asMicroseconds()};
            mySeekCallback(time, myUserData);
        }
    }

    sfSoundStreamGetDataCallback myGetDataCallback;
    sfSoundStreamSeekCallback mySeekCallback;
    void *myUserData;
};

#endif // SFML_SOUNDSTREAMSTRUCT_H
