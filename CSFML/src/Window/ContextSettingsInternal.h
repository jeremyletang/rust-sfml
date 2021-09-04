
//
// SFML - Simple and Fast Multimedia Library
// Copyright (C) 2007-2016 Laurent Gomila (laurent@sfml-dev.org)
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

#ifndef SFML_CONTEXTSETTINGSINTERNAL_H
#define SFML_CONTEXTSETTINGSINTERNAL_H

// Headers

#include "Window/Window.h"
#include <SFML/Window.hpp>

namespace priv {

// Create a "null" sfContextSettings that's returned in case of an error.

inline sfContextSettings sfContextSettings_null() {
    sfContextSettings settings = {0, 0, 0, 0, 0, 0, sfFalse};

    return settings;
}

// Read the data of an sf::ContextSettings into an sfContextSettings

inline void sfContextSettings_readFromCpp(const sf::ContextSettings &from, sfContextSettings &to) {
    to.depthBits = from.depthBits;
    to.stencilBits = from.stencilBits;
    to.antialiasingLevel = from.antialiasingLevel;
    to.majorVersion = from.majorVersion;
    to.minorVersion = from.minorVersion;
    to.attributeFlags = from.attributeFlags;
    to.sRgbCapable = from.sRgbCapable ? sfTrue : sfFalse;
}

// Write the data of an sfContextSettings into an sf::ContextSettings

inline void sfContextSettings_writeToCpp(const sfContextSettings &from, sf::ContextSettings &to) {
    to.depthBits = from.depthBits;
    to.stencilBits = from.stencilBits;
    to.antialiasingLevel = from.antialiasingLevel;
    to.majorVersion = from.majorVersion;
    to.minorVersion = from.minorVersion;
    to.attributeFlags = from.attributeFlags;
    to.sRgbCapable = from.sRgbCapable == sfTrue;
}
} // namespace priv

#endif // SFML_CONTEXTSETTINGSINTERNAL_H
