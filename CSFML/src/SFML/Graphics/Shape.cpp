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
#include <SFML/Graphics/Shape.h>
#include <SFML/Graphics/ShapeStruct.h>
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/ConvertTransform.hpp>
#include <SFML/Internal.h>


////////////////////////////////////////////////////////////
sfShape* sfShape_create(sfShapeGetPointCountCallback getPointCount,
                        sfShapeGetPointCallback getPoint,
                        void* userData)
{
    return new sfShape(getPointCount, getPoint, userData);
}


////////////////////////////////////////////////////////////
void sfShape_destroy(sfShape* shape)
{
    delete shape;
}


////////////////////////////////////////////////////////////
void sfShape_setPosition(sfShape* shape, sfVector2f position)
{
    CSFML_CALL(shape, setPosition(position.x, position.y));
}


////////////////////////////////////////////////////////////
void sfShape_setRotation(sfShape* shape, float angle)
{
    CSFML_CALL(shape, setRotation(angle));
}


////////////////////////////////////////////////////////////
void sfShape_setScale(sfShape* shape, sfVector2f scale)
{
    CSFML_CALL(shape, setScale(scale.x, scale.y));
}


////////////////////////////////////////////////////////////
void sfShape_setOrigin(sfShape* shape, sfVector2f origin)
{
    CSFML_CALL(shape, setOrigin(origin.x, origin.y));
}


////////////////////////////////////////////////////////////
sfVector2f sfShape_getPosition(const sfShape* shape)
{
    sfVector2f position = {0, 0};
    CSFML_CHECK_RETURN(shape, position);

    sf::Vector2f sfmlPos = shape->This.getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}


////////////////////////////////////////////////////////////
float sfShape_getRotation(const sfShape* shape)
{
    CSFML_CALL_RETURN(shape, getRotation(), 0.f);
}


////////////////////////////////////////////////////////////
sfVector2f sfShape_getScale(const sfShape* shape)
{
    sfVector2f scale = {0, 0};
    CSFML_CHECK_RETURN(shape, scale);

    sf::Vector2f sfmlScale = shape->This.getScale();
    scale.x = sfmlScale.x;
    scale.y = sfmlScale.y;

    return scale;
}


////////////////////////////////////////////////////////////
sfVector2f sfShape_getOrigin(const sfShape* shape)
{
    sfVector2f origin = {0, 0};
    CSFML_CHECK_RETURN(shape, origin);

    sf::Vector2f sfmlOrigin = shape->This.getOrigin();
    origin.x = sfmlOrigin.x;
    origin.y = sfmlOrigin.y;

    return origin;
}


////////////////////////////////////////////////////////////
void sfShape_move(sfShape* shape, sfVector2f offset)
{
    CSFML_CALL(shape, move(offset.x, offset.y));
}


////////////////////////////////////////////////////////////
void sfShape_rotate(sfShape* shape, float angle)
{
    CSFML_CALL(shape, rotate(angle));
}


////////////////////////////////////////////////////////////
void sfShape_scale(sfShape* shape, sfVector2f factors)
{
    CSFML_CALL(shape, scale(factors.x, factors.y));
}


////////////////////////////////////////////////////////////
sfTransform sfShape_getTransform(const sfShape* shape)
{
    CSFML_CHECK_RETURN(shape, sfTransform_Identity);

    shape->Transform = convertTransform(shape->This.getTransform());
    return shape->Transform;
}


////////////////////////////////////////////////////////////
sfTransform sfShape_getInverseTransform(const sfShape* shape)
{
    CSFML_CHECK_RETURN(shape, sfTransform_Identity);

    shape->InverseTransform = convertTransform(shape->This.getInverseTransform());
    return shape->InverseTransform;
}


////////////////////////////////////////////////////////////
void sfShape_setTexture(sfShape* shape, const sfTexture* texture, sfBool resetRect)
{
    CSFML_CALL(shape, setTexture(texture ? texture->This : NULL, resetRect == sfTrue));
    shape->Texture = texture;
}


////////////////////////////////////////////////////////////
void sfShape_setTextureRect(sfShape* shape, sfIntRect rect)
{
    CSFML_CALL(shape, setTextureRect(sf::IntRect(rect.left, rect.top, rect.width, rect.height)));
}


////////////////////////////////////////////////////////////
void sfShape_setFillColor(sfShape* shape, sfColor color)
{
    CSFML_CALL(shape, setFillColor(sf::Color(color.r, color.g, color.b, color.a)));
}


////////////////////////////////////////////////////////////
void sfShape_setOutlineColor(sfShape* shape, sfColor color)
{
    CSFML_CALL(shape, setOutlineColor(sf::Color(color.r, color.g, color.b, color.a)));
}


////////////////////////////////////////////////////////////
void sfShape_setOutlineThickness(sfShape* shape, float thickness)
{
    CSFML_CALL(shape, setOutlineThickness(thickness));
}


////////////////////////////////////////////////////////////
const sfTexture* sfShape_getTexture(const sfShape* shape)
{
    CSFML_CHECK_RETURN(shape, NULL);

    return shape->Texture;
}


////////////////////////////////////////////////////////////
sfIntRect sfShape_getTextureRect(const sfShape* shape)
{
    sfIntRect rect = {0, 0, 0, 0};
    CSFML_CHECK_RETURN(shape, rect);

    sf::IntRect sfmlRect = shape->This.getTextureRect();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}


////////////////////////////////////////////////////////////
sfColor sfShape_getFillColor(const sfShape* shape)
{
    sfColor color = {0, 0, 0, 0};
    CSFML_CHECK_RETURN(shape, color);

    sf::Color sfmlColor = shape->This.getFillColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}


////////////////////////////////////////////////////////////
sfColor sfShape_getOutlineColor(const sfShape* shape)
{
    sfColor color = {0, 0, 0, 0};
    CSFML_CHECK_RETURN(shape, color);

    sf::Color sfmlColor = shape->This.getOutlineColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}


////////////////////////////////////////////////////////////
float sfShape_getOutlineThickness(const sfShape* shape)
{
    CSFML_CALL_RETURN(shape, getOutlineThickness(), 0.f);
}


////////////////////////////////////////////////////////////
size_t sfShape_getPointCount(const sfShape* shape)
{
    CSFML_CALL_RETURN(shape, getPointCount(), 0);
}


////////////////////////////////////////////////////////////
sfVector2f sfShape_getPoint(const sfShape* shape, size_t index)
{
    sfVector2f point = {0, 0};
    CSFML_CHECK_RETURN(shape, point);

    sf::Vector2f sfmlPoint = shape->This.getPoint(index);
    point.x = sfmlPoint.x;
    point.y = sfmlPoint.y;

    return point;
}


////////////////////////////////////////////////////////////
sfFloatRect sfShape_getLocalBounds(const sfShape* shape)
{
    sfFloatRect rect = {0, 0, 0, 0};
    CSFML_CHECK_RETURN(shape, rect);

    sf::FloatRect sfmlRect = shape->This.getLocalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}


////////////////////////////////////////////////////////////
sfFloatRect sfShape_getGlobalBounds(const sfShape* shape)
{
    sfFloatRect rect = {0, 0, 0, 0};
    CSFML_CHECK_RETURN(shape, rect);

    sf::FloatRect sfmlRect = shape->This.getGlobalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}


////////////////////////////////////////////////////////////
void sfShape_update(sfShape* shape)
{
    CSFML_CALL(shape, update());
}
