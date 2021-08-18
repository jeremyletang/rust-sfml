
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

#include <SFML/Audio/Sound.h>
#include <SFML/Audio/SoundStruct.h>
#include <cstddef>



sfSound* sfSound_create(void)
{
    return new sfSound;
}



sfSound* sfSound_copy(const sfSound* sound)
{


    return new sfSound(*sound);
}



void sfSound_destroy(sfSound* sound)
{
    delete sound;
}



void sfSound_play(sfSound* sound)
{
    sound->This.play();
}



void sfSound_pause(sfSound* sound)
{
    sound->This.pause();
}



void sfSound_stop(sfSound* sound)
{
    sound->This.stop();
}



void sfSound_setBuffer(sfSound* sound, const sfSoundBuffer* buffer)
{
    if (buffer)
    {
        sound->This.setBuffer(buffer->This);
        sound->Buffer = buffer;
    }
}



const sfSoundBuffer* sfSound_getBuffer(const sfSound* sound)
{


    return sound->Buffer;
}



void sfSound_setLoop(sfSound* sound, sfBool loop)
{
    sound->This.setLoop(loop == sfTrue);
}



sfBool sfSound_getLoop(const sfSound* sound)
{
    return sound->This.getLoop();
}



sfSoundStatus sfSound_getStatus(const sfSound* sound)
{


    return static_cast<sfSoundStatus>(sound->This.getStatus());
}



void sfSound_setPitch(sfSound* sound, float pitch)
{
    sound->This.setPitch(pitch);
}



void sfSound_setVolume(sfSound* sound, float volume)
{
    sound->This.setVolume(volume);
}



void sfSound_setPosition(sfSound* sound, sfVector3f position)
{
    sound->This.setPosition(sf::Vector3f(position.x, position.y, position.z));
}



void sfSound_setRelativeToListener(sfSound* sound, sfBool relative)
{
    sound->This.setRelativeToListener(relative == sfTrue);
}



void sfSound_setMinDistance(sfSound* sound, float distance)
{
    sound->This.setMinDistance(distance);
}



void sfSound_setAttenuation(sfSound* sound, float attenuation)
{
    sound->This.setAttenuation(attenuation);
}



void sfSound_setPlayingOffset(sfSound* sound, sfTime timeOffset)
{
    sound->This.setPlayingOffset(sf::microseconds(timeOffset.microseconds));
}



float sfSound_getPitch(const sfSound* sound)
{
    return sound->This.getPitch();
}



float sfSound_getVolume(const sfSound* sound)
{
    return sound->This.getVolume();
}



sfVector3f sfSound_getPosition(const sfSound* sound)
{
    sfVector3f position = {0, 0, 0};


    sf::Vector3f sfmlPos = sound->This.getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;
    position.z = sfmlPos.z;

    return position;
}



sfBool sfSound_isRelativeToListener(const sfSound* sound)
{
    return sound->This.isRelativeToListener();
}



float sfSound_getMinDistance(const sfSound* sound)
{
    return sound->This.getMinDistance();
}



float sfSound_getAttenuation(const sfSound* sound)
{
    return sound->This.getAttenuation();
}



sfTime sfSound_getPlayingOffset(const sfSound* sound)
{
    sfTime time = {0};


    time.microseconds = sound->This.getPlayingOffset().asMicroseconds();
    return time;
}
