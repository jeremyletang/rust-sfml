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

#ifndef SFML_RENDERTEXTURE_H
#define SFML_RENDERTEXTURE_H

////////////////////////////////////////////////////////////
// Headers
////////////////////////////////////////////////////////////
#include <SFML/Graphics/Export.h>
#include <SFML/Graphics/Color.h>
#include <SFML/Graphics/Rect.h>
#include <SFML/Graphics/Types.h>
#include <SFML/Graphics/PrimitiveType.h>
#include <SFML/Graphics/RenderStates.h>
#include <SFML/Graphics/Vertex.h>
#include <SFML/Window/Window.h>
#include <SFML/System/Vector2.h>
#include <stddef.h>


////////////////////////////////////////////////////////////
/// \brief Construct a new render texture
///
/// \param width       Width of the render texture
/// \param height      Height of the render texture
/// \param depthBuffer Do you want a depth-buffer attached? (useful only if you're doing 3D OpenGL on the rendertexture)
///
/// \return A new sfRenderTexture object, or NULL if it failed
///
/// \deprecated
/// Use sfRenderTexture_createWithSettings instead.
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfRenderTexture* sfRenderTexture_create(unsigned int width, unsigned int height, sfBool depthBuffer);

////////////////////////////////////////////////////////////
/// \brief Construct a new render texture
///
/// \param width    Width of the render texture
/// \param height   Height of the render texture
/// \param settings Settings of the render texture
///
/// \return A new sfRenderTexture object, or NULL if it failed
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfRenderTexture* sfRenderTexture_createWithSettings(unsigned int width, unsigned int height, const sfContextSettings* settings);

////////////////////////////////////////////////////////////
/// \brief Destroy an existing render texture
///
/// \param renderTexture Render texture to destroy
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfRenderTexture_destroy(sfRenderTexture* renderTexture);

////////////////////////////////////////////////////////////
/// \brief Get the size of the rendering region of a render texture
///
/// \param renderTexture Render texture object
///
/// \return Size in pixels
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfVector2u sfRenderTexture_getSize(const sfRenderTexture* renderTexture);

////////////////////////////////////////////////////////////
/// \brief Activate or deactivate a render texture as the current target for rendering
///
/// \param renderTexture Render texture object
/// \param active        sfTrue to activate, sfFalse to deactivate
///
/// \return True if operation was successful, false otherwise
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfBool sfRenderTexture_setActive(sfRenderTexture* renderTexture, sfBool active);

////////////////////////////////////////////////////////////
/// \brief Update the contents of the target texture
///
/// \param renderTexture Render texture object
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfRenderTexture_display(sfRenderTexture* renderTexture);

////////////////////////////////////////////////////////////
/// \brief Clear the rendertexture with the given color
///
/// \param renderTexture Render texture object
/// \param color         Fill color
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfRenderTexture_clear(sfRenderTexture* renderTexture, sfColor color);

////////////////////////////////////////////////////////////
/// \brief Change the current active view of a render texture
///
/// \param renderTexture Render texture object
/// \param view          Pointer to the new view
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfRenderTexture_setView(sfRenderTexture* renderTexture, const sfView* view);

////////////////////////////////////////////////////////////
/// \brief Get the current active view of a render texture
///
/// \param renderTexture Render texture object
///
/// \return Current active view
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API const sfView* sfRenderTexture_getView(const sfRenderTexture* renderTexture);

////////////////////////////////////////////////////////////
/// \brief Get the default view of a render texture
///
/// \param renderTexture Render texture object
///
/// \return Default view of the rendertexture
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API const sfView* sfRenderTexture_getDefaultView(const sfRenderTexture* renderTexture);

////////////////////////////////////////////////////////////
/// \brief Get the viewport of a view applied to this target
///
/// \param renderTexture Render texture object
/// \param view          Target view
///
/// \return Viewport rectangle, expressed in pixels in the current target
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfIntRect sfRenderTexture_getViewport(const sfRenderTexture* renderTexture, const sfView* view);

