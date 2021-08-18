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

#ifndef SFML_FONT_H
#define SFML_FONT_H

////////////////////////////////////////////////////////////
// Headers
////////////////////////////////////////////////////////////
#include <SFML/Graphics/Export.h>
#include <SFML/Graphics/FontInfo.h>
#include <SFML/Graphics/Glyph.h>
#include <SFML/Graphics/Types.h>
#include <SFML/System/InputStream.h>
#include <stddef.h>


////////////////////////////////////////////////////////////
/// \brief Create a new font from a file
///
/// \param filename Path of the font file to load
///
/// \return A new sfFont object, or NULL if it failed
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfFont* sfFont_createFromFile(const char* filename);

////////////////////////////////////////////////////////////
/// \brief Create a new image font a file in memory
///
/// \param data        Pointer to the file data in memory
/// \param sizeInBytes Size of the data to load, in bytes
///
/// \return A new sfFont object, or NULL if it failed
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfFont* sfFont_createFromMemory(const void* data, size_t sizeInBytes);

////////////////////////////////////////////////////////////
/// \brief Create a new image font a custom stream
///
/// \param stream Source stream to read from
///
/// \return A new sfFont object, or NULL if it failed
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfFont* sfFont_createFromStream(sfInputStream* stream);

////////////////////////////////////////////////////////////
/// \brief Copy an existing font
///
/// \param font Font to copy
///
/// \return Copied object
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfFont* sfFont_copy(const sfFont* font);

////////////////////////////////////////////////////////////
/// \brief Destroy an existing font
///
/// \param font Font to delete
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfFont_destroy(sfFont* font);

////////////////////////////////////////////////////////////
/// \brief Get a glyph in a font
///
/// \param font             Source font
/// \param codePoint        Unicode code point of the character to get
/// \param characterSize    Character size, in pixels
/// \param bold             Retrieve the bold version or the regular one?
/// \param outlineThickness Thickness of outline (when != 0 the glyph will not be filled)
///
/// \return The corresponding glyph
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfGlyph sfFont_getGlyph(const sfFont* font, sfUint32 codePoint, unsigned int characterSize, sfBool bold, float outlineThickness);

////////////////////////////////////////////////////////////
/// \brief Get the kerning value corresponding to a given pair of characters in a font
///
/// \param font          Source font
/// \param first         Unicode code point of the first character
/// \param second        Unicode code point of the second character
/// \param characterSize Character size, in pixels
///
/// \return Kerning offset, in pixels
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API float sfFont_getKerning(const sfFont* font, sfUint32 first, sfUint32 second, unsigned int characterSize);

////////////////////////////////////////////////////////////
/// \brief Get the line spacing value
///
/// \param font          Source font
/// \param characterSize Character size, in pixels
///
/// \return Line spacing, in pixels
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API float sfFont_getLineSpacing(const sfFont* font, unsigned int characterSize);

////////////////////////////////////////////////////////////
/// \brief Get the position of the underline
///
/// Underline position is the vertical offset to apply between the
/// baseline and the underline.
///
/// \param font          Source font
/// \param characterSize Reference character size
///
/// \return Underline position, in pixels
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API float sfFont_getUnderlinePosition(const sfFont* font, unsigned int characterSize);

////////////////////////////////////////////////////////////
/// \brief Get the thickness of the underline
///
/// Underline thickness is the vertical size of the underline.
///
/// \param font          Source font
/// \param characterSize Reference character size
///
/// \return Underline thickness, in pixels
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API float sfFont_getUnderlineThickness(const sfFont* font, unsigned int characterSize);

////////////////////////////////////////////////////////////
/// \brief Get the texture containing the glyphs of a given size in a font
///
/// \param font          Source font
/// \param characterSize Character size, in pixels
///
/// \return Read-only pointer to the texture
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API const sfTexture* sfFont_getTexture(sfFont* font, unsigned int characterSize);

////////////////////////////////////////////////////////////
/// \brief Get the font information
///
/// The returned structure will remain valid only if the font
/// is still valid. If the font is invalid an invalid structure
/// is returned.
///
/// \param font Source font
///
/// \return A structure that holds the font information
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfFontInfo sfFont_getInfo(const sfFont* font);


#endif // SFML_FONT_H
