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

////////////////////////////////////////////////////////////
// Headers
////////////////////////////////////////////////////////////
#include <SFML/Graphics/VertexBuffer.h>
#include <SFML/Graphics/VertexBufferStruct.h>
#include <SFML/Internal.h>


////////////////////////////////////////////////////////////
sfVertexBuffer* sfVertexBuffer_create(unsigned int vertexCount, sfPrimitiveType type, sfVertexBufferUsage usage)
{
    sfVertexBuffer* buffer = new sfVertexBuffer;

    if (!buffer->This.create(vertexCount))
    {
        delete buffer;
        buffer = NULL;
    }
    else
    {
        buffer->This.setPrimitiveType(static_cast<sf::PrimitiveType>(type));
        buffer->This.setUsage(static_cast<sf::VertexBuffer::Usage>(usage));
    }

    return buffer;
}


////////////////////////////////////////////////////////////
sfVertexBuffer* sfVertexBuffer_copy(const sfVertexBuffer* vertexBuffer)
{
    sfVertexBuffer* buffer = new sfVertexBuffer;
    buffer->This = sf::VertexBuffer( vertexBuffer->This );
    return buffer;
}


////////////////////////////////////////////////////////////
void sfVertexBuffer_destroy(sfVertexBuffer* vertexBuffer)
{
    delete vertexBuffer;
}


////////////////////////////////////////////////////////////
unsigned int sfVertexBuffer_getVertexCount(const sfVertexBuffer* vertexBuffer)
{
    CSFML_CALL_RETURN(vertexBuffer, getVertexCount(), 0);
}


////////////////////////////////////////////////////////////
sfBool sfVertexBuffer_update(sfVertexBuffer* vertexBuffer, const sfVertex* vertices, unsigned int vertexCount, unsigned int offset)
{
    // the cast is safe, sfVertex has to be binary compatible with sf::Vertex
    CSFML_CALL_RETURN(vertexBuffer, update(reinterpret_cast<const sf::Vertex*>(vertices), vertexCount, offset), sfFalse);
}


////////////////////////////////////////////////////////////
sfBool sfVertexBuffer_updateFromVertexBuffer(sfVertexBuffer* vertexBuffer, const sfVertexBuffer* other)
{
    CSFML_CALL_RETURN(vertexBuffer, update(other->This), sfFalse);
}


///////////////////////////////////////////////////////////
void sfVertexBuffer_swap(sfVertexBuffer* left, sfVertexBuffer* right)
{
    CSFML_CHECK(right);

    CSFML_CALL(left, swap(right->This));
}


////////////////////////////////////////////////////////////
unsigned int sfVertexBuffer_getNativeHandle(sfVertexBuffer* vertexBuffer)
{
    CSFML_CALL_RETURN(vertexBuffer, getNativeHandle(), 0);
}


////////////////////////////////////////////////////////////
void sfVertexBuffer_setPrimitiveType(sfVertexBuffer* vertexBuffer, sfPrimitiveType type)
{
    CSFML_CALL(vertexBuffer, setPrimitiveType(static_cast<sf::PrimitiveType>(type)));
}


////////////////////////////////////////////////////////////
sfPrimitiveType sfVertexBuffer_getPrimitiveType(const sfVertexBuffer* vertexBuffer)
{
    CSFML_CHECK_RETURN(vertexBuffer, sfPoints);
    return static_cast<sfPrimitiveType>(vertexBuffer->This.getPrimitiveType());
}


////////////////////////////////////////////////////////////
void sfVertexBuffer_setUsage(sfVertexBuffer* vertexBuffer, sfVertexBufferUsage usage)
{
    CSFML_CALL(vertexBuffer, setUsage(static_cast<sf::VertexBuffer::Usage>(usage)));
}


////////////////////////////////////////////////////////////
sfVertexBufferUsage sfVertexBuffer_getUsage(const sfVertexBuffer* vertexBuffer)
{
    CSFML_CHECK_RETURN(vertexBuffer, sfVertexBufferStream);
    return static_cast<sfVertexBufferUsage>(vertexBuffer->This.getUsage());
}


////////////////////////////////////////////////////////////
void sfVertexBuffer_bind(const sfVertexBuffer* vertexBuffer)
{
    sf::VertexBuffer::bind(&vertexBuffer->This);
}


////////////////////////////////////////////////////////////
sfBool sfVertexBuffer_isAvailable()
{
    return sf::VertexBuffer::isAvailable() ? sfTrue : sfFalse;
}
