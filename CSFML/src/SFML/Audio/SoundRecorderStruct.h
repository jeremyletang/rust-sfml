
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

#ifndef SFML_SOUNDRECORDERSTRUCT_H
#define SFML_SOUNDRECORDERSTRUCT_H

// Headers

#include <SFML/Audio/SoundRecorder.h>
#include <SFML/Audio/SoundRecorder.hpp>

// Helper class implementing the callback forwarding from
// C++ to C in sfSoundRecorder

class sfSoundRecorderImpl : public sf::SoundRecorder {
  public:
    sfSoundRecorderImpl(sfSoundRecorderStartCallback onStart,
                        sfSoundRecorderProcessCallback onProcess,
                        sfSoundRecorderStopCallback onStop,
                        void *userData) : myStartCallback(onStart),
                                          myProcessCallback(onProcess),
                                          myStopCallback(onStop),
                                          myUserData(userData) {
    }

    void setProcessingInterval(sfTime interval) {
        sf::SoundRecorder::setProcessingInterval(sf::microseconds(interval.microseconds));
    }

  private:
    virtual bool onStart() {
        if (myStartCallback)
            return myStartCallback(myUserData) == sfTrue;
        else
            return true;
    }

    virtual bool onProcessSamples(const sf::Int16 *samples, std::size_t sampleCount) {
        if (myProcessCallback)
            return myProcessCallback(samples, sampleCount, myUserData) == sfTrue;
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

// Internal structure of sfSoundRecorder

struct sfSoundRecorder {
    sfSoundRecorder(sfSoundRecorderStartCallback onStart,
                    sfSoundRecorderProcessCallback onProcess,
                    sfSoundRecorderStopCallback onStop,
                    void *userData) : This(onStart, onProcess, onStop, userData) {
    }

    sfSoundRecorderImpl This;
    std::string DeviceName;
};

#endif // SFML_SOUNDRECORDERSTRUCT_H
