
//
// SFML - Simple and Fast Multimedia Library
// Copyright (C) 2007-2018 Laurent Gomila (laurent@sfml-dev.org)
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it freely,
// subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented;
//    you must not claim that you wrote the original software.
//    If you use this software in a product, an acknowledgment
//    in the product documentation would be appreciated but is not required.
//
// 2. Altered source versions must be plainly marked as such,
//    and must not be misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

#ifndef SFML_SOUNDSTREAMSTRUCT_H
#define SFML_SOUNDSTREAMSTRUCT_H

// Headers

#include "Audio/SoundStream.h"
#include <SFML/Audio/SoundStream.hpp>

// Helper class implementing the callback forwarding from
// C++ to C in sfSoundStream

class sfSoundStreamImpl : public sf::SoundStream {
  public:
    sfSoundStreamImpl(sfSoundStreamGetDataCallback onGetData,
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
        bool ok = (myGetDataCallback(&chunk, myUserData) == sfTrue);

        data.samples = chunk.samples;
        data.sampleCount = chunk.sampleCount;

        return ok;
    }

    virtual void onSeek(sf::Time timeOffset) {
        if (mySeekCallback) {
            sfInt64 time = {timeOffset.asMicroseconds()};
            mySeekCallback(time, myUserData);
        }
    }

    sfSoundStreamGetDataCallback myGetDataCallback;
    sfSoundStreamSeekCallback mySeekCallback;
    void *myUserData;
};

// Internal structure of sfSoundStream

struct sfSoundStream {
    sfSoundStream(sfSoundStreamGetDataCallback onGetData,
                  sfSoundStreamSeekCallback onSeek,
                  unsigned int channelCount,
                  unsigned int sampleRate,
                  void *userData) : This(onGetData, onSeek, channelCount, sampleRate, userData) {
    }

    sfSoundStreamImpl This;
};

#endif // SFML_SOUNDSTREAMSTRUCT_H
