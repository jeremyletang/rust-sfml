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

#ifndef SFML_TIME_H
#define SFML_TIME_H

////////////////////////////////////////////////////////////
// Headers
////////////////////////////////////////////////////////////
#include <SFML/System/Export.h>


////////////////////////////////////////////////////////////
/// \brief Represents a time value
///
////////////////////////////////////////////////////////////
typedef struct
{
    sfInt64 microseconds;
} sfTime;

////////////////////////////////////////////////////////////
/// \brief Predefined "zero" time value
///
////////////////////////////////////////////////////////////
CSFML_SYSTEM_API sfTime sfTime_Zero;

////////////////////////////////////////////////////////////
/// \brief Return a time value as a number of seconds
///
/// \param time Time value
///
/// \return Time in seconds
///
////////////////////////////////////////////////////////////
CSFML_SYSTEM_API float sfTime_asSeconds(sfTime time);

////////////////////////////////////////////////////////////
/// \brief Return a time value as a number of milliseconds
///
/// \param time Time value
///
/// \return Time in milliseconds
///
////////////////////////////////////////////////////////////
CSFML_SYSTEM_API sfInt32 sfTime_asMilliseconds(sfTime time);

////////////////////////////////////////////////////////////
/// \brief Return a time value as a number of microseconds
///
/// \param time Time value
///
/// \return Time in microseconds
///
////////////////////////////////////////////////////////////
CSFML_SYSTEM_API sfInt64 sfTime_asMicroseconds(sfTime time);

////////////////////////////////////////////////////////////
/// \brief Construct a time value from a number of seconds
///
/// \param amount Number of seconds
///
/// \return Time value constructed from the amount of seconds
///
////////////////////////////////////////////////////////////
CSFML_SYSTEM_API sfTime sfSeconds(float amount);

////////////////////////////////////////////////////////////
/// \brief Construct a time value from a number of milliseconds
///
/// \param amount Number of milliseconds
///
/// \return Time value constructed from the amount of milliseconds
///
////////////////////////////////////////////////////////////
CSFML_SYSTEM_API sfTime sfMilliseconds(sfInt32 amount);

////////////////////////////////////////////////////////////
/// \brief Construct a time value from a number of microseconds
///
/// \param amount Number of microseconds
///
/// \return Time value constructed from the amount of microseconds
///
////////////////////////////////////////////////////////////
CSFML_SYSTEM_API sfTime sfMicroseconds(sfInt64 amount);


#endif // SFML_TIME_H
