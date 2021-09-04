
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

#ifndef SFML_VERTEXBUFFER_H
#define SFML_VERTEXBUFFER_H

// Headers

#include "Graphics/PrimitiveType.h"
#include "Graphics/Types.h"
#include "Graphics/Vertex.h"
#include <stddef.h>

typedef enum {
    sfVertexBufferStream,  ///< Constantly changing data
    sfVertexBufferDynamic, ///< Occasionally changing data
    sfVertexBufferStatic   ///< Rarely changing data
} sfVertexBufferUsage;

extern "C" sfVertexBuffer *sfVertexBuffer_create(unsigned int vertexCount, sfPrimitiveType type, sfVertexBufferUsage usage);

extern "C" sfVertexBuffer *sfVertexBuffer_copy(const sfVertexBuffer *vertexBuffer);

extern "C" void sfVertexBuffer_destroy(sfVertexBuffer *vertexBuffer);

extern "C" unsigned int sfVertexBuffer_getVertexCount(const sfVertexBuffer *vertexBuffer);

extern "C" sfBool sfVertexBuffer_update(sfVertexBuffer *vertexBuffer, const sfVertex *vertices, unsigned int vertexCount, unsigned int offset);

extern "C" sfBool sfVertexBuffer_updateFromVertexBuffer(sfVertexBuffer *vertexBuffer, const sfVertexBuffer *other);

extern "C" void sfVertexBuffer_swap(sfVertexBuffer *left, sfVertexBuffer *right);

extern "C" unsigned int sfVertexBuffer_getNativeHandle(sfVertexBuffer *vertexBuffer);

extern "C" void sfVertexBuffer_setPrimitiveType(sfVertexBuffer *vertexBuffer, sfPrimitiveType type);

extern "C" sfPrimitiveType sfVertexBuffer_getPrimitiveType(const sfVertexBuffer *vertexBuffer);

extern "C" void sfVertexBuffer_setUsage(sfVertexBuffer *vertexBuffer, sfVertexBufferUsage usage);

extern "C" sfVertexBufferUsage sfVertexBuffer_getUsage(const sfVertexBuffer *vertexBuffer);

extern "C" void sfVertexBuffer_bind(const sfVertexBuffer *vertexBuffer);

extern "C" sfBool sfVertexBuffer_isAvailable();

#endif // SFML_VERTEXBUFFER_H
