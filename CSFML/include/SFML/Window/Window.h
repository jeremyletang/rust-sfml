
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

#ifndef SFML_WINDOW_H
#define SFML_WINDOW_H

// Headers

#include <SFML/Config.h>
#include <SFML/System/Vector2.h>
#include <SFML/Window/Event.h>

#include <SFML/Window/Types.h>
#include <SFML/Window/VideoMode.h>
#include <SFML/Window/WindowHandle.h>

typedef enum {
    sfNone = 0,                                      ///< No border / title bar (this flag and all others are mutually exclusive)
    sfTitlebar = 1 << 0,                             ///< Title bar + fixed border
    sfResize = 1 << 1,                               ///< Titlebar + resizable border + maximize button
    sfClose = 1 << 2,                                ///< Titlebar + close button
    sfFullscreen = 1 << 3,                           ///< Fullscreen mode (this flag and all others are mutually exclusive)
    sfDefaultStyle = sfTitlebar | sfResize | sfClose ///< Default window style
} sfWindowStyle;

typedef enum {
    sfContextDefault = 0,   ///< Non-debug, compatibility context (this and the core attribute are mutually exclusive)
    sfContextCore = 1 << 0, ///< Core attribute
    sfContextDebug = 1 << 2 ///< Debug attribute
} sfContextAttribute;

typedef struct
{
    unsigned int depthBits;         ///< Bits of the depth buffer
    unsigned int stencilBits;       ///< Bits of the stencil buffer
    unsigned int antialiasingLevel; ///< Level of antialiasing
    unsigned int majorVersion;      ///< Major number of the context version to create
    unsigned int minorVersion;      ///< Minor number of the context version to create
    sfUint32 attributeFlags;        ///< The attribute flags to create the context with
    sfBool sRgbCapable;             ///< Whether the context framebuffer is sRGB capable
} sfContextSettings;

extern "C" sfWindow *sfWindow_create(sfVideoMode mode, const char *title, sfUint32 style, const sfContextSettings *settings);

extern "C" sfWindow *sfWindow_createUnicode(sfVideoMode mode, const sfUint32 *title, sfUint32 style, const sfContextSettings *settings);

extern "C" sfWindow *sfWindow_createFromHandle(sfWindowHandle handle, const sfContextSettings *settings);

extern "C" void sfWindow_destroy(sfWindow *window);

extern "C" void sfWindow_close(sfWindow *window);

extern "C" sfBool sfWindow_isOpen(const sfWindow *window);

extern "C" sfContextSettings sfWindow_getSettings(const sfWindow *window);

extern "C" sfBool sfWindow_pollEvent(sfWindow *window, sfEvent *event);

extern "C" sfBool sfWindow_waitEvent(sfWindow *window, sfEvent *event);

extern "C" sfVector2i sfWindow_getPosition(const sfWindow *window);

extern "C" void sfWindow_setPosition(sfWindow *window, sfVector2i position);

extern "C" sfVector2u sfWindow_getSize(const sfWindow *window);

extern "C" void sfWindow_setSize(sfWindow *window, sfVector2u size);

extern "C" void sfWindow_setTitle(sfWindow *window, const char *title);

extern "C" void sfWindow_setUnicodeTitle(sfWindow *window, const sfUint32 *title);

extern "C" void sfWindow_setIcon(sfWindow *window, unsigned int width, unsigned int height, const sfUint8 *pixels);

extern "C" void sfWindow_setVisible(sfWindow *window, sfBool visible);

extern "C" void sfWindow_setVerticalSyncEnabled(sfWindow *window, sfBool enabled);

extern "C" void sfWindow_setMouseCursorVisible(sfWindow *window, sfBool visible);

extern "C" void sfWindow_setMouseCursorGrabbed(sfWindow *window, sfBool grabbed);

extern "C" void sfWindow_setMouseCursor(sfWindow *window, const sfCursor *cursor);

extern "C" void sfWindow_setKeyRepeatEnabled(sfWindow *window, sfBool enabled);

extern "C" void sfWindow_setFramerateLimit(sfWindow *window, unsigned int limit);

extern "C" void sfWindow_setJoystickThreshold(sfWindow *window, float threshold);

extern "C" sfBool sfWindow_setActive(sfWindow *window, sfBool active);

extern "C" void sfWindow_requestFocus(sfWindow *window);

extern "C" sfBool sfWindow_hasFocus(const sfWindow *window);

extern "C" void sfWindow_display(sfWindow *window);

extern "C" sfWindowHandle sfWindow_getSystemHandle(const sfWindow *window);

#endif // SFML_WINDOW_H
