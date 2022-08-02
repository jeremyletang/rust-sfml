
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

#include "Graphics/PrimitiveType.h"
#include "Graphics/Vertex.h"
#include "Graphics/VertexBufferStruct.h"
#include <cstddef>

typedef enum {
    sfVertexBufferStream,  ///< Constantly changing data
    sfVertexBufferDynamic, ///< Occasionally changing data
    sfVertexBufferStatic   ///< Rarely changing data
} sfVertexBufferUsage;

extern "C" sfVertexBuffer *sfVertexBuffer_create(unsigned int vertexCount, sfPrimitiveType type, sfVertexBufferUsage usage) {
    sfVertexBuffer *buffer = new sfVertexBuffer;

    if (!buffer->This.create(vertexCount)) {
        delete buffer;
        buffer = NULL;
    } else {
        buffer->This.setPrimitiveType(static_cast<sf::PrimitiveType>(type));
        buffer->This.setUsage(static_cast<sf::VertexBuffer::Usage>(usage));
    }

    return buffer;
}

extern "C" sfVertexBuffer *sfVertexBuffer_copy(const sfVertexBuffer *vertexBuffer) {
    sfVertexBuffer *buffer = new sfVertexBuffer;
    buffer->This = sf::VertexBuffer(vertexBuffer->This);
    return buffer;
}

extern "C" void sfVertexBuffer_destroy(sfVertexBuffer *vertexBuffer) {
    delete vertexBuffer;
}

extern "C" unsigned int sfVertexBuffer_getVertexCount(const sfVertexBuffer *vertexBuffer) {
    return vertexBuffer->This.getVertexCount();
}

extern "C" bool sfVertexBuffer_update(sfVertexBuffer *vertexBuffer, const sfVertex *vertices, unsigned int vertexCount, unsigned int offset) {
    // the cast is safe, sfVertex has to be binary compatible with sf::Vertex
    return vertexBuffer->This.update(reinterpret_cast<const sf::Vertex *>(vertices), vertexCount, offset);
}

extern "C" bool sfVertexBuffer_updateFromVertexBuffer(sfVertexBuffer *vertexBuffer, const sfVertexBuffer *other) {
    return vertexBuffer->This.update(other->This);
}

extern "C" void sfVertexBuffer_swap(sfVertexBuffer *left, sfVertexBuffer *right) {

    left->This.swap(right->This);
}

extern "C" unsigned int sfVertexBuffer_getNativeHandle(sfVertexBuffer *vertexBuffer) {
    return vertexBuffer->This.getNativeHandle();
}

extern "C" void sfVertexBuffer_setPrimitiveType(sfVertexBuffer *vertexBuffer, sfPrimitiveType type) {
    vertexBuffer->This.setPrimitiveType(static_cast<sf::PrimitiveType>(type));
}

extern "C" sfPrimitiveType sfVertexBuffer_getPrimitiveType(const sfVertexBuffer *vertexBuffer) {

    return static_cast<sfPrimitiveType>(vertexBuffer->This.getPrimitiveType());
}

extern "C" void sfVertexBuffer_setUsage(sfVertexBuffer *vertexBuffer, sfVertexBufferUsage usage) {
    vertexBuffer->This.setUsage(static_cast<sf::VertexBuffer::Usage>(usage));
}

extern "C" sfVertexBufferUsage sfVertexBuffer_getUsage(const sfVertexBuffer *vertexBuffer) {

    return static_cast<sfVertexBufferUsage>(vertexBuffer->This.getUsage());
}

extern "C" void sfVertexBuffer_bind(const sfVertexBuffer *vertexBuffer) {
    sf::VertexBuffer::bind(&vertexBuffer->This);
}

extern "C" bool sfVertexBuffer_isAvailable() {
    return sf::VertexBuffer::isAvailable();
}
