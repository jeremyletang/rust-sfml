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

#ifndef SFML_VIEW_H
#define SFML_VIEW_H

////////////////////////////////////////////////////////////
// Headers
////////////////////////////////////////////////////////////
#include <SFML/Graphics/Export.h>
#include <SFML/Graphics/Rect.h>
#include <SFML/Graphics/Types.h>
#include <SFML/System/Vector2.h>


////////////////////////////////////////////////////////////
/// \brief Create a default view
///
/// This function creates a default view of (0, 0, 1000, 1000)
///
/// \return A new sfView object
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfView* sfView_create(void);

////////////////////////////////////////////////////////////
/// \brief Construct a view from a rectangle
///
/// \param rectangle Rectangle defining the zone to display
///
/// \return A new sfView object
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfView* sfView_createFromRect(sfFloatRect rectangle);

////////////////////////////////////////////////////////////
/// \brief Copy an existing view
///
/// \param view View to copy
///
/// \return Copied object
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfView* sfView_copy(const sfView* view);

////////////////////////////////////////////////////////////
/// \brief Destroy an existing view
///
/// \param view View to destroy
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfView_destroy(sfView* view);

////////////////////////////////////////////////////////////
/// \brief Set the center of a view
///
/// \param view   View object
/// \param center New center
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfView_setCenter(sfView* view, sfVector2f center);

////////////////////////////////////////////////////////////
/// \brief Set the size of a view
///
/// \param view View object
/// \param size New size of the view
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfView_setSize(sfView* view, sfVector2f size);

////////////////////////////////////////////////////////////
/// \brief Set the orientation of a view
///
/// The default rotation of a view is 0 degree.
///
/// \param view  View object
/// \param angle New angle, in degrees
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfView_setRotation(sfView* view, float angle);

////////////////////////////////////////////////////////////
/// \brief Set the target viewport of a view
///
/// The viewport is the rectangle into which the contents of the
/// view are displayed, expressed as a factor (between 0 and 1)
/// of the size of the render target to which the view is applied.
/// For example, a view which takes the left side of the target would
/// be defined by a rect of (0, 0, 0.5, 1).
/// By default, a view has a viewport which covers the entire target.
///
/// \param view     View object
/// \param viewport New viewport rectangle
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfView_setViewport(sfView* view, sfFloatRect viewport);

////////////////////////////////////////////////////////////
/// \brief Reset a view to the given rectangle
///
/// Note that this function resets the rotation angle to 0.
///
/// \param view      View object
/// \param rectangle Rectangle defining the zone to display
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfView_reset(sfView* view, sfFloatRect rectangle);

////////////////////////////////////////////////////////////
/// \brief Get the center of a view
///
/// \param view View object
///
/// \return Center of the view
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfVector2f sfView_getCenter(const sfView* view);

////////////////////////////////////////////////////////////
/// \brief Get the size of a view
///
/// \param view View object
///
/// \return Size of the view
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfVector2f sfView_getSize(const sfView* view);

////////////////////////////////////////////////////////////
/// \brief Get the current orientation of a view
///
/// \param view View object
///
/// \return Rotation angle of the view, in degrees
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API float sfView_getRotation(const sfView* view);

////////////////////////////////////////////////////////////
/// \brief Get the target viewport rectangle of a view
///
/// \param view View object
///
/// \return Viewport rectangle, expressed as a factor of the target size
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfFloatRect sfView_getViewport(const sfView* view);

////////////////////////////////////////////////////////////
/// \brief Move a view relatively to its current position
///
/// \param view   View object
/// \param offset Offset
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfView_move(sfView* view, sfVector2f offset);

////////////////////////////////////////////////////////////
/// \brief Rotate a view relatively to its current orientation
///
/// \param view  View object
/// \param angle Angle to rotate, in degrees
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfView_rotate(sfView* view, float angle);

////////////////////////////////////////////////////////////
/// \brief Resize a view rectangle relatively to its current size
///
/// Resizing the view simulates a zoom, as the zone displayed on
/// screen grows or shrinks.
/// \a factor is a multiplier:
/// \li 1 keeps the size unchanged
/// \li > 1 makes the view bigger (objects appear smaller)
/// \li < 1 makes the view smaller (objects appear bigger)
///
/// \param view   View object
/// \param factor Zoom factor to apply
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfView_zoom(sfView* view, float factor);


#endif // SFML_VIEW_H
