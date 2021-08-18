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
#include <SFML/Graphics/Text.h>
#include <SFML/Graphics/TextStruct.h>
#include <SFML/Graphics/Font.h>
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/ConvertTransform.hpp>
#include <SFML/Internal.h>


////////////////////////////////////////////////////////////
sfText* sfText_create(void)
{
    sfText* text = new sfText;
    text->Font = NULL;

    return text;
}


////////////////////////////////////////////////////////////
sfText* sfText_copy(const sfText* text)
{
    CSFML_CHECK_RETURN(text, NULL);

    return new sfText(*text);
}


////////////////////////////////////////////////////////////
void sfText_destroy(sfText* text)
{
    delete text;
}


////////////////////////////////////////////////////////////
void sfText_setPosition(sfText* text, sfVector2f position)
{
    CSFML_CALL(text, setPosition(position.x, position.y));
}


////////////////////////////////////////////////////////////
void sfText_setRotation(sfText* text, float angle)
{
    CSFML_CALL(text, setRotation(angle));
}


////////////////////////////////////////////////////////////
void sfText_setScale(sfText* text, sfVector2f scale)
{
    CSFML_CALL(text, setScale(scale.x, scale.y));
}


////////////////////////////////////////////////////////////
void sfText_setOrigin(sfText* text, sfVector2f origin)
{
    CSFML_CALL(text, setOrigin(origin.x, origin.y));
}


////////////////////////////////////////////////////////////
sfVector2f sfText_getPosition(const sfText* text)
{
    sfVector2f position = {0, 0};
    CSFML_CHECK_RETURN(text, position);

    sf::Vector2f sfmlPos = text->This.getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}


////////////////////////////////////////////////////////////
float sfText_getRotation(const sfText* text)
{
    CSFML_CALL_RETURN(text, getRotation(), 0.f);
}


////////////////////////////////////////////////////////////
sfVector2f sfText_getScale(const sfText* text)
{
    sfVector2f scale = {0, 0};
    CSFML_CHECK_RETURN(text, scale);

    sf::Vector2f sfmlScale = text->This.getScale();
    scale.x = sfmlScale.x;
    scale.y = sfmlScale.y;

    return scale;
}


////////////////////////////////////////////////////////////
sfVector2f sfText_getOrigin(const sfText* text)
{
    sfVector2f origin = {0, 0};
    CSFML_CHECK_RETURN(text, origin);

    sf::Vector2f sfmlOrigin = text->This.getOrigin();
    origin.x = sfmlOrigin.x;
    origin.y = sfmlOrigin.y;

    return origin;
}


////////////////////////////////////////////////////////////
void sfText_move(sfText* text, sfVector2f offset)
{
    CSFML_CALL(text, move(offset.x, offset.y));
}


////////////////////////////////////////////////////////////
void sfText_rotate(sfText* text, float angle)
{
    CSFML_CALL(text, rotate(angle));
}


////////////////////////////////////////////////////////////
void sfText_scale(sfText* text, sfVector2f factors)
{
    CSFML_CALL(text, scale(factors.x, factors.y));
}


////////////////////////////////////////////////////////////
sfTransform sfText_getTransform(const sfText* text)
{
    CSFML_CHECK_RETURN(text, sfTransform_Identity);

    text->Transform = convertTransform(text->This.getTransform());
    return text->Transform;
}


////////////////////////////////////////////////////////////
sfTransform sfText_getInverseTransform(const sfText* text)
{
    CSFML_CHECK_RETURN(text, sfTransform_Identity);

    text->InverseTransform = convertTransform(text->This.getInverseTransform());
    return text->InverseTransform;
}


////////////////////////////////////////////////////////////
void sfText_setString(sfText* text, const char* string)
{
    CSFML_CALL(text, setString(string));
}


////////////////////////////////////////////////////////////
void sfText_setUnicodeString(sfText* text, const sfUint32* string)
{
    sf::String UTF32Text = string;
    CSFML_CALL(text, setString(UTF32Text));
}


////////////////////////////////////////////////////////////
void sfText_setFont(sfText* text, const sfFont* font)
{
    CSFML_CHECK(font);

    CSFML_CALL(text, setFont(font->This));
    text->Font = font;
}


////////////////////////////////////////////////////////////
void sfText_setCharacterSize(sfText* text, unsigned int size)
{
    CSFML_CALL(text, setCharacterSize(size));
}


