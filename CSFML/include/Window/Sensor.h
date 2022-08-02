
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

#ifndef SFML_SENSOR_H
#define SFML_SENSOR_H

// Headers

#include "System/Vector3.h"

typedef enum {
    sfSensorAccelerometer,    ///< Measures the raw acceleration (m/s^2)
    sfSensorGyroscope,        ///< Measures the raw rotation rates (degrees/s)
    sfSensorMagnetometer,     ///< Measures the ambient magnetic field (micro-teslas)
    sfSensorGravity,          ///< Measures the direction and intensity of gravity, independent of device acceleration (m/s^2)
    sfSensorUserAcceleration, ///< Measures the direction and intensity of device acceleration, independent of the gravity (m/s^2)
    sfSensorOrientation,      ///< Measures the absolute 3D orientation (degrees)

    sfSensorCount ///< Keep last -- the total number of sensor types
} sfSensorType;

extern "C" bool sfSensor_isAvailable(sfSensorType sensor);

extern "C" void sfSensor_setEnabled(sfSensorType sensor, bool enabled);

extern "C" sfVector3f sfSensor_getValue(sfSensorType sensor);

#endif // SFML_SENSOR_H
