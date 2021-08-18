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
#include <cstddef>


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
    sprite->This.setPosition(position.x, position.y);
}


////////////////////////////////////////////////////////////
void sfSprite_setRotation(sfSprite* sprite, float angle)
{
    sprite->This.setRotation(angle);
}


////////////////////////////////////////////////////////////
void sfSprite_setScale(sfSprite* sprite, sfVector2f scale)
{
    sprite->This.setScale(scale.x, scale.y);
}


////////////////////////////////////////////////////////////
void sfSprite_setOrigin(sfSprite* sprite, sfVector2f origin)
{
    sprite->This.setOrigin(origin.x, origin.y);
}


////////////////////////////////////////////////////////////
sfVector2f sfSprite_getPosition(const sfSprite* sprite)
{
    sfVector2f position = {0, 0};


    sf::Vector2f sfmlPos = sprite->This.getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}


////////////////////////////////////////////////////////////
float sfSprite_getRotation(const sfSprite* sprite)
{
    return sprite->This.getRotation();
}


////////////////////////////////////////////////////////////
sfVector2f sfSprite_getScale(const sfSprite* sprite)
{
    sfVector2f scale = {0, 0};


    sf::Vector2f sfmlScale = sprite->This.getScale();
    scale.x = sfmlScale.x;
    scale.y = sfmlScale.y;

    return scale;
}


////////////////////////////////////////////////////////////
sfVector2f sfSprite_getOrigin(const sfSprite* sprite)
{
    sfVector2f origin = {0, 0};


    sf::Vector2f sfmlOrigin = sprite->This.getOrigin();
    origin.x = sfmlOrigin.x;
    origin.y = sfmlOrigin.y;

    return origin;
}


////////////////////////////////////////////////////////////
void sfSprite_move(sfSprite* sprite, sfVector2f offset)
{
    sprite->This.move(offset.x, offset.y);
}


////////////////////////////////////////////////////////////
void sfSprite_rotate(sfSprite* sprite, float angle)
{
    sprite->This.rotate(angle);
}


////////////////////////////////////////////////////////////
void sfSprite_scale(sfSprite* sprite, sfVector2f factors)
{
    sprite->This.scale(factors.x, factors.y);
}


////////////////////////////////////////////////////////////
sfTransform sfSprite_getTransform(const sfSprite* sprite)
{


    sprite->Transform = convertTransform(sprite->This.getTransform());
    return sprite->Transform;
}


////////////////////////////////////////////////////////////
sfTransform sfSprite_getInverseTransform(const sfSprite* sprite)
{


    sprite->InverseTransform = convertTransform(sprite->This.getInverseTransform());
    return sprite->InverseTransform;
}


////////////////////////////////////////////////////////////
void sfSprite_setTexture(sfSprite* sprite, const sfTexture* texture, sfBool resetRect)
{
    if (texture && texture->This)
    {
        sprite->This.setTexture(*texture->This, resetRect == sfTrue);
        sprite->Texture = texture;
    }
}


////////////////////////////////////////////////////////////
void sfSprite_setTextureRect(sfSprite* sprite, sfIntRect rectangle)
{
    sprite->This.setTextureRect(sf::IntRect(rectangle.left, rectangle.top, rectangle.width, rectangle.height));
}


////////////////////////////////////////////////////////////
void sfSprite_setColor(sfSprite* sprite, sfColor color)
{
    sprite->This.setColor(sf::Color(color.r, color.g, color.b, color.a));
}


////////////////////////////////////////////////////////////
const sfTexture* sfSprite_getTexture(const sfSprite* sprite)
{


    return sprite->Texture;
}


////////////////////////////////////////////////////////////
sfIntRect sfSprite_getTextureRect(const sfSprite* sprite)
{
    sfIntRect rect = {0, 0, 0, 0};


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


    sf::FloatRect sfmlRect = sprite->This.getGlobalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}
