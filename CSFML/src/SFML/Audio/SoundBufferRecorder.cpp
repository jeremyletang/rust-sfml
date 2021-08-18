////////////////////////////////////////////////////////////
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
////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////
// Headers
////////////////////////////////////////////////////////////
#include <SFML/Audio/SoundBufferRecorder.h>
#include <SFML/Audio/SoundBufferRecorderStruct.h>
#include <SFML/Internal.h>


////////////////////////////////////////////////////////////
sfSoundBufferRecorder* sfSoundBufferRecorder_create(void)
{
    return new sfSoundBufferRecorder;
}


////////////////////////////////////////////////////////////
void sfSoundBufferRecorder_destroy(sfSoundBufferRecorder* soundBufferRecorder)
{
    delete soundBufferRecorder;
}


////////////////////////////////////////////////////////////
sfBool sfSoundBufferRecorder_start(sfSoundBufferRecorder* soundBufferRecorder, unsigned int sampleRate)
{
    CSFML_CALL_RETURN(soundBufferRecorder, start(sampleRate), sfFalse);
}


////////////////////////////////////////////////////////////
void sfSoundBufferRecorder_stop(sfSoundBufferRecorder* soundBufferRecorder)
{
    CSFML_CALL(soundBufferRecorder, stop());
}


////////////////////////////////////////////////////////////
unsigned int sfSoundBufferRecorder_getSampleRate(const sfSoundBufferRecorder* soundBufferRecorder)
{
    CSFML_CALL_RETURN(soundBufferRecorder, getSampleRate(), 0);
}


////////////////////////////////////////////////////////////
const sfSoundBuffer* sfSoundBufferRecorder_getBuffer(const sfSoundBufferRecorder* soundBufferRecorder)
{
    CSFML_CHECK_RETURN(soundBufferRecorder, NULL);

    soundBufferRecorder->SoundBuffer.This = soundBufferRecorder->This.getBuffer();

    return &soundBufferRecorder->SoundBuffer;
}

////////////////////////////////////////////////////////////
sfBool sfSoundBufferRecorder_setDevice(sfSoundBufferRecorder* soundBufferRecorder, const char* name)
{
    CSFML_CALL_RETURN(soundBufferRecorder, setDevice(name), sfFalse);
}

////////////////////////////////////////////////////////////
const char* sfSoundBufferRecorder_getDevice(sfSoundBufferRecorder* soundBufferRecorder)
{
    CSFML_CHECK_RETURN(soundBufferRecorder, NULL);

    soundBufferRecorder->DeviceName = soundBufferRecorder->This.getDevice();

    return soundBufferRecorder->DeviceName.c_str();
}
