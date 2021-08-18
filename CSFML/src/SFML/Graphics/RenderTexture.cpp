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
#include <SFML/Graphics/RenderTexture.h>
#include <SFML/Graphics/RenderTextureStruct.h>
#include <SFML/Graphics/SpriteStruct.h>
#include <SFML/Graphics/TextStruct.h>
#include <SFML/Graphics/ShapeStruct.h>
#include <SFML/Graphics/CircleShapeStruct.h>
#include <SFML/Graphics/ConvexShapeStruct.h>
#include <SFML/Graphics/RectangleShapeStruct.h>
#include <SFML/Graphics/VertexArrayStruct.h>
#include <SFML/Graphics/VertexBufferStruct.h>
#include <SFML/Graphics/ConvertRenderStates.hpp>
#include <SFML/Internal.h>
#include <SFML/Window/ContextSettingsInternal.h>


////////////////////////////////////////////////////////////
sfRenderTexture* sfRenderTexture_create(unsigned int width, unsigned int height, sfBool depthBuffer)
{
    sfRenderTexture* renderTexture = new sfRenderTexture;
    renderTexture->This.create(width, height, depthBuffer == sfTrue);
    renderTexture->Target = new sfTexture(const_cast<sf::Texture*>(&renderTexture->This.getTexture()));
    renderTexture->DefaultView.This = renderTexture->This.getDefaultView();
    renderTexture->CurrentView.This = renderTexture->This.getView();

    return renderTexture;
}


////////////////////////////////////////////////////////////
sfRenderTexture* sfRenderTexture_createWithSettings(unsigned int width, unsigned int height, const sfContextSettings* settings)
{
    // Convert context settings
    sf::ContextSettings params;
    if (settings)
    {
        priv::sfContextSettings_writeToCpp(*settings, params);
    }

    // Create the render texture
    sfRenderTexture* renderTexture = new sfRenderTexture;
    renderTexture->This.create(width, height, params);
    renderTexture->Target = new sfTexture(const_cast<sf::Texture*>(&renderTexture->This.getTexture()));
    renderTexture->DefaultView.This = renderTexture->This.getDefaultView();
    renderTexture->CurrentView.This = renderTexture->This.getView();

    return renderTexture;
}


////////////////////////////////////////////////////////////
void sfRenderTexture_destroy(sfRenderTexture* renderTexture)
{
    delete renderTexture->Target;
    delete renderTexture;
}


////////////////////////////////////////////////////////////
sfVector2u sfRenderTexture_getSize(const sfRenderTexture* renderTexture)
{
    sfVector2u size = {0, 0};
    CSFML_CHECK_RETURN(renderTexture, size);

    sf::Vector2u sfmlSize = renderTexture->This.getSize();
    size.x = sfmlSize.x;
    size.y = sfmlSize.y;

    return size;
}


////////////////////////////////////////////////////////////
sfBool sfRenderTexture_setActive(sfRenderTexture* renderTexture, sfBool active)
{
    CSFML_CALL_RETURN(renderTexture, setActive(active == sfTrue), sfFalse);
}


////////////////////////////////////////////////////////////
void sfRenderTexture_display(sfRenderTexture* renderTexture)
{
    CSFML_CALL(renderTexture, display());
}


////////////////////////////////////////////////////////////
void sfRenderTexture_clear(sfRenderTexture* renderTexture, sfColor color)
{
    sf::Color SFMLColor(color.r, color.g, color.b, color.a);

    CSFML_CALL(renderTexture, clear(SFMLColor));
}


////////////////////////////////////////////////////////////
void sfRenderTexture_setView(sfRenderTexture* renderTexture, const sfView* view)
{
    CSFML_CHECK(view);
    CSFML_CALL(renderTexture, setView(view->This));
    renderTexture->CurrentView.This = view->This;
}


////////////////////////////////////////////////////////////
const sfView* sfRenderTexture_getView(const sfRenderTexture* renderTexture)
{
    CSFML_CHECK_RETURN(renderTexture, NULL);

    return &renderTexture->CurrentView;
}


////////////////////////////////////////////////////////////
const sfView* sfRenderTexture_getDefaultView(const sfRenderTexture* renderTexture)
{
    CSFML_CHECK_RETURN(renderTexture, NULL);

    return &renderTexture->DefaultView;
}


////////////////////////////////////////////////////////////
sfIntRect sfRenderTexture_getViewport(const sfRenderTexture* renderTexture, const sfView* view)
{
    sfIntRect rect = {0, 0, 0, 0};
    CSFML_CHECK_RETURN(view, rect);
    CSFML_CHECK_RETURN(renderTexture, rect);

    sf::IntRect SFMLrect = renderTexture->This.getViewport(view->This);
    rect.left   = SFMLrect.left;
    rect.top    = SFMLrect.top;
    rect.width  = SFMLrect.width;
    rect.height = SFMLrect.height;

    return rect;
}


////////////////////////////////////////////////////////////
sfVector2f sfRenderTexture_mapPixelToCoords(const sfRenderTexture* renderTexture, sfVector2i point, const sfView* targetView)
{
    sfVector2f result = {0, 0};
    CSFML_CHECK_RETURN(renderTexture, result);

    sf::Vector2f sfmlPoint;
    if (targetView)
        sfmlPoint = renderTexture->This.mapPixelToCoords(sf::Vector2i(point.x, point.y), targetView->This);
    else
        sfmlPoint = renderTexture->This.mapPixelToCoords(sf::Vector2i(point.x, point.y));

    result.x = sfmlPoint.x;
    result.y = sfmlPoint.y;

    return result;
}


