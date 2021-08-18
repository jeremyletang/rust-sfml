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
#include <SFML/Graphics/Texture.h>
#include <SFML/Graphics/TextureStruct.h>
#include <SFML/Graphics/ImageStruct.h>
#include <SFML/Graphics/RenderWindowStruct.h>
#include <SFML/Window/WindowStruct.h>
#include <SFML/Internal.h>
#include <SFML/CallbackStream.h>


////////////////////////////////////////////////////////////
sfTexture* sfTexture_create(unsigned int width, unsigned int height)
{
    sfTexture* texture = new sfTexture;

    if (!texture->This->create(width, height))
    {
        delete texture;
        texture = NULL;
    }

    return texture;
}


////////////////////////////////////////////////////////////
sfTexture* sfTexture_createFromFile(const char* filename, const sfIntRect* area)
{
    sfTexture* texture = new sfTexture;

    sf::IntRect rect;
    if (area)
        rect = sf::IntRect(area->left, area->top, area->width, area->height);

    if (!texture->This->loadFromFile(filename, rect))
    {
        delete texture;
        texture = NULL;
    }

    return texture;
}


////////////////////////////////////////////////////////////
sfTexture* sfTexture_createFromMemory(const void* data, size_t sizeInBytes, const sfIntRect* area)
{
    sfTexture* texture = new sfTexture;

    sf::IntRect rect;
    if (area)
        rect = sf::IntRect(area->left, area->top, area->width, area->height);

    if (!texture->This->loadFromMemory(data, sizeInBytes, rect))
    {
        delete texture;
        texture = NULL;
    }

    return texture;
}


////////////////////////////////////////////////////////////
sfTexture* sfTexture_createFromStream(sfInputStream* stream, const sfIntRect* area)
{
    CSFML_CHECK_RETURN(stream, NULL);

    sfTexture* texture = new sfTexture;

    sf::IntRect rect;
    if (area)
        rect = sf::IntRect(area->left, area->top, area->width, area->height);

    CallbackStream sfmlStream(stream);
    if (!texture->This->loadFromStream(sfmlStream, rect))
    {
        delete texture;
        texture = NULL;
    }

    return texture;
}


////////////////////////////////////////////////////////////
sfTexture* sfTexture_createFromImage(const sfImage* image, const sfIntRect* area)
{
    CSFML_CHECK_RETURN(image, NULL);

    sfTexture* texture = new sfTexture;

    sf::IntRect rect;
    if (area)
        rect = sf::IntRect(area->left, area->top, area->width, area->height);

    if (!texture->This->loadFromImage(image->This, rect))
    {
        delete texture;
        texture = NULL;
    }

    return texture;
}


////////////////////////////////////////////////////////////
sfTexture* sfTexture_copy(const sfTexture* texture)
{
    CSFML_CHECK_RETURN(texture, NULL);

    return new sfTexture(*texture);
}


////////////////////////////////////////////////////////////
void sfTexture_destroy(sfTexture* texture)
{
    delete texture;
}


////////////////////////////////////////////////////////////
sfVector2u sfTexture_getSize(const sfTexture* texture)
{
    sfVector2u size = {0, 0};
    CSFML_CHECK_RETURN(texture, size);

    sf::Vector2u sfmlSize = texture->This->getSize();

    size.x = sfmlSize.x;
    size.y = sfmlSize.y;

    return size;
}


////////////////////////////////////////////////////////////
sfImage* sfTexture_copyToImage(const sfTexture* texture)
{
    CSFML_CHECK_RETURN(texture, NULL);
    CSFML_CHECK_RETURN(texture->This, NULL);

    sfImage* image = new sfImage;
    image->This = texture->This->copyToImage();

    return image;
}


////////////////////////////////////////////////////////////
void sfTexture_updateFromPixels(sfTexture* texture, const sfUint8* pixels, unsigned int width, unsigned int height, unsigned int x, unsigned int y)
{
    CSFML_CALL_PTR(texture, update(pixels, width, height, x, y));
}


////////////////////////////////////////////////////////////
void sfTexture_updateFromTexture(sfTexture* destination, const sfTexture* texture, unsigned int x, unsigned int y)
{
    CSFML_CHECK(texture);
    CSFML_CHECK(texture->This);

    CSFML_CALL_PTR(destination, update(*texture->This, x, y));
}


////////////////////////////////////////////////////////////
void sfTexture_updateFromImage(sfTexture* texture, const sfImage* image, unsigned int x, unsigned int y)
{
    CSFML_CHECK(image);

    CSFML_CALL_PTR(texture, update(image->This, x, y));
}


////////////////////////////////////////////////////////////
void sfTexture_updateFromWindow(sfTexture* texture, const sfWindow* window, unsigned int x, unsigned int y)
{
    CSFML_CHECK(window);

    CSFML_CALL_PTR(texture, update(window->This, x, y));
}


////////////////////////////////////////////////////////////
void sfTexture_updateFromRenderWindow(sfTexture* texture, const sfRenderWindow* renderWindow, unsigned int x, unsigned int y)
{
    CSFML_CHECK(renderWindow);

    CSFML_CALL_PTR(texture, update(renderWindow->This, x, y));
}


////////////////////////////////////////////////////////////
void sfTexture_setSmooth(sfTexture* texture, sfBool smooth)
{
    CSFML_CALL_PTR(texture, setSmooth(smooth == sfTrue));
}


////////////////////////////////////////////////////////////
sfBool sfTexture_isSmooth(const sfTexture* texture)
{
    CSFML_CHECK_RETURN(texture, sfFalse);
    CSFML_CHECK_RETURN(texture->This, sfFalse);

    return texture->This->isSmooth();
}


////////////////////////////////////////////////////////////
void sfTexture_setSrgb(sfTexture* texture, sfBool sRgb)
{
    CSFML_CALL_PTR(texture, setSrgb(sRgb == sfTrue));
}


////////////////////////////////////////////////////////////
sfBool sfTexture_isSrgb(const sfTexture* texture)
{
    CSFML_CALL_PTR_RETURN(texture, isSrgb(), sfFalse);
}


////////////////////////////////////////////////////////////
void sfTexture_setRepeated(sfTexture* texture, sfBool repeated)
{
    CSFML_CALL_PTR(texture, setRepeated(repeated == sfTrue));
}


////////////////////////////////////////////////////////////
sfBool sfTexture_isRepeated(const sfTexture* texture)
{
    CSFML_CHECK_RETURN(texture, sfFalse);
    CSFML_CHECK_RETURN(texture->This, sfFalse);

    return texture->This->isRepeated();
}


////////////////////////////////////////////////////////////
sfBool sfTexture_generateMipmap(sfTexture* texture)
{
    CSFML_CALL_PTR_RETURN(texture, generateMipmap(), sfFalse);
}


////////////////////////////////////////////////////////////
void sfTexture_swap(sfTexture* left, sfTexture* right)
{
    CSFML_CHECK(right);

    CSFML_CALL_PTR(left, swap(*right->This));
}


////////////////////////////////////////////////////////////
unsigned int sfTexture_getNativeHandle(const sfTexture* texture)
{
    CSFML_CALL_PTR_RETURN(texture, getNativeHandle(), 0);
}


////////////////////////////////////////////////////////////
void sfTexture_bind(const sfTexture* texture)
{
    sf::Texture::bind(texture ? texture->This : NULL);
}


////////////////////////////////////////////////////////////
unsigned int sfTexture_getMaximumSize()
{
    return sf::Texture::getMaximumSize();
}
