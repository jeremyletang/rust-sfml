
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

#ifndef SFML_TEXTURESTRUCT_H
#define SFML_TEXTURESTRUCT_H

// Headers

#include <SFML/Graphics/Texture.hpp>

// Internal structure of sfTexture

struct sfTexture {
    sfTexture() {
        This = new sf::Texture;
        OwnInstance = true;
    }

    sfTexture(sf::Texture *texture) {
        This = texture;
        OwnInstance = false;
    }

    sfTexture(const sfTexture &texture) {
        This = texture.This ? new sf::Texture(*texture.This) : NULL;
        OwnInstance = true;
    }

    ~sfTexture() {
        if (OwnInstance)
            delete This;
    }

    sf::Texture *This;
    bool OwnInstance;
};

#endif // SFML_TEXTURESTRUCT_H
