
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

#ifndef SFML_VERTEXARRAY_H
#define SFML_VERTEXARRAY_H

// Headers

#include <SFML/Graphics/PrimitiveType.h>
#include <SFML/Graphics/Rect.h>
#include <SFML/Graphics/Types.h>
#include <SFML/Graphics/Vertex.h>
#include <stddef.h>

extern "C" sfVertexArray *sfVertexArray_create(void);

extern "C" sfVertexArray *sfVertexArray_copy(const sfVertexArray *vertexArray);

extern "C" void sfVertexArray_destroy(sfVertexArray *vertexArray);

extern "C" size_t sfVertexArray_getVertexCount(const sfVertexArray *vertexArray);

extern "C" sfVertex *sfVertexArray_getVertex(sfVertexArray *vertexArray, size_t index);

extern "C" void sfVertexArray_clear(sfVertexArray *vertexArray);

extern "C" void sfVertexArray_resize(sfVertexArray *vertexArray, size_t vertexCount);

extern "C" void sfVertexArray_append(sfVertexArray *vertexArray, sfVertex vertex);

extern "C" void sfVertexArray_setPrimitiveType(sfVertexArray *vertexArray, sfPrimitiveType type);

extern "C" sfPrimitiveType sfVertexArray_getPrimitiveType(sfVertexArray *vertexArray);

extern "C" sfFloatRect sfVertexArray_getBounds(sfVertexArray *vertexArray);

#endif // SFML_VERTEXARRAY_H
