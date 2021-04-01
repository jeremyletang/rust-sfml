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

////////////////////////////////////////////////////////////
// Headers
////////////////////////////////////////////////////////////
#include <SFML/Graphics/Rect.h>
#include <SFML/Graphics/Rect.hpp>
#include <SFML/Internal.h>


////////////////////////////////////////////////////////////
/// Check if a point is inside a rectangle's area
////////////////////////////////////////////////////////////
sfBool sfFloatRect_contains(const sfFloatRect* rect, float x, float y)
{
    CSFML_CHECK_RETURN(rect, sfFalse);
    return sf::FloatRect(rect->left, rect->top, rect->width, rect->height).contains(x, y);
}
sfBool sfIntRect_contains(const sfIntRect* rect, int x, int y)
{
    CSFML_CHECK_RETURN(rect, sfFalse);
    return sf::IntRect(rect->left, rect->top, rect->width, rect->height).contains(x, y);
}


////////////////////////////////////////////////////////////
/// Check intersection between two rectangles
////////////////////////////////////////////////////////////
sfBool sfFloatRect_intersects(const sfFloatRect* rect1, const sfFloatRect* rect2, sfFloatRect* intersection)
{
    CSFML_CHECK_RETURN(rect1, sfFalse);
    CSFML_CHECK_RETURN(rect2, sfFalse);

    sf::FloatRect SFMLRect1(rect1->left, rect1->top, rect1->width, rect1->height);
    sf::FloatRect SFMLRect2(rect2->left, rect2->top, rect2->width, rect2->height);

    if (intersection)
    {
        sf::FloatRect overlap;
        bool intersects = SFMLRect1.intersects(SFMLRect2, overlap);

        intersection->left   = overlap.left;
        intersection->top    = overlap.top;
        intersection->width  = overlap.width;
        intersection->height = overlap.height;

        return intersects;
    }
    else
    {
        return SFMLRect1.intersects(SFMLRect2);
    }
}
sfBool sfIntRect_intersects(const sfIntRect* rect1, const sfIntRect* rect2, sfIntRect* intersection)
{
    CSFML_CHECK_RETURN(rect1, sfFalse);
    CSFML_CHECK_RETURN(rect2, sfFalse);

    sf::IntRect SFMLRect1(rect1->left, rect1->top, rect1->width, rect1->height);
    sf::IntRect SFMLRect2(rect2->left, rect2->top, rect2->width, rect2->height);

    if (intersection)
    {
        sf::IntRect overlap;
        bool intersects = SFMLRect1.intersects(SFMLRect2, overlap);

        intersection->left   = overlap.left;
        intersection->top    = overlap.top;
        intersection->width  = overlap.width;
        intersection->height = overlap.height;

        return intersects;
    }
    else
    {
        return SFMLRect1.intersects(SFMLRect2);
    }
}
