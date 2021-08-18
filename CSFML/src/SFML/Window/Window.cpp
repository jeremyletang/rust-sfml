
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

#include <SFML/Window/Window.h>
#include <SFML/Window/WindowStruct.h>
#include <cstddef>
#include <SFML/Window/ContextSettingsInternal.h>
#include <SFML/Window/CursorStruct.h>
#include <SFML/ConvertEvent.h>



sfWindow* sfWindow_create(sfVideoMode mode, const char* title, sfUint32 style, const sfContextSettings* settings)
{
    // Convert video mode
    sf::VideoMode videoMode(mode.width, mode.height, mode.bitsPerPixel);

    // Convert context settings
    sf::ContextSettings params;
    if (settings)
    {
        priv::sfContextSettings_writeToCpp(*settings, params);
    }

    // Create the window
    sfWindow* window = new sfWindow;
    window->This.create(videoMode, title, style, params);

    return window;
}


sfWindow* sfWindow_createUnicode(sfVideoMode mode, const sfUint32* title, sfUint32 style, const sfContextSettings* settings)
{
    // Convert video mode
    sf::VideoMode videoMode(mode.width, mode.height, mode.bitsPerPixel);

    // Convert context settings
    sf::ContextSettings params;
    if (settings)
    {
        priv::sfContextSettings_writeToCpp(*settings, params);
    }

    // Create the window
    sfWindow* window = new sfWindow;
    window->This.create(videoMode, title, style, params);

    return window;
}



sfWindow* sfWindow_createFromHandle(sfWindowHandle handle, const sfContextSettings* settings)
{
    // Convert context settings
    sf::ContextSettings params;
    if (settings)
    {
        priv::sfContextSettings_writeToCpp(*settings, params);
    }

    // Create the window
    sfWindow* window = new sfWindow;
    window->This.create(handle, params);

    return window;
}



void sfWindow_destroy(sfWindow* window)
{
    delete window;
}


void sfWindow_close(sfWindow* window)
{
    window->This.close();
}



sfBool sfWindow_isOpen(const sfWindow* window)
{
    return window->This.isOpen();
}



sfContextSettings sfWindow_getSettings(const sfWindow* window)
{
    sfContextSettings settings = priv::sfContextSettings_null();


    const sf::ContextSettings& params = window->This.getSettings();
    priv::sfContextSettings_readFromCpp(params, settings);

    return settings;
}



sfBool sfWindow_pollEvent(sfWindow* window, sfEvent* event)
{



    // Get the event
    sf::Event SFMLEvent;
    sfBool ret = window->This.pollEvent(SFMLEvent);

    // No event, return
    if (!ret)
        return sfFalse;

    // Convert the sf::Event event to a sfEvent
    convertEvent(SFMLEvent, event);

    return sfTrue;
}



sfBool sfWindow_waitEvent(sfWindow* window, sfEvent* event)
{



    // Get the event
    sf::Event SFMLEvent;
    sfBool ret = window->This.waitEvent(SFMLEvent);

    // Error, return
    if (!ret)
        return sfFalse;

    // Convert the sf::Event event to a sfEvent
    convertEvent(SFMLEvent, event);

    return sfTrue;
}



sfVector2i sfWindow_getPosition(const sfWindow* window)
{
    sfVector2i position = {0, 0};


    sf::Vector2i sfmlPos = window->This.getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}



void sfWindow_setPosition(sfWindow* window, sfVector2i position)
{
    window->This.setPosition(sf::Vector2i(position.x, position.y));
}



sfVector2u sfWindow_getSize(const sfWindow* window)
{
    sfVector2u size = {0, 0};


    sf::Vector2u sfmlSize = window->This.getSize();
    size.x = sfmlSize.x;
    size.y = sfmlSize.y;

    return size;
}



void sfWindow_setSize(sfWindow* window, sfVector2u size)
{
    window->This.setSize(sf::Vector2u(size.x, size.y));
}



void sfWindow_setTitle(sfWindow* window, const char* title)
{
    window->This.setTitle(title);
}



void sfWindow_setUnicodeTitle(sfWindow* window, const sfUint32* title)
{
    window->This.setTitle(title);
}



void sfWindow_setIcon(sfWindow* window, unsigned int width, unsigned int height, const sfUint8* pixels)
{
    window->This.setIcon(width, height, pixels);
}



void sfWindow_setVisible(sfWindow* window, sfBool visible)
{
    window->This.setVisible(visible == sfTrue);
}



void sfWindow_setMouseCursorVisible(sfWindow* window, sfBool visible)
{
    window->This.setMouseCursorVisible(visible == sfTrue);
}



void sfWindow_setMouseCursorGrabbed(sfWindow* window, sfBool grabbed)
{
    window->This.setMouseCursorGrabbed(grabbed == sfTrue);
}



void sfWindow_setMouseCursor(sfWindow* window, const sfCursor* cursor)
{


    window->This.setMouseCursor(cursor->This);
}


void sfWindow_setVerticalSyncEnabled(sfWindow* window, sfBool enabled)
{
    window->This.setVerticalSyncEnabled(enabled == sfTrue);
}



void sfWindow_setKeyRepeatEnabled(sfWindow* window, sfBool enabled)
{
    window->This.setKeyRepeatEnabled(enabled == sfTrue);
}



sfBool sfWindow_setActive(sfWindow* window, sfBool active)
{
    return window->This.setActive(active == sfTrue);
}



void sfWindow_requestFocus(sfWindow* window)
{
    window->This.requestFocus();
}



sfBool sfWindow_hasFocus(const sfWindow* window)
{
    return window->This.hasFocus();
}



void sfWindow_display(sfWindow* window)
{
    window->This.display();
}



void sfWindow_setFramerateLimit(sfWindow* window, unsigned int limit)
{
    window->This.setFramerateLimit(limit);
}



void sfWindow_setJoystickThreshold(sfWindow* window, float threshold)
{
    window->This.setJoystickThreshold(threshold);
}



sfWindowHandle sfWindow_getSystemHandle(const sfWindow* window)
{


    return (sfWindowHandle)window->This.getSystemHandle();
}
