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
