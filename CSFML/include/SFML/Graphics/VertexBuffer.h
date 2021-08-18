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

#ifndef SFML_VERTEXBUFFER_H
#define SFML_VERTEXBUFFER_H

////////////////////////////////////////////////////////////
// Headers
////////////////////////////////////////////////////////////
#include <SFML/Graphics/Export.h>
#include <SFML/Graphics/PrimitiveType.h>
#include <SFML/Graphics/Types.h>
#include <SFML/Graphics/Vertex.h>
#include <stddef.h>


////////////////////////////////////////////////////////////
/// \brief Usage specifiers
///
/// If data is going to be updated once or more every frame,
/// set the usage to sfVertexBufferStream. If data is going
/// to be set once and used for a long time without being
/// modified, set the usage to sfVertexBufferUsageStatic.
/// For everything else sfVertexBufferUsageDynamic should
/// be a good compromise.
///
////////////////////////////////////////////////////////////
typedef enum
{
    sfVertexBufferStream,  ///< Constantly changing data
    sfVertexBufferDynamic, ///< Occasionally changing data
    sfVertexBufferStatic   ///< Rarely changing data
} sfVertexBufferUsage;

////////////////////////////////////////////////////////////
/// \brief Create a new vertex buffer with a specific
/// sfPrimitiveType and usage specifier.
///
/// Creates the vertex buffer, allocating enough graphcis
/// memory to hold \p vertexCount vertices, and sets its
/// primitive type to \p type and usage to \p usage.
///
/// \param vertexCount Amount of vertices
/// \param type Type of primitive
/// \param usage Usage specifier
///
/// \return A new sfVertexBuffer object
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfVertexBuffer* sfVertexBuffer_create(unsigned int vertexCount, sfPrimitiveType type, sfVertexBufferUsage usage);

////////////////////////////////////////////////////////////
/// \brief Copy an existing vertex buffer
///
/// \param vertexBuffer Vertex buffer to copy
///
/// \return Copied object
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfVertexBuffer* sfVertexBuffer_copy(const sfVertexBuffer* vertexBuffer);

////////////////////////////////////////////////////////////
/// \brief Destroy an existing vertex buffer
///
/// \param vertexBuffer Vertex buffer to delete
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfVertexBuffer_destroy(sfVertexBuffer* vertexBuffer);

////////////////////////////////////////////////////////////
/// \brief Return the vertex count
///
/// \param vertexBuffer Vertex buffer object
///
/// \return Number of vertices in the vertex buffer
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API unsigned int sfVertexBuffer_getVertexCount(const sfVertexBuffer* vertexBuffer);

////////////////////////////////////////////////////////////
/// \brief Update a part of the buffer from an array of vertices
///
/// \p offset is specified as the number of vertices to skip
/// from the beginning of the buffer.
///
/// If \p offset is 0 and \p vertexCount is equal to the size of
/// the currently created buffer, its whole contents are replaced.
///
/// If \p offset is 0 and \p vertexCount is greater than the
/// size of the currently created buffer, a new buffer is created
/// containing the vertex data.
///
/// If \p offset is 0 and \p vertexCount is less than the size of
/// the currently created buffer, only the corresponding region
/// is updated.
///
/// If \p offset is not 0 and \p offset + \p vertexCount is greater
/// than the size of the currently created buffer, the update fails.
///
/// No additional check is performed on the size of the vertex
/// array, passing invalid arguments will lead to undefined
/// behavior.
///
/// \param vertices    Array of vertices to copy to the buffer
/// \param vertexCount Number of vertices to copy
/// \param offset      Offset in the buffer to copy to
///
/// \return sfTrue if the update was successful
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfBool sfVertexBuffer_update(sfVertexBuffer* vertexBuffer, const sfVertex* vertices, unsigned int vertexCount, unsigned int offset);

