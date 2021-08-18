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

#ifndef SFML_WINDOWHANDLE_H
#define SFML_WINDOWHANDLE_H

////////////////////////////////////////////////////////////
// Headers
////////////////////////////////////////////////////////////
#include <SFML/Window/Export.h>


////////////////////////////////////////////////////////////
// Define a low-level window handle type, specific to
// each platform
////////////////////////////////////////////////////////////
#if defined(CSFML_SYSTEM_WINDOWS)

    // Window handle is HWND (HWND__*) on Windows
    struct HWND__;
    typedef struct HWND__* sfWindowHandle;

#elif defined(CSFML_SYSTEM_LINUX) || defined(CSFML_SYSTEM_FREEBSD)

    // Window handle is Window (unsigned long) on Unix - X11
    typedef unsigned long sfWindowHandle;

#elif defined(CSFML_SYSTEM_MACOS)

    // Window handle is NSWindow (void*) on Mac OS X - Cocoa
	typedef void* sfWindowHandle;

#endif


#endif // SFML_WINDOWHANDLE_H
