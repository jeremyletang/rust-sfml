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

#ifndef SFML_CURSOR_H
#define SFML_CURSOR_H

////////////////////////////////////////////////////////////
// Headers
////////////////////////////////////////////////////////////
#include <SFML/System/Vector2.h>
#include <SFML/Window/Export.h>
#include <SFML/Window/Types.h>


////////////////////////////////////////////////////////////
/// \brief Enumeration of the native system cursor types
///
/// Refer to the following table to determine which cursor
/// is available on which platform.
///
///  Type                               | Linux | Mac OS X | Windows
/// ------------------------------------|:-----:|:--------:|:--------:
///  sfCursorArrow                  |  yes  |    yes   |   yes
///  sfCursorArrowWait              |  no   |    no    |   yes
///  sfCursorWait                   |  yes  |    no    |   yes
///  sfCursorText                   |  yes  |    yes   |   yes
///  sfCursorHand                   |  yes  |    yes   |   yes
///  sfCursorSizeHorizontal         |  yes  |    yes   |   yes
///  sfCursorSizeVertical           |  yes  |    yes   |   yes
///  sfCursorSizeTopLeftBottomRight |  no   |    no    |   yes
///  sfCursorSizeBottomLeftTopRight |  no   |    no    |   yes
///  sfCursorSizeAll                |  yes  |    no    |   yes
///  sfCursorCross                  |  yes  |    yes   |   yes
///  sfCursorHelp                   |  yes  |    no    |   yes
///  sfCursorNotAllowed             |  yes  |    yes   |   yes
///
////////////////////////////////////////////////////////////
typedef enum
{
    sfCursorArrow,                  ///< Arrow cursor (default)
    sfCursorArrowWait,              ///< Busy arrow cursor
    sfCursorWait,                   ///< Busy cursor
    sfCursorText,                   ///< I-beam, cursor when hovering over a field allowing text entry
    sfCursorHand,                   ///< Pointing hand cursor
    sfCursorSizeHorizontal,         ///< Horizontal double arrow cursor
    sfCursorSizeVertical,           ///< Vertical double arrow cursor
    sfCursorSizeTopLeftBottomRight, ///< Double arrow cursor going from top-left to bottom-right
    sfCursorSizeBottomLeftTopRight, ///< Double arrow cursor going from bottom-left to top-right
    sfCursorSizeAll,                ///< Combination of SizeHorizontal and SizeVertical
    sfCursorCross,                  ///< Crosshair cursor
    sfCursorHelp,                   ///< Help cursor
    sfCursorNotAllowed              ///< Action not allowed cursor
} sfCursorType;


////////////////////////////////////////////////////////////
/// \brief Create a cursor with the provided image
///
/// \a pixels must be an array of \a width by \a height pixels
/// in 32-bit RGBA format. If not, this will cause undefined behavior.
///
/// If \a pixels is null or either \a width or \a height are 0,
/// the current cursor is left unchanged and the function will
/// return false.
///
/// In addition to specifying the pixel data, you can also
/// specify the location of the hotspot of the cursor. The
/// hotspot is the pixel coordinate within the cursor image
/// which will be located exactly where the mouse pointer
/// position is. Any mouse actions that are performed will
/// return the window/screen location of the hotspot.
///
/// \warning On Unix, the pixels are mapped into a monochrome
///          bitmap: pixels with an alpha channel to 0 are
///          transparent, black if the RGB channel are close
///          to zero, and white otherwise.
///
/// \param pixels   Array of pixels of the image
/// \param size     Width and height of the image
/// \param hotspot  (x,y) location of the hotspot
/// \return A new sfCursor object
///
////////////////////////////////////////////////////////////
CSFML_WINDOW_API sfCursor* sfCursor_createFromPixels(const sfUint8* pixels, sfVector2u size, sfVector2u hotspot);


////////////////////////////////////////////////////////////
/// \brief Create a native system cursor
///
/// Refer to the list of cursor available on each system
/// (see sfCursorType) to know whether a given cursor is
/// expected to load successfully or is not supported by
/// the operating system.
///
/// \param type Native system cursor type
/// \return A new sfCursor object
///
////////////////////////////////////////////////////////////
CSFML_WINDOW_API sfCursor* sfCursor_createFromSystem(sfCursorType type);


////////////////////////////////////////////////////////////
/// \brief Destroy a cursor
///
/// \param cursor Cursor to destroy
///
////////////////////////////////////////////////////////////
CSFML_WINDOW_API void sfCursor_destroy(sfCursor* cursor);


#endif // SFML_CURSOR_H