////////////////////////////////////////////////////////////
/// \brief Copy the contents of another buffer into this buffer
///
/// \param vertexBuffer Vertex buffer object
/// \param other Vertex buffer whose contents to copy into first vertex buffer
///
/// \return sfTrue if the copy was successful
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfBool sfVertexBuffer_updateFromVertexBuffer(sfVertexBuffer* vertexBuffer, const sfVertexBuffer* other);

////////////////////////////////////////////////////////////
/// \brief Swap the contents of this vertex buffer with those of another
///
/// \param left Instance to swap
/// \param right Instance to swap with
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfVertexBuffer_swap(sfVertexBuffer* left, sfVertexBuffer* right);

////////////////////////////////////////////////////////////
/// \brief Get the underlying OpenGL handle of the vertex buffer.
///
/// You shouldn't need to use this function, unless you have
/// very specific stuff to implement that SFML doesn't support,
/// or implement a temporary workaround until a bug is fixed.
///
/// \return OpenGL handle of the vertex buffer or 0 if not yet created
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API unsigned int sfVertexBuffer_getNativeHandle(sfVertexBuffer* vertexBuffer);

////////////////////////////////////////////////////////////
/// \brief Set the type of primitives to draw
///
/// This function defines how the vertices must be interpreted
/// when it's time to draw them.
///
/// The default primitive type is sf::Points.
///
/// \param vertexBuffer Vertex buffer object
/// \param type Type of primitive
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfVertexBuffer_setPrimitiveType(sfVertexBuffer* vertexBuffer, sfPrimitiveType type);

////////////////////////////////////////////////////////////
/// \brief Get the type of primitives drawn by the vertex buffer
///
/// \param vertexBuffer Vertex buffer object
///
/// \return Primitive type
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfPrimitiveType sfVertexBuffer_getPrimitiveType(const sfVertexBuffer* vertexBuffer);

////////////////////////////////////////////////////////////
/// \brief Set the usage specifier of this vertex buffer
///
/// This function provides a hint about how this vertex buffer is
/// going to be used in terms of data update frequency.
///
/// After changing the usage specifier, the vertex buffer has
/// to be updated with new data for the usage specifier to
/// take effect.
///
/// The default primitive type is sfVertexBufferStream.
///
/// \param vertexBuffer Vertex buffer object
/// \param usage Usage specifier
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfVertexBuffer_setUsage(sfVertexBuffer* vertexBuffer, sfVertexBufferUsage usage);

////////////////////////////////////////////////////////////
/// \brief Get the usage specifier of this vertex buffer
///
/// \param vertexBuffer Vertex buffer object
///
/// \return Usage specifier
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfVertexBufferUsage sfVertexBuffer_getUsage(const sfVertexBuffer* vertexBuffer);

////////////////////////////////////////////////////////////
/// \brief Bind a vertex buffer for rendering
///
/// This function is not part of the graphics API, it mustn't be
/// used when drawing SFML entities. It must be used only if you
/// mix sfVertexBuffer with OpenGL code.
///
/// \code
/// sfVertexBuffer* vb1, vb2;
/// ...
/// sfVertexBuffer_bind(vb1);
/// // draw OpenGL stuff that use vb1...
/// sfVertexBuffer_bind(vb2);
/// // draw OpenGL stuff that use vb2...
/// sfVertexBuffer_bind(NULL);
/// // draw OpenGL stuff that use no vertex buffer...
/// \endcode
///
/// \param vertexBuffer Pointer to the vertex buffer to bind, can be null to use no vertex buffer
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfVertexBuffer_bind(const sfVertexBuffer* vertexBuffer);

////////////////////////////////////////////////////////////
/// \brief Tell whether or not the system supports vertex buffers
///
/// This function should always be called before using
/// the vertex buffer features. If it returns false, then
/// any attempt to use sf::VertexBuffer will fail.
///
/// \return True if vertex buffers are supported, false otherwise
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfBool sfVertexBuffer_isAvailable();


#endif // SFML_VERTEXBUFFER_H
