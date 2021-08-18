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
#include <SFML/Audio/Music.h>
#include <SFML/Audio/MusicStruct.h>
#include <SFML/Internal.h>


////////////////////////////////////////////////////////////
sfMusic* sfMusic_createFromFile(const char* filename)
{
    sfMusic* music = new sfMusic;
    if (!music->This.openFromFile(filename))
    {
        delete music;
        music = NULL;
    }

    return music;
}


////////////////////////////////////////////////////////////
sfMusic* sfMusic_createFromMemory(const void* data, size_t sizeInBytes)
{
    sfMusic* music = new sfMusic;
    if (!music->This.openFromMemory(data, sizeInBytes))
    {
        delete music;
        music = NULL;
    }

    return music;
}


////////////////////////////////////////////////////////////
sfMusic* sfMusic_createFromStream(sfInputStream* stream)
{
    CSFML_CHECK_RETURN(stream, NULL);

    sfMusic* music = new sfMusic;
    music->Stream = CallbackStream(stream);
    if (!music->This.openFromStream(music->Stream))
    {
        delete music;
        music = NULL;
    }

    return music;
}


////////////////////////////////////////////////////////////
void sfMusic_destroy(sfMusic* music)
{
    delete music;
}


////////////////////////////////////////////////////////////
void sfMusic_setLoop(sfMusic* music, sfBool loop)
{
    CSFML_CALL(music, setLoop(loop != 0));
}


////////////////////////////////////////////////////////////
sfBool sfMusic_getLoop(const sfMusic* music)
{
    CSFML_CALL_RETURN(music, getLoop(), sfFalse);
}


////////////////////////////////////////////////////////////
sfTime sfMusic_getDuration(const sfMusic* music)
{
    sfTime time = {0};
    CSFML_CHECK_RETURN(music, time);

    time.microseconds = music->This.getDuration().asMicroseconds();
    return time;
}


////////////////////////////////////////////////////////////
sfTimeSpan sfMusic_getLoopPoints(const sfMusic* music)
{
    sfTimeSpan timeSpan = {{0}, {0}};
    CSFML_CHECK_RETURN(music, timeSpan);

    sf::Music::TimeSpan span = music->This.getLoopPoints();

    timeSpan.offset.microseconds = span.offset.asMicroseconds();
    timeSpan.length.microseconds = span.length.asMicroseconds();

    return timeSpan;
}


////////////////////////////////////////////////////////////
void sfMusic_setLoopPoints(sfMusic* music, sfTimeSpan timePoints)
{
    CSFML_CALL(music, setLoopPoints(sf::Music::TimeSpan(sf::microseconds(timePoints.offset.microseconds),
                                                        sf::microseconds(timePoints.length.microseconds))));
}


////////////////////////////////////////////////////////////
void sfMusic_play(sfMusic* music)
{
    CSFML_CALL(music, play());
}


////////////////////////////////////////////////////////////
void sfMusic_pause(sfMusic* music)
{
    CSFML_CALL(music, pause());
}


////////////////////////////////////////////////////////////
void sfMusic_stop(sfMusic* music)
{
    CSFML_CALL(music, stop());
}


////////////////////////////////////////////////////////////
unsigned int sfMusic_getChannelCount(const sfMusic* music)
{
    CSFML_CALL_RETURN(music, getChannelCount(), 0);
}


////////////////////////////////////////////////////////////
unsigned int sfMusic_getSampleRate(const sfMusic* music)
{
    CSFML_CALL_RETURN(music, getSampleRate(), 0);
}


////////////////////////////////////////////////////////////
sfSoundStatus sfMusic_getStatus(const sfMusic* music)
{
    CSFML_CHECK_RETURN(music, sfStopped);

    return static_cast<sfSoundStatus>(music->This.getStatus());
}


////////////////////////////////////////////////////////////
sfTime sfMusic_getPlayingOffset(const sfMusic* music)
{
    sfTime time = {0};
    CSFML_CHECK_RETURN(music, time);

    time.microseconds = music->This.getPlayingOffset().asMicroseconds();
    return time;
}


////////////////////////////////////////////////////////////
void sfMusic_setPitch(sfMusic* music, float pitch)
{
    CSFML_CALL(music, setPitch(pitch));
}


////////////////////////////////////////////////////////////
void sfMusic_setVolume(sfMusic* music, float volume)
{
    CSFML_CALL(music, setVolume(volume));
}


////////////////////////////////////////////////////////////
void sfMusic_setPosition(sfMusic* music, sfVector3f position)
{
    CSFML_CALL(music, setPosition(sf::Vector3f(position.x, position.y, position.z)));
}


////////////////////////////////////////////////////////////
void sfMusic_setRelativeToListener(sfMusic* music, sfBool relative)
{
    CSFML_CALL(music, setRelativeToListener(relative == sfTrue));
}


////////////////////////////////////////////////////////////
void sfMusic_setMinDistance(sfMusic* music, float distance)
{
    CSFML_CALL(music, setMinDistance(distance));
}


////////////////////////////////////////////////////////////
void sfMusic_setAttenuation(sfMusic* music, float attenuation)
{
    CSFML_CALL(music, setAttenuation(attenuation));
}


////////////////////////////////////////////////////////////
void sfMusic_setPlayingOffset(sfMusic* music, sfTime timeOffset)
{
    CSFML_CALL(music, setPlayingOffset(sf::microseconds(timeOffset.microseconds)));
}


////////////////////////////////////////////////////////////
float sfMusic_getPitch(const sfMusic* music)
{
    CSFML_CALL_RETURN(music, getPitch(), 0.f);
}


////////////////////////////////////////////////////////////
float sfMusic_getVolume(const sfMusic* music)
{
    CSFML_CALL_RETURN(music, getVolume(), 0.f);
}


////////////////////////////////////////////////////////////
sfVector3f sfMusic_getPosition(const sfMusic* music)
{
    sfVector3f position = {0, 0, 0};
    CSFML_CHECK_RETURN(music, position);

    sf::Vector3f sfmlPos = music->This.getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;
    position.z = sfmlPos.z;

    return position;
}


////////////////////////////////////////////////////////////
sfBool sfMusic_isRelativeToListener(const sfMusic* music)
{
    CSFML_CALL_RETURN(music, isRelativeToListener(), sfFalse);
}


////////////////////////////////////////////////////////////
float sfMusic_getMinDistance(const sfMusic* music)
{
    CSFML_CALL_RETURN(music, getMinDistance(), 0.f);
}


////////////////////////////////////////////////////////////
float sfMusic_getAttenuation(const sfMusic* music)
{
    CSFML_CALL_RETURN(music, getAttenuation(), 0.f);
}