////////////////////////////////////////////////////////////
void sfText_setLineSpacing(sfText* text, float spacingFactor)
{
    CSFML_CALL(text, setLineSpacing(spacingFactor));
}


////////////////////////////////////////////////////////////
void sfText_setLetterSpacing(sfText* text, float spacingFactor)
{
    CSFML_CALL(text, setLetterSpacing(spacingFactor));
}


////////////////////////////////////////////////////////////
void sfText_setStyle(sfText* text, sfUint32 style)
{
    CSFML_CALL(text, setStyle(style));
}


////////////////////////////////////////////////////////////
void sfText_setColor(sfText* text, sfColor color)
{
    sfText_setFillColor(text, color);
}


////////////////////////////////////////////////////////////
void sfText_setFillColor(sfText* text, sfColor color)
{
    CSFML_CALL(text, setFillColor(sf::Color(color.r, color.g, color.b, color.a)));
}


////////////////////////////////////////////////////////////
void sfText_setOutlineColor(sfText* text, sfColor color)
{
    CSFML_CALL(text, setOutlineColor(sf::Color(color.r, color.g, color.b, color.a)));
}


////////////////////////////////////////////////////////////
void sfText_setOutlineThickness(sfText* text, float thickness)
{
    CSFML_CALL(text, setOutlineThickness(thickness));
}


////////////////////////////////////////////////////////////
const char* sfText_getString(const sfText* text)
{
    CSFML_CHECK_RETURN(text, NULL);

    text->String = text->This.getString().toAnsiString();

    return text->String.c_str();
}


////////////////////////////////////////////////////////////
const sfUint32* sfText_getUnicodeString(const sfText* text)
{
    CSFML_CHECK_RETURN(text, NULL);

    return text->This.getString().getData();
}


////////////////////////////////////////////////////////////
const sfFont* sfText_getFont(const sfText* text)
{
    CSFML_CHECK_RETURN(text, NULL);

    return text->Font;
}


////////////////////////////////////////////////////////////
unsigned int sfText_getCharacterSize(const sfText* text)
{
    CSFML_CALL_RETURN(text, getCharacterSize(), 0);
}


////////////////////////////////////////////////////////////
float sfText_getLetterSpacing(const sfText* text)
{
    CSFML_CALL_RETURN(text, getLetterSpacing(), 1);
}


////////////////////////////////////////////////////////////
float sfText_getLineSpacing(const sfText* text)
{
    CSFML_CALL_RETURN(text, getLineSpacing(), 1);
}



////////////////////////////////////////////////////////////
sfUint32 sfText_getStyle(const sfText* text)
{
    CSFML_CALL_RETURN(text, getStyle(), 0);
}


////////////////////////////////////////////////////////////
sfColor sfText_getColor(const sfText* text)
{
    return sfText_getFillColor(text);
}


////////////////////////////////////////////////////////////
sfColor sfText_getFillColor(const sfText* text)
{
    sfColor color = {0, 0, 0, 0};
    CSFML_CHECK_RETURN(text, color);

    sf::Color sfmlColor = text->This.getFillColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}


////////////////////////////////////////////////////////////
sfColor sfText_getOutlineColor(const sfText* text)
{
    sfColor color = { 0, 0, 0, 0 };
    CSFML_CHECK_RETURN(text, color);

    sf::Color sfmlColor = text->This.getOutlineColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}


////////////////////////////////////////////////////////////
float sfText_getOutlineThickness(const sfText* text)
{
    CSFML_CALL_RETURN(text, getOutlineThickness(), 0.f);
}


////////////////////////////////////////////////////////////
sfVector2f sfText_findCharacterPos(const sfText* text, size_t index)
{
    sfVector2f position = {0, 0};
    CSFML_CHECK_RETURN(text, position);

    sf::Vector2f sfmlPos = text->This.findCharacterPos(index);
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}


////////////////////////////////////////////////////////////
sfFloatRect sfText_getLocalBounds(const sfText* text)
{
    sfFloatRect rect = {0, 0, 0, 0};
    CSFML_CHECK_RETURN(text, rect);

    sf::FloatRect sfmlRect = text->This.getLocalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}


////////////////////////////////////////////////////////////
sfFloatRect sfText_getGlobalBounds(const sfText* text)
{
    sfFloatRect rect = {0, 0, 0, 0};
    CSFML_CHECK_RETURN(text, rect);

    sf::FloatRect sfmlRect = text->This.getGlobalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}
