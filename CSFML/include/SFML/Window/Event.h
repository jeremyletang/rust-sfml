
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

#ifndef SFML_EVENT_H
#define SFML_EVENT_H

// Headers

#include <SFML/Window/Joystick.h>
#include <SFML/Window/Keyboard.h>
#include <SFML/Window/Mouse.h>
#include <SFML/Window/Sensor.h>

const int sfEvtClosed = 0;
const int sfEvtResized = 1;
const int sfEvtLostFocus = 2;
const int sfEvtGainedFocus = 3;
const int sfEvtTextEntered = 4;
const int sfEvtKeyPressed = 5;
const int sfEvtKeyReleased = 6;
const int sfEvtMouseWheelScrolled = 8;
const int sfEvtMouseButtonPressed = 9;
const int sfEvtMouseButtonReleased = 10;
const int sfEvtMouseMoved = 11;
const int sfEvtMouseEntered = 12;
const int sfEvtMouseLeft = 13;
const int sfEvtJoystickButtonPressed = 14;
const int sfEvtJoystickButtonReleased = 15;
const int sfEvtJoystickMoved = 16;
const int sfEvtJoystickConnected = 17;
const int sfEvtJoystickDisconnected = 18;
const int sfEvtTouchBegan = 19;
const int sfEvtTouchMoved = 20;
const int sfEvtTouchEnded = 21;
const int sfEvtSensorChanged = 22;
const int sfEvtCount = 23;

typedef struct
{
    int type;
    sfKeyCode code;
    sfBool alt;
    sfBool control;
    sfBool shift;
    sfBool system;
} sfKeyEvent;

typedef struct
{
    int type;
    sfUint32 unicode;
} sfTextEvent;

typedef struct
{
    int type;
    int x;
    int y;
} sfMouseMoveEvent;

typedef struct
{
    int type;
    sfMouseButton button;
    int x;
    int y;
} sfMouseButtonEvent;

typedef struct
{
    int type;
    sfMouseWheel wheel;
    float delta;
    int x;
    int y;
} sfMouseWheelScrollEvent;

typedef struct
{
    int type;
    unsigned int joystickId;
    sfJoystickAxis axis;
    float position;
} sfJoystickMoveEvent;

typedef struct
{
    int type;
    unsigned int joystickId;
    unsigned int button;
} sfJoystickButtonEvent;

typedef struct
{
    int type;
    unsigned int joystickId;
} sfJoystickConnectEvent;

typedef struct
{
    int type;
    unsigned int width;
    unsigned int height;
} sfSizeEvent;

typedef struct
{
    int type;
    unsigned int finger;
    int x;
    int y;
} sfTouchEvent;

typedef struct
{
    int type;
    sfSensorType sensorType;
    float x;
    float y;
    float z;
} sfSensorEvent;

typedef union {
    int type;                         ///< Type of the event
    sfSizeEvent size;                         ///< Size event parameters
    sfKeyEvent key;                           ///< Key event parameters
    sfTextEvent text;                         ///< Text event parameters
    sfMouseMoveEvent mouseMove;               ///< Mouse move event parameters
    sfMouseButtonEvent mouseButton;           ///< Mouse button event parameters
    sfMouseWheelScrollEvent mouseWheelScroll; ///< Mouse wheel event parameters
    sfJoystickMoveEvent joystickMove;         ///< Joystick move event parameters
    sfJoystickButtonEvent joystickButton;     ///< Joystick button event parameters
    sfJoystickConnectEvent joystickConnect;   ///< Joystick (dis)connect event parameters
    sfTouchEvent touch;                       ///< Touch events parameters
    sfSensorEvent sensor;                     ///< Sensor event parameters
} sfEvent;

#endif // SFML_EVENT_H
