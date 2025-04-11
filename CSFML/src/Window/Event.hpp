#pragma once

#include "System/Vector2.hpp"
#include "System/Vector3.hpp"
#include "Window/Joystick.hpp"
#include "Window/Keyboard.hpp"
#include "Window/Mouse.hpp"
#include "Window/Sensor.hpp"
#include <SFML/Window/Event.hpp>
#include <cstdint>
#include <optional>

////////////////////////////////////////////////////////////
/// \brief Definition of all the event types
///
////////////////////////////////////////////////////////////
typedef enum {
    sfEvtClosed,                 ///< The window requested to be closed (no data)
    sfEvtResized,                ///< The window was resized (data in event.size)
    sfEvtFocusLost,              ///< The window lost the focus (no data)
    sfEvtFocusGained,            ///< The window gained the focus (no data)
    sfEvtTextEntered,            ///< A character was entered (data in event.text)
    sfEvtKeyPressed,             ///< A key was pressed (data in event.key)
    sfEvtKeyReleased,            ///< A key was released (data in event.key)
    sfEvtMouseWheelScrolled,     ///< The mouse wheel was scrolled (data in event.mouseWheelScroll)
    sfEvtMouseButtonPressed,     ///< A mouse button was pressed (data in event.mouseButton)
    sfEvtMouseButtonReleased,    ///< A mouse button was released (data in event.mouseButton)
    sfEvtMouseMoved,             ///< The mouse cursor moved (data in event.mouseMove)
    sfEvtMouseMovedRaw,          ///< The mouse cursor moved (data in event.mouseMove)
    sfEvtMouseEntered,           ///< The mouse cursor entered the area of the window (no data)
    sfEvtMouseLeft,              ///< The mouse cursor left the area of the window (no data)
    sfEvtJoystickButtonPressed,  ///< A joystick button was pressed (data in event.joystickButton)
    sfEvtJoystickButtonReleased, ///< A joystick button was released (data in event.joystickButton)
    sfEvtJoystickMoved,          ///< The joystick moved along an axis (data in event.joystickMove)
    sfEvtJoystickConnected,      ///< A joystick was connected (data in event.joystickConnect)
    sfEvtJoystickDisconnected,   ///< A joystick was disconnected (data in event.joystickConnect)
    sfEvtTouchBegan,             ///< A touch event began (data in event.touch)
    sfEvtTouchMoved,             ///< A touch moved (data in event.touch)
    sfEvtTouchEnded,             ///< A touch event ended (data in event.touch)
    sfEvtSensorChanged,          ///< A sensor value changed (data in event.sensor)

    sfEvtCount ///< Keep last -- the total number of event types
} sfEventType;

////////////////////////////////////////////////////////////
/// \brief Keyboard event parameters
///
////////////////////////////////////////////////////////////
typedef struct
{
    sfEventType type;
    sfKeyCode code;
    sfScancode scancode;
    bool alt;
    bool control;
    bool shift;
    bool system;
} sfKeyEvent;

////////////////////////////////////////////////////////////
/// \brief Text event parameters
///
////////////////////////////////////////////////////////////
typedef struct
{
    sfEventType type;
    uint32_t unicode;
} sfTextEvent;

////////////////////////////////////////////////////////////
/// \brief Mouse move event parameters
///
////////////////////////////////////////////////////////////
typedef struct
{
    sfEventType type;
    sfVector2i position;
} sfMouseMoveEvent;

////////////////////////////////////////////////////////////
/// \brief Mouse move raw event parameters
///
////////////////////////////////////////////////////////////
typedef struct
{
    sfEventType type;
    sfVector2i delta;
} sfMouseMoveRawEvent;

////////////////////////////////////////////////////////////
/// \brief Mouse buttons events parameters
///
////////////////////////////////////////////////////////////
typedef struct
{
    sfEventType type;
    sfMouseButton button;
    sfVector2i position;
} sfMouseButtonEvent;

////////////////////////////////////////////////////////////
/// \brief Mouse wheel events parameters
///
////////////////////////////////////////////////////////////
typedef struct
{
    sfEventType type;
    sfMouseWheel wheel;
    float delta;
    sfVector2i position;
} sfMouseWheelScrollEvent;

////////////////////////////////////////////////////////////
/// \brief Joystick axis move event parameters
///
////////////////////////////////////////////////////////////
typedef struct
{
    sfEventType type;
    unsigned int joystickId;
    sfJoystickAxis axis;
    float position;
} sfJoystickMoveEvent;

