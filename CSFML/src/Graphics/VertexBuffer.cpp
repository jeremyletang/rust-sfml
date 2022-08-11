
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

#include "Graphics/Vertex.h"
#include <SFML/Graphics/VertexBuffer.hpp>
#include <cstddef>

extern "C" sf::VertexBuffer *sfVertexBuffer_create(unsigned int vertexCount, sf::PrimitiveType type, sf::VertexBuffer::Usage usage) {
    sf::VertexBuffer *buffer = new sf::VertexBuffer;

    if (!buffer->create(vertexCount)) {
        delete buffer;
        buffer = NULL;
    } else {
        buffer->setPrimitiveType(type);
        buffer->setUsage(usage);
    }

    return buffer;
}

extern "C" sf::VertexBuffer *sfVertexBuffer_copy(const sf::VertexBuffer *vertexBuffer) {
    return new sf::VertexBuffer(*vertexBuffer);
}

extern "C" void sfVertexBuffer_destroy(sf::VertexBuffer *vertexBuffer) {
    delete vertexBuffer;
}

extern "C" unsigned int sfVertexBuffer_getVertexCount(const sf::VertexBuffer *vertexBuffer) {
    return vertexBuffer->getVertexCount();
}

extern "C" bool sfVertexBuffer_update(sf::VertexBuffer *vertexBuffer, const sf::Vertex *vertices, unsigned int vertexCount, unsigned int offset) {
    // the cast is safe, sfVertex has to be binary compatible with sf::Vertex
    return vertexBuffer->update(vertices, vertexCount, offset);
}

extern "C" bool sfVertexBuffer_updateFromVertexBuffer(sf::VertexBuffer *vertexBuffer, const sf::VertexBuffer *other) {
    return vertexBuffer->update(*other);
}

extern "C" void sfVertexBuffer_swap(sf::VertexBuffer *left, sf::VertexBuffer *right) {
    left->swap(*right);
}

extern "C" unsigned int sfVertexBuffer_getNativeHandle(sf::VertexBuffer *vertexBuffer) {
    return vertexBuffer->getNativeHandle();
}

extern "C" void sfVertexBuffer_setPrimitiveType(sf::VertexBuffer *vertexBuffer, sf::PrimitiveType type) {
    vertexBuffer->setPrimitiveType(type);
}

extern "C" sf::PrimitiveType sfVertexBuffer_getPrimitiveType(const sf::VertexBuffer *vertexBuffer) {

    return vertexBuffer->getPrimitiveType();
}

extern "C" void sfVertexBuffer_setUsage(sf::VertexBuffer *vertexBuffer, sf::VertexBuffer::Usage usage) {
    vertexBuffer->setUsage(usage);
}

extern "C" sf::VertexBuffer::Usage sfVertexBuffer_getUsage(const sf::VertexBuffer *vertexBuffer) {
    return vertexBuffer->getUsage();
}

extern "C" void sfVertexBuffer_bind(const sf::VertexBuffer *vertexBuffer) {
    sf::VertexBuffer::bind(vertexBuffer);
}

extern "C" bool sfVertexBuffer_isAvailable() {
    return sf::VertexBuffer::isAvailable();
}