////////////////////////////////////////////////////////////
sfVector2i sfRenderTexture_mapCoordsToPixel(const sfRenderTexture* renderTexture, sfVector2f point, const sfView* targetView)
{
    sfVector2i result = {0, 0};
    CSFML_CHECK_RETURN(renderTexture, result);

    sf::Vector2i sfmlPoint;
    if (targetView)
        sfmlPoint = renderTexture->This.mapCoordsToPixel(sf::Vector2f(point.x, point.y), targetView->This);
    else
        sfmlPoint = renderTexture->This.mapCoordsToPixel(sf::Vector2f(point.x, point.y));

    result.x = sfmlPoint.x;
    result.y = sfmlPoint.y;

    return result;
}


////////////////////////////////////////////////////////////
void sfRenderTexture_drawSprite(sfRenderTexture* renderTexture, const sfSprite* object, const sfRenderStates* states)
{
    CSFML_CHECK(object);
    CSFML_CALL(renderTexture, draw(object->This, convertRenderStates(states)));
}
void sfRenderTexture_drawText(sfRenderTexture* renderTexture, const sfText* object, const sfRenderStates* states)
{
    CSFML_CHECK(object);
    CSFML_CALL(renderTexture, draw(object->This, convertRenderStates(states)));
}
void sfRenderTexture_drawShape(sfRenderTexture* renderTexture, const sfShape* object, const sfRenderStates* states)
{
    CSFML_CHECK(object);
    CSFML_CALL(renderTexture, draw(object->This, convertRenderStates(states)));
}
void sfRenderTexture_drawCircleShape(sfRenderTexture* renderTexture, const sfCircleShape* object, const sfRenderStates* states)
{
    CSFML_CHECK(object);
    CSFML_CALL(renderTexture, draw(object->This, convertRenderStates(states)));
}
void sfRenderTexture_drawConvexShape(sfRenderTexture* renderTexture, const sfConvexShape* object, const sfRenderStates* states)
{
    CSFML_CHECK(object);
    CSFML_CALL(renderTexture, draw(object->This, convertRenderStates(states)));
}
void sfRenderTexture_drawRectangleShape(sfRenderTexture* renderTexture, const sfRectangleShape* object, const sfRenderStates* states)
{
    CSFML_CHECK(object);
    CSFML_CALL(renderTexture, draw(object->This, convertRenderStates(states)));
}
void sfRenderTexture_drawVertexArray(sfRenderTexture* renderTexture, const sfVertexArray* object, const sfRenderStates* states)
{
    CSFML_CHECK(object);
    CSFML_CALL(renderTexture, draw(object->This, convertRenderStates(states)));
}
void sfRenderTexture_drawVertexBuffer(sfRenderTexture* renderTexture, const sfVertexBuffer* object, const sfRenderStates* states)
{
    CSFML_CHECK(object);
    CSFML_CALL(renderTexture, draw(object->This, convertRenderStates(states)));
}


////////////////////////////////////////////////////////////
void sfRenderTexture_drawPrimitives(sfRenderTexture* renderTexture,
                                    const sfVertex* vertices, size_t vertexCount,
                                    sfPrimitiveType type, const sfRenderStates* states)
{
    CSFML_CALL(renderTexture, draw(reinterpret_cast<const sf::Vertex*>(vertices), vertexCount,
               static_cast<sf::PrimitiveType>(type), convertRenderStates(states)));
}


////////////////////////////////////////////////////////////
void sfRenderTexture_pushGLStates(sfRenderTexture* renderTexture)
{
    CSFML_CALL(renderTexture, pushGLStates());
}


////////////////////////////////////////////////////////////
void sfRenderTexture_popGLStates(sfRenderTexture* renderTexture)
{
    CSFML_CALL(renderTexture, popGLStates());
}


////////////////////////////////////////////////////////////
void sfRenderTexture_resetGLStates(sfRenderTexture* renderTexture)
{
    CSFML_CALL(renderTexture, resetGLStates());
}


////////////////////////////////////////////////////////////
const sfTexture* sfRenderTexture_getTexture(const sfRenderTexture* renderTexture)
{
    CSFML_CHECK_RETURN(renderTexture, NULL);

    return renderTexture->Target;
}


////////////////////////////////////////////////////////////
void sfRenderTexture_setSmooth(sfRenderTexture* renderTexture, sfBool smooth)
{
    CSFML_CALL(renderTexture, setSmooth(smooth == sfTrue));
}


////////////////////////////////////////////////////////////
unsigned int sfRenderTexture_getMaximumAntialiasingLevel()
{
    return sf::RenderTexture::getMaximumAntialiasingLevel();
}


////////////////////////////////////////////////////////////
sfBool sfRenderTexture_isSmooth(const sfRenderTexture* renderTexture)
{
    CSFML_CALL_RETURN(renderTexture, isSmooth(), sfFalse);
}

////////////////////////////////////////////////////////////
void sfRenderTexture_setRepeated(sfRenderTexture* renderTexture, sfBool repeated)
{
    CSFML_CALL(renderTexture, setRepeated(repeated == sfTrue));
}


////////////////////////////////////////////////////////////
sfBool sfRenderTexture_isRepeated(const sfRenderTexture* renderTexture)
{
    CSFML_CALL_RETURN(renderTexture, isRepeated(), sfFalse);
}


////////////////////////////////////////////////////////////
sfBool sfRenderTexture_generateMipmap(sfRenderTexture* renderTexture)
{
    CSFML_CALL_RETURN(renderTexture, generateMipmap(), sfFalse);
}