////////////////////////////////////////////////////////////
/// \brief Joystick buttons events parameters
///
////////////////////////////////////////////////////////////
typedef struct
{
    sfEventType type;
    unsigned int joystickId;
    unsigned int button;
} sfJoystickButtonEvent;

////////////////////////////////////////////////////////////
/// \brief Joystick connection/disconnection event parameters
///
////////////////////////////////////////////////////////////
typedef struct
{
    sfEventType type;
    unsigned int joystickId;
} sfJoystickConnectEvent;

////////////////////////////////////////////////////////////
/// \brief Size events parameters
///
////////////////////////////////////////////////////////////
typedef struct
{
    sfEventType type;
    sfVector2u size;
} sfSizeEvent;

////////////////////////////////////////////////////////////
/// \brief Touch events parameters
///
////////////////////////////////////////////////////////////
typedef struct
{
    sfEventType type;
    unsigned int finger;
    sfVector2i position;
} sfTouchEvent;

////////////////////////////////////////////////////////////
/// \brief Sensor event parameters
///
////////////////////////////////////////////////////////////
typedef struct
{
    sfEventType type;
    sfSensorType sensorType;
    sfVector3f value;
} sfSensorEvent;

////////////////////////////////////////////////////////////
/// \brief sfEvent defines a system event and its parameters
///
////////////////////////////////////////////////////////////
typedef union {
    sfEventType type;                         ///< Type of the event
    sfSizeEvent size;                         ///< Size event parameters
    sfKeyEvent key;                           ///< Key event parameters
    sfTextEvent text;                         ///< Text event parameters
    sfMouseMoveEvent mouseMove;               ///< Mouse move event parameters
    sfMouseMoveRawEvent mouseMoveRaw;         ///< Mouse move raw event parameters
    sfMouseButtonEvent mouseButton;           ///< Mouse button event parameters
    sfMouseWheelScrollEvent mouseWheelScroll; ///< Mouse wheel event parameters
    sfJoystickMoveEvent joystickMove;         ///< Joystick move event parameters
    sfJoystickButtonEvent joystickButton;     ///< Joystick button event parameters
    sfJoystickConnectEvent joystickConnect;   ///< Joystick (dis)connect event parameters
    sfTouchEvent touch;                       ///< Touch events parameters
    sfSensorEvent sensor;                     ///< Sensor event parameters
} sfEvent;

