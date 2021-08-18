
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

#include <SFML/Graphics/Font.h>
#include <SFML/Graphics/FontStruct.h>
#include <cstddef>
#include <SFML/CallbackStream.h>



sfFont* sfFont_createFromFile(const char* filename)
{
    sfFont* font = new sfFont;
    if (!font->This.loadFromFile(filename))
    {
        delete font;
        font = NULL;
    }

    return font;
}



sfFont* sfFont_createFromMemory(const void* data, size_t sizeInBytes)
{
    sfFont* font = new sfFont;
    if (!font->This.loadFromMemory(data, sizeInBytes))
    {
        delete font;
        font = NULL;
    }

    return font;
}



sfFont* sfFont_createFromStream(sfInputStream* stream)
{


    sfFont* font = new sfFont;
    font->Stream = CallbackStream(stream);
    if (!font->This.loadFromStream(font->Stream))
    {
        delete font;
        font = NULL;
    }

    return font;
}



sfFont* sfFont_copy(const sfFont* font)
{


    return new sfFont(*font);
}



void sfFont_destroy(sfFont* font)
{
    delete font;
}



sfGlyph sfFont_getGlyph(const sfFont* font, sfUint32 codePoint, unsigned int characterSize, sfBool bold, float outlineThickness)
{
    sfGlyph glyph = {0, {0, 0, 0, 0}, {0, 0, 0, 0}};


    sf::Glyph SFMLGlyph = font->This.getGlyph(codePoint, characterSize, bold == sfTrue, outlineThickness);

    glyph.advance            = SFMLGlyph.advance;
    glyph.bounds.left        = SFMLGlyph.bounds.left;
    glyph.bounds.top         = SFMLGlyph.bounds.top;
    glyph.bounds.width       = SFMLGlyph.bounds.width;
    glyph.bounds.height      = SFMLGlyph.bounds.height;
    glyph.textureRect.left   = SFMLGlyph.textureRect.left;
    glyph.textureRect.top    = SFMLGlyph.textureRect.top;
    glyph.textureRect.width  = SFMLGlyph.textureRect.width;
    glyph.textureRect.height = SFMLGlyph.textureRect.height;

    return glyph;
}



float sfFont_getKerning(const sfFont* font, sfUint32 first, sfUint32 second, unsigned int characterSize)
{
    return font->This.getKerning(first, second, characterSize);
}



float sfFont_getLineSpacing(const sfFont* font, unsigned int characterSize)
{
    return font->This.getLineSpacing(characterSize);
}



float sfFont_getUnderlinePosition(const sfFont* font, unsigned int characterSize)
{
    return font->This.getUnderlinePosition(characterSize);
}



float sfFont_getUnderlineThickness(const sfFont* font, unsigned int characterSize)
{
    return font->This.getUnderlineThickness(characterSize);
}



const sfTexture* sfFont_getTexture(sfFont* font, unsigned int characterSize)
{


    *font->Textures[characterSize].This = font->This.getTexture(characterSize);

    return &font->Textures[characterSize];
}


sfFontInfo sfFont_getInfo(const sfFont* font)
{
    sfFontInfo info = {NULL};


    info.family = font->This.getInfo().family.c_str();

    return info;
}
