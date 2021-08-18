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
#include <SFML/Graphics/Sprite.h>
#include <SFML/Graphics/SpriteStruct.h>
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/Image.hpp>
#include <SFML/Graphics/ConvertTransform.hpp>
#include <SFML/Internal.h>


////////////////////////////////////////////////////////////
sfSprite* sfSprite_create(void)
{
    sfSprite* sprite = new sfSprite;
    sprite->Texture = NULL;

    return sprite;
}


////////////////////////////////////////////////////////////
sfSprite* sfSprite_copy(const sfSprite* sprite)
{
    CSFML_CHECK_RETURN(sprite, NULL);

    return new sfSprite(*sprite);
}


////////////////////////////////////////////////////////////
void sfSprite_destroy(sfSprite* sprite)
{
    delete sprite;
}


////////////////////////////////////////////////////////////
void sfSprite_setPosition(sfSprite* sprite, sfVector2f position)
{
    CSFML_CALL(sprite, setPosition(position.x, position.y));
}


////////////////////////////////////////////////////////////
void sfSprite_setRotation(sfSprite* sprite, float angle)
{
    CSFML_CALL(sprite, setRotation(angle));
}


////////////////////////////////////////////////////////////
void sfSprite_setScale(sfSprite* sprite, sfVector2f scale)
{
    CSFML_CALL(sprite, setScale(scale.x, scale.y));
}


////////////////////////////////////////////////////////////
void sfSprite_setOrigin(sfSprite* sprite, sfVector2f origin)
{
    CSFML_CALL(sprite, setOrigin(origin.x, origin.y));
}


////////////////////////////////////////////////////////////
sfVector2f sfSprite_getPosition(const sfSprite* sprite)
{
    sfVector2f position = {0, 0};
    CSFML_CHECK_RETURN(sprite, position);

    sf::Vector2f sfmlPos = sprite->This.getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}


////////////////////////////////////////////////////////////
float sfSprite_getRotation(const sfSprite* sprite)
{
    CSFML_CALL_RETURN(sprite, getRotation(), 0.f);
}


////////////////////////////////////////////////////////////
sfVector2f sfSprite_getScale(const sfSprite* sprite)
{
    sfVector2f scale = {0, 0};
    CSFML_CHECK_RETURN(sprite, scale);

    sf::Vector2f sfmlScale = sprite->This.getScale();
    scale.x = sfmlScale.x;
    scale.y = sfmlScale.y;

    return scale;
}


////////////////////////////////////////////////////////////
sfVector2f sfSprite_getOrigin(const sfSprite* sprite)
{
    sfVector2f origin = {0, 0};
    CSFML_CHECK_RETURN(sprite, origin);

    sf::Vector2f sfmlOrigin = sprite->This.getOrigin();
    origin.x = sfmlOrigin.x;
    origin.y = sfmlOrigin.y;

    return origin;
}


////////////////////////////////////////////////////////////
void sfSprite_move(sfSprite* sprite, sfVector2f offset)
{
    CSFML_CALL(sprite, move(offset.x, offset.y));
}


////////////////////////////////////////////////////////////
void sfSprite_rotate(sfSprite* sprite, float angle)
{
    CSFML_CALL(sprite, rotate(angle));
}


////////////////////////////////////////////////////////////
void sfSprite_scale(sfSprite* sprite, sfVector2f factors)
{
    CSFML_CALL(sprite, scale(factors.x, factors.y));
}


////////////////////////////////////////////////////////////
sfTransform sfSprite_getTransform(const sfSprite* sprite)
{
    CSFML_CHECK_RETURN(sprite, sfTransform_Identity);

    sprite->Transform = convertTransform(sprite->This.getTransform());
    return sprite->Transform;
}


////////////////////////////////////////////////////////////
sfTransform sfSprite_getInverseTransform(const sfSprite* sprite)
{
    CSFML_CHECK_RETURN(sprite, sfTransform_Identity);

    sprite->InverseTransform = convertTransform(sprite->This.getInverseTransform());
    return sprite->InverseTransform;
}


////////////////////////////////////////////////////////////
void sfSprite_setTexture(sfSprite* sprite, const sfTexture* texture, sfBool resetRect)
{
    if (texture && texture->This)
    {
        CSFML_CALL(sprite, setTexture(*texture->This, resetRect == sfTrue));
        sprite->Texture = texture;
    }
}


////////////////////////////////////////////////////////////
void sfSprite_setTextureRect(sfSprite* sprite, sfIntRect rectangle)
{
    CSFML_CALL(sprite, setTextureRect(sf::IntRect(rectangle.left, rectangle.top, rectangle.width, rectangle.height)));
}


////////////////////////////////////////////////////////////
void sfSprite_setColor(sfSprite* sprite, sfColor color)
{
    CSFML_CALL(sprite, setColor(sf::Color(color.r, color.g, color.b, color.a)));
}


////////////////////////////////////////////////////////////
const sfTexture* sfSprite_getTexture(const sfSprite* sprite)
{
    CSFML_CHECK_RETURN(sprite, NULL);

    return sprite->Texture;
}


////////////////////////////////////////////////////////////
sfIntRect sfSprite_getTextureRect(const sfSprite* sprite)
{
    sfIntRect rect = {0, 0, 0, 0};
    CSFML_CHECK_RETURN(sprite, rect);

    sf::IntRect sfmlRect = sprite->This.getTextureRect();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}


////////////////////////////////////////////////////////////
sfColor sfSprite_getColor(const sfSprite* sprite)
{
    sfColor color = {0, 0, 0, 0};
    CSFML_CHECK_RETURN(sprite, color);

    sf::Color sfmlColor = sprite->This.getColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}


////////////////////////////////////////////////////////////
sfFloatRect sfSprite_getLocalBounds(const sfSprite* sprite)
{
    sfFloatRect rect = {0, 0, 0, 0};
    CSFML_CHECK_RETURN(sprite, rect);

    sf::FloatRect sfmlRect = sprite->This.getLocalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}


////////////////////////////////////////////////////////////
sfFloatRect sfSprite_getGlobalBounds(const sfSprite* sprite)
{
    sfFloatRect rect = {0, 0, 0, 0};
    CSFML_CHECK_RETURN(sprite, rect);

    sf::FloatRect sfmlRect = sprite->This.getGlobalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}