[[nodiscard]] inline bool convertEvent(const std::optional<sf::Event> &sfmlEvent, sfEvent &event) {
    if (!sfmlEvent)
        return false;

    if (sfmlEvent->is<sf::Event::Closed>()) {
        event.type = sfEvtClosed;
    } else if (const auto *resized = sfmlEvent->getIf<sf::Event::Resized>()) {
        event.type = sfEvtResized;
        event.size.size = {resized->size.x, resized->size.y};
    } else if (sfmlEvent->is<sf::Event::FocusLost>()) {
        event.type = sfEvtFocusLost;
    } else if (sfmlEvent->is<sf::Event::FocusGained>()) {
        event.type = sfEvtFocusGained;
    } else if (const auto *textEntered = sfmlEvent->getIf<sf::Event::TextEntered>()) {
        event.type = sfEvtTextEntered;
        event.text.unicode = textEntered->unicode;
    } else if (const auto *keyReleased = sfmlEvent->getIf<sf::Event::KeyReleased>()) {
        event.type = sfEvtKeyReleased;
        event.key.code = static_cast<sfKeyCode>(keyReleased->code);
        event.key.scancode = static_cast<sfScancode>(keyReleased->scancode);
        event.key.alt = keyReleased->alt;
        event.key.control = keyReleased->control;
        event.key.shift = keyReleased->shift;
        event.key.system = keyReleased->system;
    } else if (const auto *keyPressed = sfmlEvent->getIf<sf::Event::KeyPressed>()) {
        event.type = sfEvtKeyPressed;
        event.key.code = static_cast<sfKeyCode>(keyPressed->code);
        event.key.scancode = static_cast<sfScancode>(keyPressed->scancode);
        event.key.alt = keyPressed->alt;
        event.key.control = keyPressed->control;
        event.key.shift = keyPressed->shift;
        event.key.system = keyPressed->system;
    } else if (const auto *mouseWheelScrolled = sfmlEvent->getIf<sf::Event::MouseWheelScrolled>()) {
        event.type = sfEvtMouseWheelScrolled;
        event.mouseWheelScroll.wheel = static_cast<sfMouseWheel>(mouseWheelScrolled->wheel);
        event.mouseWheelScroll.delta = mouseWheelScrolled->delta;
        event.mouseWheelScroll.position = {mouseWheelScrolled->position.x, mouseWheelScrolled->position.y};
    } else if (const auto *mouseButtonPressed = sfmlEvent->getIf<sf::Event::MouseButtonPressed>()) {
        event.type = sfEvtMouseButtonPressed;
        event.mouseButton.button = static_cast<sfMouseButton>(mouseButtonPressed->button);
        event.mouseButton.position = {mouseButtonPressed->position.x, mouseButtonPressed->position.y};
    } else if (const auto *mouseButtonReleased = sfmlEvent->getIf<sf::Event::MouseButtonReleased>()) {
        event.type = sfEvtMouseButtonReleased;
        event.mouseButton.button = static_cast<sfMouseButton>(mouseButtonReleased->button);
        event.mouseButton.position = {mouseButtonReleased->position.x, mouseButtonReleased->position.y};
    } else if (const auto *mouseMoved = sfmlEvent->getIf<sf::Event::MouseMoved>()) {
        event.type = sfEvtMouseMoved;
        event.mouseMove.position = {mouseMoved->position.x, mouseMoved->position.y};
    } else if (const auto *mouseMovedRaw = sfmlEvent->getIf<sf::Event::MouseMovedRaw>()) {
        event.type = sfEvtMouseMovedRaw;
        event.mouseMoveRaw.delta = {mouseMovedRaw->delta.x, mouseMovedRaw->delta.y};
    } else if (sfmlEvent->is<sf::Event::MouseEntered>()) {
        event.type = sfEvtMouseEntered;
    } else if (sfmlEvent->is<sf::Event::MouseLeft>()) {
        event.type = sfEvtMouseLeft;
    } else if (const auto *joystickButtonPressed = sfmlEvent->getIf<sf::Event::JoystickButtonPressed>()) {
        event.type = sfEvtJoystickButtonPressed;
        event.joystickButton.joystickId = joystickButtonPressed->joystickId;
        event.joystickButton.button = joystickButtonPressed->button;
    } else if (const auto *joystickButtonReleased = sfmlEvent->getIf<sf::Event::JoystickButtonReleased>()) {
        event.type = sfEvtJoystickButtonReleased;
        event.joystickButton.joystickId = joystickButtonReleased->joystickId;
        event.joystickButton.button = joystickButtonReleased->button;
    } else if (const auto *joystickMoved = sfmlEvent->getIf<sf::Event::JoystickMoved>()) {
        event.type = sfEvtJoystickMoved;
        event.joystickMove.joystickId = joystickMoved->joystickId;
        event.joystickMove.axis = static_cast<sfJoystickAxis>(joystickMoved->axis);
        event.joystickMove.position = joystickMoved->position;
    } else if (const auto *joystickConnected = sfmlEvent->getIf<sf::Event::JoystickConnected>()) {
        event.type = sfEvtJoystickConnected;
        event.joystickConnect.joystickId = joystickConnected->joystickId;
    } else if (const auto *joystickDisconnected = sfmlEvent->getIf<sf::Event::JoystickDisconnected>()) {
        event.type = sfEvtJoystickDisconnected;
        event.joystickConnect.joystickId = joystickDisconnected->joystickId;
    } else if (const auto *touchBegan = sfmlEvent->getIf<sf::Event::TouchBegan>()) {
        event.type = sfEvtTouchBegan;
        event.touch.finger = touchBegan->finger;
        event.touch.position = {touchBegan->position.x, touchBegan->position.y};
    } else if (const auto *touchMoved = sfmlEvent->getIf<sf::Event::TouchMoved>()) {
        event.type = sfEvtTouchMoved;
        event.touch.finger = touchMoved->finger;
        event.touch.position = {touchMoved->position.x, touchMoved->position.y};
    } else if (const auto *touchEnded = sfmlEvent->getIf<sf::Event::TouchEnded>()) {
        event.type = sfEvtTouchEnded;
        event.touch.finger = touchEnded->finger;
        event.touch.position = {touchEnded->position.x, touchEnded->position.y};
    } else if (const auto *sensorChanged = sfmlEvent->getIf<sf::Event::SensorChanged>()) {
        event.type = sfEvtSensorChanged;
        event.sensor.sensorType = static_cast<sfSensorType>(sensorChanged->type);
        event.sensor.value = {sensorChanged->value.x, sensorChanged->value.y, sensorChanged->value.z};
    }

    return true;
}
