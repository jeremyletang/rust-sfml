
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

#include <SFML/Window/VideoMode.h>
#include <SFML/Window/VideoMode.hpp>
#include <cstddef>

sfVideoMode sfVideoMode_getDesktopMode(void) {
    sf::VideoMode desktop = sf::VideoMode::getDesktopMode();
    sfVideoMode ret;
    ret.width = desktop.width;
    ret.height = desktop.height;
    ret.bitsPerPixel = desktop.bitsPerPixel;

    return ret;
}

const sfVideoMode *sfVideoMode_getFullscreenModes(size_t *count) {
    static std::vector<sfVideoMode> modes;

    // Populate the array on first call
    if (modes.empty()) {
        const std::vector<sf::VideoMode> &SFMLModes = sf::VideoMode::getFullscreenModes();
        for (std::vector<sf::VideoMode>::const_iterator it = SFMLModes.begin(); it != SFMLModes.end(); ++it) {
            sfVideoMode mode;
            mode.width = it->width;
            mode.height = it->height;
            mode.bitsPerPixel = it->bitsPerPixel;
            modes.push_back(mode);
        }
    }

    if (count)
        *count = modes.size();

    return !modes.empty() ? &modes[0] : NULL;
}

sfBool sfVideoMode_isValid(sfVideoMode mode) {
    sf::VideoMode videoMode(mode.width, mode.height, mode.bitsPerPixel);
    return videoMode.isValid() ? sfTrue : sfFalse;
}
