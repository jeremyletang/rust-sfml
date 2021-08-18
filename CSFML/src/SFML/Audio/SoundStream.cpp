
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



// Headers

#include <SFML/Audio/SoundStream.h>
#include <SFML/Audio/SoundStreamStruct.h>
#include <cstddef>



sfSoundStream* sfSoundStream_create(sfSoundStreamGetDataCallback onGetData,
                                    sfSoundStreamSeekCallback    onSeek,
                                    unsigned int                 channelCount,
                                    unsigned int                 sampleRate,
                                    void*                        userData)
{
    return new sfSoundStream(onGetData, onSeek, channelCount, sampleRate, userData);
}



void sfSoundStream_destroy(sfSoundStream* soundStream)
{
    delete soundStream;
}



void sfSoundStream_play(sfSoundStream* soundStream)
{
    soundStream->This.play();
}



void sfSoundStream_pause(sfSoundStream* soundStream)
{
    soundStream->This.pause();
}



void sfSoundStream_stop(sfSoundStream* soundStream)
{
    soundStream->This.stop();
}



sfSoundStatus sfSoundStream_getStatus(const sfSoundStream* soundStream)
{


    return static_cast<sfSoundStatus>(soundStream->This.getStatus());
}



unsigned int sfSoundStream_getChannelCount(const sfSoundStream* soundStream)
{
    return soundStream->This.getChannelCount();
}



unsigned int sfSoundStream_getSampleRate(const sfSoundStream* soundStream)
{
    return soundStream->This.getSampleRate();
}



void sfSoundStream_setPitch(sfSoundStream* soundStream, float pitch)
{
    soundStream->This.setPitch(pitch);
}



void sfSoundStream_setVolume(sfSoundStream* soundStream, float volume)
{
    soundStream->This.setVolume(volume);
}



void sfSoundStream_setPosition(sfSoundStream* soundStream, sfVector3f position)
{
    soundStream->This.setPosition(position.x, position.y, position.z);
}



void sfSoundStream_setRelativeToListener(sfSoundStream* soundStream, sfBool relative)
{
    soundStream->This.setRelativeToListener(relative == sfTrue);
}



void sfSoundStream_setMinDistance(sfSoundStream* soundStream, float distance)
{
    soundStream->This.setMinDistance(distance);
}



void sfSoundStream_setAttenuation(sfSoundStream* soundStream, float attenuation)
{
    soundStream->This.setAttenuation(attenuation);
}



void sfSoundStream_setPlayingOffset(sfSoundStream* soundStream, sfTime timeOffset)
{
    soundStream->This.setPlayingOffset(sf::microseconds(timeOffset.microseconds));
}



void sfSoundStream_setLoop(sfSoundStream* soundStream, sfBool loop)
{
    soundStream->This.setLoop(loop == sfTrue);
}



float sfSoundStream_getPitch(const sfSoundStream* soundStream)
{
    return soundStream->This.getPitch();
}



float sfSoundStream_getVolume(const sfSoundStream* soundStream)
{
    return soundStream->This.getVolume();
}



sfVector3f sfSoundStream_getPosition(const sfSoundStream* soundStream)
{
    sfVector3f position = {0, 0, 0};


    sf::Vector3f sfmlPos = soundStream->This.getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;
    position.z = sfmlPos.z;

    return position;
}



sfBool sfSoundStream_isRelativeToListener(const sfSoundStream* soundStream)
{
    return soundStream->This.isRelativeToListener();
}



float sfSoundStream_getMinDistance(const sfSoundStream* soundStream)
{
    return soundStream->This.getMinDistance();
}



float sfSoundStream_getAttenuation(const sfSoundStream* soundStream)
{
    return soundStream->This.getAttenuation();
}



sfBool sfSoundStream_getLoop(const sfSoundStream* soundStream)
{
    return soundStream->This.getLoop();
}



sfTime sfSoundStream_getPlayingOffset(const sfSoundStream* soundStream)
{
    sfTime time = {0};


    time.microseconds = soundStream->This.getPlayingOffset().asMicroseconds();
    return time;
}
