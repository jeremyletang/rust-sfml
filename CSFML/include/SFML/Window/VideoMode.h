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

#ifndef SFML_VIDEOMODE_H
#define SFML_VIDEOMODE_H

////////////////////////////////////////////////////////////
// Headers
////////////////////////////////////////////////////////////
#include <SFML/Window/Export.h>
#include <stddef.h>


////////////////////////////////////////////////////////////
/// \brief sfVideoMode defines a video mode (width, height, bpp, frequency)
///        and provides functions for getting modes supported
///        by the display device
///
////////////////////////////////////////////////////////////
typedef struct
{
    unsigned int width;        ///< Video mode width, in pixels
    unsigned int height;       ///< Video mode height, in pixels
    unsigned int bitsPerPixel; ///< Video mode pixel depth, in bits per pixels
} sfVideoMode;


////////////////////////////////////////////////////////////
/// \brief Get the current desktop video mode
///
/// \return Current desktop video mode
///
////////////////////////////////////////////////////////////
CSFML_WINDOW_API sfVideoMode sfVideoMode_getDesktopMode(void);

////////////////////////////////////////////////////////////
/// \brief Retrieve all the video modes supported in fullscreen mode
///
/// When creating a fullscreen window, the video mode is restricted
/// to be compatible with what the graphics driver and monitor
/// support. This function returns the complete list of all video
/// modes that can be used in fullscreen mode.
/// The returned array is sorted from best to worst, so that
/// the first element will always give the best mode (higher
/// width, height and bits-per-pixel).
///
/// \param count Pointer to a variable that will be filled with the number of modes in the array
///
/// \return Pointer to an array containing all the supported fullscreen modes
///
////////////////////////////////////////////////////////////
CSFML_WINDOW_API const sfVideoMode* sfVideoMode_getFullscreenModes(size_t* count);

////////////////////////////////////////////////////////////
/// \brief Tell whether or not a video mode is valid
///
/// The validity of video modes is only relevant when using
/// fullscreen windows; otherwise any video mode can be used
/// with no restriction.
///
/// \param mode Video mode
///
/// \return sfTrue if the video mode is valid for fullscreen mode
///
////////////////////////////////////////////////////////////
CSFML_WINDOW_API sfBool sfVideoMode_isValid(sfVideoMode mode);


#endif // SFML_VIDEOMODE_H
