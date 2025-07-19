#include "Graphics/PrimitiveType.hpp"
#include "SFML/Graphics/PrimitiveType.hpp"
#include "SFML/Graphics/RenderTarget.hpp"
#include <SFML/Graphics/VertexBuffer.hpp>
#include <cstddef>

extern "C" sf::VertexBuffer *sfVertexBuffer_new() {
    return new sf::VertexBuffer;
}

extern "C" sf::VertexBuffer *sfVertexBuffer_cpy(const sf::VertexBuffer *vertexBuffer) {
    return new sf::VertexBuffer(*vertexBuffer);
}

extern "C" void sfVertexBuffer_del(sf::VertexBuffer *vertexBuffer) {
    delete vertexBuffer;
}

extern "C" bool sfVertexBuffer_create(sf::VertexBuffer *buffer, size_t vertexCount) {
    return buffer->create(vertexCount);
}

extern "C" size_t sfVertexBuffer_getVertexCount(const sf::VertexBuffer *vertexBuffer) {
    return vertexBuffer->getVertexCount();
}

extern "C" bool sfVertexBuffer_update(sf::VertexBuffer *vertexBuffer, const sf::Vertex *vertices, size_t vertexCount, unsigned int offset) {
    return vertexBuffer->update(vertices, vertexCount, offset);
}

extern "C" bool sfVertexBuffer_updateFromVertexBuffer(sf::VertexBuffer *vertexBuffer, const sf::VertexBuffer *other) {
    return vertexBuffer->update(*other);
}

extern "C" void sfVertexBuffer_swap(sf::VertexBuffer *left, sf::VertexBuffer *right) {
    left->swap(*right);
}

extern "C" unsigned int sfVertexBuffer_getNativeHandle(const sf::VertexBuffer *vertexBuffer) {
    return vertexBuffer->getNativeHandle();
}

extern "C" void sfVertexBuffer_setPrimitiveType(sf::VertexBuffer *vertexBuffer, sfPrimitiveType type) {
    vertexBuffer->setPrimitiveType(static_cast<sf::PrimitiveType>(type));
}

extern "C" sfPrimitiveType sfVertexBuffer_getPrimitiveType(const sf::VertexBuffer *vertexBuffer) {
    return static_cast<sfPrimitiveType>(vertexBuffer->getPrimitiveType());
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