////////////////////////////////////////////////////////////
/// \brief Convert a point from texture coordinates to world coordinates
///
/// This function finds the 2D position that matches the
/// given pixel of the render-texture. In other words, it does
/// the inverse of what the graphics card does, to find the
/// initial position of a rendered pixel.
///
/// Initially, both coordinate systems (world units and target pixels)
/// match perfectly. But if you define a custom view or resize your
/// render-texture, this assertion is not true anymore, ie. a point
/// located at (10, 50) in your render-texture may map to the point
/// (150, 75) in your 2D world -- if the view is translated by (140, 25).
///
/// This version uses a custom view for calculations, see the other
/// overload of the function if you want to use the current view of the
/// render-texture.
///
/// \param renderTexture Render texture object
/// \param point Pixel to convert
/// \param view The view to use for converting the point
///
/// \return The converted point, in "world" units
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfVector2f sfRenderTexture_mapPixelToCoords(const sfRenderTexture* renderTexture, sfVector2i point, const sfView* view);

////////////////////////////////////////////////////////////
/// \brief Convert a point from world coordinates to texture coordinates
///
/// This function finds the pixel of the render-texture that matches
/// the given 2D point. In other words, it goes through the same process
/// as the graphics card, to compute the final position of a rendered point.
///
/// Initially, both coordinate systems (world units and target pixels)
/// match perfectly. But if you define a custom view or resize your
/// render-texture, this assertion is not true anymore, ie. a point
/// located at (150, 75) in your 2D world may map to the pixel
/// (10, 50) of your render-texture -- if the view is translated by (140, 25).
///
/// This version uses a custom view for calculations, see the other
/// overload of the function if you want to use the current view of the
/// render-texture.
///
/// \param renderTexture Render texture object
/// \param point Point to convert
/// \param view The view to use for converting the point
///
/// \return The converted point, in target coordinates (pixels)
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfVector2i sfRenderTexture_mapCoordsToPixel(const sfRenderTexture* renderTexture, sfVector2f point, const sfView* view);

////////////////////////////////////////////////////////////
/// \brief Draw a drawable object to the render-target
///
/// \param renderTexture Render texture object
/// \param object        Object to draw
/// \param states        Render states to use for drawing (NULL to use the default states)
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfRenderTexture_drawSprite(sfRenderTexture* renderTexture, const sfSprite* object, const sfRenderStates* states);
CSFML_GRAPHICS_API void sfRenderTexture_drawText(sfRenderTexture* renderTexture, const sfText* object, const sfRenderStates* states);
CSFML_GRAPHICS_API void sfRenderTexture_drawShape(sfRenderTexture* renderTexture, const sfShape* object, const sfRenderStates* states);
CSFML_GRAPHICS_API void sfRenderTexture_drawCircleShape(sfRenderTexture* renderTexture, const sfCircleShape* object, const sfRenderStates* states);
CSFML_GRAPHICS_API void sfRenderTexture_drawConvexShape(sfRenderTexture* renderTexture, const sfConvexShape* object, const sfRenderStates* states);
CSFML_GRAPHICS_API void sfRenderTexture_drawRectangleShape(sfRenderTexture* renderTexture, const sfRectangleShape* object, const sfRenderStates* states);
CSFML_GRAPHICS_API void sfRenderTexture_drawVertexArray(sfRenderTexture* renderTexture, const sfVertexArray* object, const sfRenderStates* states);
CSFML_GRAPHICS_API void sfRenderTexture_drawVertexBuffer(sfRenderTexture* renderTexture, const sfVertexBuffer* object, const sfRenderStates* states);

////////////////////////////////////////////////////////////
/// \brief Draw primitives defined by an array of vertices to a render texture
///
/// \param renderTexture Render texture object
/// \param vertices      Pointer to the vertices
/// \param vertexCount   Number of vertices in the array
/// \param type          Type of primitives to draw
/// \param states        Render states to use for drawing (NULL to use the default states)
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfRenderTexture_drawPrimitives(sfRenderTexture* renderTexture,
                                                       const sfVertex* vertices, size_t vertexCount,
                                                       sfPrimitiveType type, const sfRenderStates* states);

