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
#include <SFML/Graphics/Transform.h>
#include <SFML/Graphics/Transform.hpp>
#include <SFML/Graphics/ConvertTransform.hpp>
#include <SFML/Internal.h>
#include <cstring>


////////////////////////////////////////////////////////////
const sfTransform sfTransform_Identity =
{
    1, 0, 0,
    0, 1, 0,
    0, 0, 1,
};


////////////////////////////////////////////////////////////
sfTransform sfTransform_fromMatrix(float a00, float a01, float a02,
                                   float a10, float a11, float a12,
                                   float a20, float a21, float a22)
{
    sfTransform transform = {a00, a01, a02, a10, a11, a12, a20, a21, a22};
    return transform;
}


////////////////////////////////////////////////////////////
void sfTransform_getMatrix(const sfTransform* transform, float* matrix)
{
    CSFML_CHECK(transform);

    sf::Transform converted = convertTransform(*transform);
    if (matrix)
        std::memcpy(matrix, converted.getMatrix(), 16 * sizeof(float));
}


////////////////////////////////////////////////////////////
sfTransform sfTransform_getInverse(const sfTransform* transform)
{
    CSFML_CHECK_RETURN(transform, sfTransform_Identity);

    return convertTransform(convertTransform(*transform).getInverse());
}


////////////////////////////////////////////////////////////
sfVector2f sfTransform_transformPoint(const sfTransform* transform, sfVector2f point)
{
    sfVector2f p = {0, 0};
    CSFML_CHECK_RETURN(transform, p);

    sf::Vector2f sfmlPoint = convertTransform(*transform).transformPoint(point.x, point.y);

    p.x = sfmlPoint.x;
    p.y = sfmlPoint.y;

    return p;
}


////////////////////////////////////////////////////////////
sfFloatRect sfTransform_transformRect(const sfTransform* transform, sfFloatRect rectangle)
{
    sfFloatRect rect = {0, 0, 0, 0};
    CSFML_CHECK_RETURN(transform, rect);

    sf::FloatRect sfmlRect = convertTransform(*transform).transformRect(sf::FloatRect(rectangle.left, rectangle.top, rectangle.width, rectangle.height));

    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}


////////////////////////////////////////////////////////////
void sfTransform_combine(sfTransform* transform, const sfTransform* other)
{
    CSFML_CHECK(transform);
    CSFML_CHECK(other);

    *transform = convertTransform(convertTransform(*transform).combine(convertTransform(*other)));
}


////////////////////////////////////////////////////////////
void sfTransform_translate(sfTransform* transform, float x, float y)
{
    CSFML_CHECK(transform);

    *transform = convertTransform(convertTransform(*transform).translate(x, y));
}


////////////////////////////////////////////////////////////
void sfTransform_rotate(sfTransform* transform, float angle)
{
    CSFML_CHECK(transform);

    *transform = convertTransform(convertTransform(*transform).rotate(angle));
}


////////////////////////////////////////////////////////////
void sfTransform_rotateWithCenter(sfTransform* transform, float angle, float centerX, float centerY)
{
    CSFML_CHECK(transform);

    *transform = convertTransform(convertTransform(*transform).rotate(angle, centerX, centerY));
}


////////////////////////////////////////////////////////////
void sfTransform_scale(sfTransform* transform, float scaleX, float scaleY)
{
    CSFML_CHECK(transform);

    *transform = convertTransform(convertTransform(*transform).scale(scaleX, scaleY));
}


////////////////////////////////////////////////////////////
void sfTransform_scaleWithCenter(sfTransform* transform, float scaleX, float scaleY, float centerX, float centerY)
{
    CSFML_CHECK(transform);

    *transform = convertTransform(convertTransform(*transform).scale(scaleX, scaleY, centerX, centerY));
}


////////////////////////////////////////////////////////////
sfBool sfTransform_equal(sfTransform* left, sfTransform* right)
{
    CSFML_CHECK_RETURN(left, false);
    CSFML_CHECK_RETURN(right, false);

    return (left->matrix == right->matrix) ? sfTrue : sfFalse;
}

