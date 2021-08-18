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
#include <SFML/Audio/SoundRecorder.h>
#include <SFML/Audio/SoundRecorderStruct.h>
#include <SFML/Internal.h>


////////////////////////////////////////////////////////////
sfSoundRecorder* sfSoundRecorder_create(sfSoundRecorderStartCallback   onStart,
                                        sfSoundRecorderProcessCallback onProcess,
                                        sfSoundRecorderStopCallback    onStop,
                                        void*                          userData)
{
    return new sfSoundRecorder(onStart, onProcess, onStop, userData);
}


////////////////////////////////////////////////////////////
void sfSoundRecorder_destroy(sfSoundRecorder* soundRecorder)
{
    delete soundRecorder;
}


////////////////////////////////////////////////////////////
sfBool sfSoundRecorder_start(sfSoundRecorder* soundRecorder, unsigned int sampleRate)
{
    CSFML_CALL_RETURN(soundRecorder, start(sampleRate), sfFalse);
}


////////////////////////////////////////////////////////////
void sfSoundRecorder_stop(sfSoundRecorder* soundRecorder)
{
    CSFML_CALL(soundRecorder, stop());
}


////////////////////////////////////////////////////////////
unsigned int sfSoundRecorder_getSampleRate(const sfSoundRecorder* soundRecorder)
{
    CSFML_CALL_RETURN(soundRecorder, getSampleRate(), 0);
}


////////////////////////////////////////////////////////////
sfBool sfSoundRecorder_isAvailable(void)
{
    return sf::SoundRecorder::isAvailable();
}


////////////////////////////////////////////////////////////
void sfSoundRecorder_setProcessingInterval(sfSoundRecorder* soundRecorder, sfTime interval)
{
    CSFML_CALL(soundRecorder, setProcessingInterval(interval));
}


////////////////////////////////////////////////////////////
const char** sfSoundRecorder_getAvailableDevices(size_t* count)
{
    static std::vector<std::string> stringDevices = sf::SoundRecorder::getAvailableDevices();
    static std::vector<const char*> cstringDevices;

    if (cstringDevices.empty() && !stringDevices.empty())
    {
        for (std::vector<std::string>::const_iterator it = stringDevices.begin(); it != stringDevices.end(); ++it)
        {
            cstringDevices.push_back(it->c_str());
        }
    }

    if (count)
        *count = cstringDevices.size();

    return !cstringDevices.empty() ? &cstringDevices[0] : NULL;
}


////////////////////////////////////////////////////////////
const char* sfSoundRecorder_getDefaultDevice()
{
    static std::string defaultDevice = sf::SoundRecorder::getDefaultDevice();

    return !defaultDevice.empty() ? defaultDevice.c_str() : NULL;
}


////////////////////////////////////////////////////////////
sfBool sfSoundRecorder_setDevice(sfSoundRecorder* soundRecorder, const char* name)
{
    CSFML_CALL_RETURN(soundRecorder, setDevice(name), sfFalse);
}


////////////////////////////////////////////////////////////
const char* sfSoundRecorder_getDevice(sfSoundRecorder* soundRecorder)
{
    CSFML_CHECK_RETURN(soundRecorder, NULL);

    soundRecorder->DeviceName = soundRecorder->This.getDevice();

    return soundRecorder->DeviceName.c_str();
}


////////////////////////////////////////////////////////////
void sfSoundRecorder_setChannelCount(sfSoundRecorder* soundRecorder, unsigned int channelCount)
{
    CSFML_CALL(soundRecorder, setChannelCount(channelCount));
}


////////////////////////////////////////////////////////////
unsigned int sfSoundRecorder_getChannelCount(const sfSoundRecorder* soundRecorder)
{
    CSFML_CALL_RETURN(soundRecorder, getChannelCount(), 0);
}