////////////////////////////////////////////////////////////
/// \brief Save the current OpenGL render states and matrices
///
/// This function can be used when you mix SFML drawing
/// and direct OpenGL rendering. Combined with popGLStates,
/// it ensures that:
/// \li SFML's internal states are not messed up by your OpenGL code
/// \li your OpenGL states are not modified by a call to a SFML function
///
/// Note that this function is quite expensive: it saves all the
/// possible OpenGL states and matrices, even the ones you
/// don't care about. Therefore it should be used wisely.
/// It is provided for convenience, but the best results will
/// be achieved if you handle OpenGL states yourself (because
/// you know which states have really changed, and need to be
/// saved and restored). Take a look at the resetGLStates
/// function if you do so.
///
/// \param renderTexture Render texture object
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfRenderTexture_pushGLStates(sfRenderTexture* renderTexture);

////////////////////////////////////////////////////////////
/// \brief Restore the previously saved OpenGL render states and matrices
///
/// See the description of pushGLStates to get a detailed
/// description of these functions.
///
/// \param renderTexture Render texture object
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfRenderTexture_popGLStates(sfRenderTexture* renderTexture);

////////////////////////////////////////////////////////////
/// \brief Reset the internal OpenGL states so that the target is ready for drawing
///
/// This function can be used when you mix SFML drawing
/// and direct OpenGL rendering, if you choose not to use
/// pushGLStates/popGLStates. It makes sure that all OpenGL
/// states needed by SFML are set, so that subsequent sfRenderTexture_draw*()
/// calls will work as expected.
///
/// \param renderTexture Render texture object
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfRenderTexture_resetGLStates(sfRenderTexture* renderTexture);

////////////////////////////////////////////////////////////
/// \brief Get the target texture of a render texture
///
/// \param renderTexture Render texture object
///
/// \return Pointer to the target texture
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API const sfTexture* sfRenderTexture_getTexture(const sfRenderTexture* renderTexture);


////////////////////////////////////////////////////////////
/// \brief Get the maximum anti-aliasing level supported by the system
///
/// \return The maximum anti-aliasing level supported by the system
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API unsigned int sfRenderTexture_getMaximumAntialiasingLevel();

////////////////////////////////////////////////////////////
/// \brief Enable or disable the smooth filter on a render texture
///
/// \param renderTexture Render texture object
/// \param smooth        sfTrue to enable smoothing, sfFalse to disable it
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfRenderTexture_setSmooth(sfRenderTexture* renderTexture, sfBool smooth);

////////////////////////////////////////////////////////////
/// \brief Tell whether the smooth filter is enabled or not for a render texture
///
/// \param renderTexture Render texture object
///
/// \return sfTrue if smoothing is enabled, sfFalse if it is disabled
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfBool sfRenderTexture_isSmooth(const sfRenderTexture* renderTexture);

////////////////////////////////////////////////////////////
/// \brief Enable or disable texture repeating
///
/// \param renderTexture Render texture object
/// \param repeated      sfTrue to enable repeating, sfFalse to disable it
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfRenderTexture_setRepeated(sfRenderTexture* renderTexture, sfBool repeated);

////////////////////////////////////////////////////////////
/// \brief Tell whether the texture is repeated or not
///
/// \param renderTexture Render texture object
///
/// \return sfTrue if repeat mode is enabled, sfFalse if it is disabled
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfBool sfRenderTexture_isRepeated(const sfRenderTexture* renderTexture);

////////////////////////////////////////////////////////////
/// \brief Generate a mipmap using the current texture data
///
/// This function is similar to sfTexture_generateMipmap and operates
/// on the texture used as the target for drawing.
/// Be aware that any draw operation may modify the base level image data.
/// For this reason, calling this function only makes sense after all
/// drawing is completed and display has been called. Not calling display
/// after subsequent drawing will lead to undefined behavior if a mipmap
/// had been previously generated.
///
/// \return sfTrue if mipmap generation was successful, sfFalse if unsuccessful
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfBool sfRenderTexture_generateMipmap(sfRenderTexture* renderTexture);


#endif // SFML_RENDERTEXTURE_H
