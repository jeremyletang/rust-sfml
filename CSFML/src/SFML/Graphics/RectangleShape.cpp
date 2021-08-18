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
#include <SFML/Graphics/RectangleShape.h>
#include <SFML/Graphics/RectangleShapeStruct.h>
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/ConvertTransform.hpp>
#include <SFML/Internal.h>


////////////////////////////////////////////////////////////
sfRectangleShape* sfRectangleShape_create(void)
{
    return new sfRectangleShape;
}


////////////////////////////////////////////////////////////
sfRectangleShape* sfRectangleShape_copy(const sfRectangleShape* shape)
{
    CSFML_CHECK_RETURN(shape, NULL);

    return new sfRectangleShape(*shape);
}


////////////////////////////////////////////////////////////
void sfRectangleShape_destroy(sfRectangleShape* shape)
{
    delete shape;
}


////////////////////////////////////////////////////////////
void sfRectangleShape_setPosition(sfRectangleShape* shape, sfVector2f position)
{
    CSFML_CALL(shape, setPosition(position.x, position.y));
}


////////////////////////////////////////////////////////////
void sfRectangleShape_setRotation(sfRectangleShape* shape, float angle)
{
    CSFML_CALL(shape, setRotation(angle));
}


////////////////////////////////////////////////////////////
void sfRectangleShape_setScale(sfRectangleShape* shape, sfVector2f scale)
{
    CSFML_CALL(shape, setScale(scale.x, scale.y));
}


////////////////////////////////////////////////////////////
void sfRectangleShape_setOrigin(sfRectangleShape* shape, sfVector2f origin)
{
    CSFML_CALL(shape, setOrigin(origin.x, origin.y));
}


////////////////////////////////////////////////////////////
sfVector2f sfRectangleShape_getPosition(const sfRectangleShape* shape)
{
    sfVector2f position = {0, 0};
    CSFML_CHECK_RETURN(shape, position);

    sf::Vector2f sfmlPos = shape->This.getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}


////////////////////////////////////////////////////////////
float sfRectangleShape_getRotation(const sfRectangleShape* shape)
{
    CSFML_CALL_RETURN(shape, getRotation(), 0.f);
}


////////////////////////////////////////////////////////////
sfVector2f sfRectangleShape_getScale(const sfRectangleShape* shape)
{
    sfVector2f scale = {0, 0};
    CSFML_CHECK_RETURN(shape, scale);

    sf::Vector2f sfmlScale = shape->This.getScale();
    scale.x = sfmlScale.x;
    scale.y = sfmlScale.y;

    return scale;
}


////////////////////////////////////////////////////////////
sfVector2f sfRectangleShape_getOrigin(const sfRectangleShape* shape)
{
    sfVector2f origin = {0, 0};
    CSFML_CHECK_RETURN(shape, origin);

    sf::Vector2f sfmlOrigin = shape->This.getOrigin();
    origin.x = sfmlOrigin.x;
    origin.y = sfmlOrigin.y;

    return origin;
}


////////////////////////////////////////////////////////////
void sfRectangleShape_move(sfRectangleShape* shape, sfVector2f offset)
{
    CSFML_CALL(shape, move(offset.x, offset.y));
}


////////////////////////////////////////////////////////////
void sfRectangleShape_rotate(sfRectangleShape* shape, float angle)
{
    CSFML_CALL(shape, rotate(angle));
}


////////////////////////////////////////////////////////////
void sfRectangleShape_scale(sfRectangleShape* shape, sfVector2f factors)
{
    CSFML_CALL(shape, scale(factors.x, factors.y));
}


////////////////////////////////////////////////////////////
sfTransform sfRectangleShape_getTransform(const sfRectangleShape* shape)
{
    CSFML_CHECK_RETURN(shape, sfTransform_Identity);

    shape->Transform = convertTransform(shape->This.getTransform());
    return shape->Transform;
}


////////////////////////////////////////////////////////////
sfTransform sfRectangleShape_getInverseTransform(const sfRectangleShape* shape)
{
    CSFML_CHECK_RETURN(shape, sfTransform_Identity);

    shape->InverseTransform = convertTransform(shape->This.getInverseTransform());
    return shape->InverseTransform;
}


////////////////////////////////////////////////////////////
void sfRectangleShape_setTexture(sfRectangleShape* shape, const sfTexture* texture, sfBool resetRect)
{
    CSFML_CALL(shape, setTexture(texture ? texture->This : NULL, resetRect == sfTrue));
    shape->Texture = texture;
}


////////////////////////////////////////////////////////////
void sfRectangleShape_setTextureRect(sfRectangleShape* shape, sfIntRect rect)
{
    CSFML_CALL(shape, setTextureRect(sf::IntRect(rect.left, rect.top, rect.width, rect.height)));
}


////////////////////////////////////////////////////////////
void sfRectangleShape_setFillColor(sfRectangleShape* shape, sfColor color)
{
    CSFML_CALL(shape, setFillColor(sf::Color(color.r, color.g, color.b, color.a)));
}


////////////////////////////////////////////////////////////
void sfRectangleShape_setOutlineColor(sfRectangleShape* shape, sfColor color)
{
    CSFML_CALL(shape, setOutlineColor(sf::Color(color.r, color.g, color.b, color.a)));
}


////////////////////////////////////////////////////////////
void sfRectangleShape_setOutlineThickness(sfRectangleShape* shape, float thickness)
{
    CSFML_CALL(shape, setOutlineThickness(thickness));
}


////////////////////////////////////////////////////////////
const sfTexture* sfRectangleShape_getTexture(const sfRectangleShape* shape)
{
    CSFML_CHECK_RETURN(shape, NULL);

    return shape->Texture;
}


////////////////////////////////////////////////////////////
sfIntRect sfRectangleShape_getTextureRect(const sfRectangleShape* shape)
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
sfColor sfRectangleShape_getFillColor(const sfRectangleShape* shape)
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
sfColor sfRectangleShape_getOutlineColor(const sfRectangleShape* shape)
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
float sfRectangleShape_getOutlineThickness(const sfRectangleShape* shape)
{
    CSFML_CALL_RETURN(shape, getOutlineThickness(), 0.f);
}


////////////////////////////////////////////////////////////
size_t sfRectangleShape_getPointCount(const sfRectangleShape* shape)
{
    CSFML_CALL_RETURN(shape, getPointCount(), 0);
}


////////////////////////////////////////////////////////////
sfVector2f sfRectangleShape_getPoint(const sfRectangleShape* shape, size_t index)
{
    sfVector2f point = {0, 0};
    CSFML_CHECK_RETURN(shape, point);

    sf::Vector2f sfmlPoint = shape->This.getPoint(index);
    point.x = sfmlPoint.x;
    point.y = sfmlPoint.y;

    return point;
}


////////////////////////////////////////////////////////////
void sfRectangleShape_setSize(sfRectangleShape* shape, sfVector2f size)
{
    CSFML_CALL(shape, setSize(sf::Vector2f(size.x, size.y)));
}


////////////////////////////////////////////////////////////
sfVector2f sfRectangleShape_getSize(const sfRectangleShape* shape)
{
    sfVector2f size = {0, 0};
    CSFML_CHECK_RETURN(shape, size);

    sf::Vector2f sfmlSize = shape->This.getSize();
    size.x = sfmlSize.x;
    size.y = sfmlSize.y;

    return size;
}


////////////////////////////////////////////////////////////
sfFloatRect sfRectangleShape_getLocalBounds(const sfRectangleShape* shape)
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
sfFloatRect sfRectangleShape_getGlobalBounds(const sfRectangleShape* shape)
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
