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

#ifndef SFML_SENSOR_H
#define SFML_SENSOR_H

////////////////////////////////////////////////////////////
// Headers
////////////////////////////////////////////////////////////
#include <SFML/Window/Export.h>
#include <SFML/Window/Types.h>
#include <SFML/System/Vector3.h>


////////////////////////////////////////////////////////////
/// \brief Sensor Types
///
////////////////////////////////////////////////////////////
typedef enum
{
    sfSensorAccelerometer,    ///< Measures the raw acceleration (m/s^2)
    sfSensorGyroscope,        ///< Measures the raw rotation rates (degrees/s)
    sfSensorMagnetometer,     ///< Measures the ambient magnetic field (micro-teslas)
    sfSensorGravity,          ///< Measures the direction and intensity of gravity, independent of device acceleration (m/s^2)
    sfSensorUserAcceleration, ///< Measures the direction and intensity of device acceleration, independent of the gravity (m/s^2)
    sfSensorOrientation,      ///< Measures the absolute 3D orientation (degrees)

    sfSensorCount             ///< Keep last -- the total number of sensor types
} sfSensorType;


////////////////////////////////////////////////////////////
/// \brief Check if a sensor is available on the underlying platform
///
/// \param sensor Sensor to check
///
/// \return sfTrue if the sensor is available, sfFalse otherwise
///
////////////////////////////////////////////////////////////
CSFML_WINDOW_API sfBool sfSensor_isAvailable(sfSensorType sensor);

////////////////////////////////////////////////////////////
/// \brief Enable or disable a sensor
///
/// All sensors are disabled by default, to avoid consuming too
/// much battery power. Once a sensor is enabled, it starts
/// sending events of the corresponding type.
///
/// This function does nothing if the sensor is unavailable.
///
/// \param sensor Sensor to enable
/// \param enabled sfTrue to enable, sfFalse to disable
///
////////////////////////////////////////////////////////////
CSFML_WINDOW_API void sfSensor_setEnabled(sfSensorType sensor, sfBool enabled);

////////////////////////////////////////////////////////////
/// \brief Get the current sensor value
///
/// \param sensor Sensor to read
///
/// \return The current sensor value
///
////////////////////////////////////////////////////////////
CSFML_WINDOW_API sfVector3f sfSensor_getValue(sfSensorType sensor);


#endif // SFML_SENSOR_H
