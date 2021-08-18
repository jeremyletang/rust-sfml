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
#include <SFML/Audio/Sound.h>
#include <SFML/Audio/SoundStruct.h>
#include <SFML/Internal.h>


////////////////////////////////////////////////////////////
sfSound* sfSound_create(void)
{
    return new sfSound;
}


////////////////////////////////////////////////////////////
sfSound* sfSound_copy(const sfSound* sound)
{
    CSFML_CHECK_RETURN(sound, NULL);

    return new sfSound(*sound);
}


////////////////////////////////////////////////////////////
void sfSound_destroy(sfSound* sound)
{
    delete sound;
}


////////////////////////////////////////////////////////////
void sfSound_play(sfSound* sound)
{
    CSFML_CALL(sound, play());
}


////////////////////////////////////////////////////////////
void sfSound_pause(sfSound* sound)
{
    CSFML_CALL(sound, pause());
}


////////////////////////////////////////////////////////////
void sfSound_stop(sfSound* sound)
{
    CSFML_CALL(sound, stop());
}


////////////////////////////////////////////////////////////
void sfSound_setBuffer(sfSound* sound, const sfSoundBuffer* buffer)
{
    if (buffer)
    {
        CSFML_CALL(sound, setBuffer(buffer->This));
        sound->Buffer = buffer;
    }
}


////////////////////////////////////////////////////////////
const sfSoundBuffer* sfSound_getBuffer(const sfSound* sound)
{
    CSFML_CHECK_RETURN(sound, NULL);

    return sound->Buffer;
}


////////////////////////////////////////////////////////////
void sfSound_setLoop(sfSound* sound, sfBool loop)
{
    CSFML_CALL(sound, setLoop(loop == sfTrue));
}


////////////////////////////////////////////////////////////
sfBool sfSound_getLoop(const sfSound* sound)
{
    CSFML_CALL_RETURN(sound, getLoop(), sfFalse);
}


////////////////////////////////////////////////////////////
sfSoundStatus sfSound_getStatus(const sfSound* sound)
{
    CSFML_CHECK_RETURN(sound, sfStopped);

    return static_cast<sfSoundStatus>(sound->This.getStatus());
}


////////////////////////////////////////////////////////////
void sfSound_setPitch(sfSound* sound, float pitch)
{
    CSFML_CALL(sound, setPitch(pitch));
}


////////////////////////////////////////////////////////////
void sfSound_setVolume(sfSound* sound, float volume)
{
    CSFML_CALL(sound, setVolume(volume));
}


////////////////////////////////////////////////////////////
void sfSound_setPosition(sfSound* sound, sfVector3f position)
{
    CSFML_CALL(sound, setPosition(sf::Vector3f(position.x, position.y, position.z)));
}


////////////////////////////////////////////////////////////
void sfSound_setRelativeToListener(sfSound* sound, sfBool relative)
{
    CSFML_CALL(sound, setRelativeToListener(relative == sfTrue));
}


////////////////////////////////////////////////////////////
void sfSound_setMinDistance(sfSound* sound, float distance)
{
    CSFML_CALL(sound, setMinDistance(distance));
}


////////////////////////////////////////////////////////////
void sfSound_setAttenuation(sfSound* sound, float attenuation)
{
    CSFML_CALL(sound, setAttenuation(attenuation));
}


////////////////////////////////////////////////////////////
void sfSound_setPlayingOffset(sfSound* sound, sfTime timeOffset)
{
    CSFML_CALL(sound, setPlayingOffset(sf::microseconds(timeOffset.microseconds)));
}


////////////////////////////////////////////////////////////
float sfSound_getPitch(const sfSound* sound)
{
    CSFML_CALL_RETURN(sound, getPitch(), 0.f);
}


////////////////////////////////////////////////////////////
float sfSound_getVolume(const sfSound* sound)
{
    CSFML_CALL_RETURN(sound, getVolume(), 0.f);
}


////////////////////////////////////////////////////////////
sfVector3f sfSound_getPosition(const sfSound* sound)
{
    sfVector3f position = {0, 0, 0};
    CSFML_CHECK_RETURN(sound, position);

    sf::Vector3f sfmlPos = sound->This.getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;
    position.z = sfmlPos.z;

    return position;
}


////////////////////////////////////////////////////////////
sfBool sfSound_isRelativeToListener(const sfSound* sound)
{
    CSFML_CALL_RETURN(sound, isRelativeToListener(), sfFalse);
}


////////////////////////////////////////////////////////////
float sfSound_getMinDistance(const sfSound* sound)
{
    CSFML_CALL_RETURN(sound, getMinDistance(), 0.f);
}


////////////////////////////////////////////////////////////
float sfSound_getAttenuation(const sfSound* sound)
{
    CSFML_CALL_RETURN(sound, getAttenuation(), 0.f);
}


////////////////////////////////////////////////////////////
sfTime sfSound_getPlayingOffset(const sfSound* sound)
{
    sfTime time = {0};
    CSFML_CHECK_RETURN(sound, time);

    time.microseconds = sound->This.getPlayingOffset().asMicroseconds();
    return time;
}
