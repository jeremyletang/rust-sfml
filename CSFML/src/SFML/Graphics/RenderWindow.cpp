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
#include <SFML/Graphics/RenderWindow.h>
#include <SFML/Graphics/RenderWindowStruct.h>
#include <SFML/Graphics/ImageStruct.h>
#include <SFML/Graphics/SpriteStruct.h>
#include <SFML/Graphics/TextStruct.h>
#include <SFML/Graphics/ShapeStruct.h>
#include <SFML/Graphics/CircleShapeStruct.h>
#include <SFML/Graphics/ConvexShapeStruct.h>
#include <SFML/Graphics/RectangleShapeStruct.h>
#include <SFML/Graphics/VertexArrayStruct.h>
#include <SFML/Graphics/VertexBufferStruct.h>
#include <SFML/Graphics/ConvertRenderStates.hpp>
#include <SFML/Window/Touch.hpp>
#include <SFML/Internal.h>
#include <SFML/Window/ContextSettingsInternal.h>
#include <SFML/Window/CursorStruct.h>
#include <SFML/ConvertEvent.h>


////////////////////////////////////////////////////////////
sfRenderWindow* sfRenderWindow_create(sfVideoMode mode, const char* title, sfUint32 style, const sfContextSettings* settings)
{
    // Convert video mode
    sf::VideoMode videoMode(mode.width, mode.height, mode.bitsPerPixel);

    // Convert context settings
    sf::ContextSettings params;
    if (settings)
    {
        priv::sfContextSettings_writeToCpp(*settings, params);
    }

    // Create the window
    sfRenderWindow* renderWindow = new sfRenderWindow;
    renderWindow->This.create(videoMode, title, style, params);
    renderWindow->DefaultView.This = renderWindow->This.getDefaultView();
    renderWindow->CurrentView.This = renderWindow->This.getView();

    return renderWindow;
}

////////////////////////////////////////////////////////////
sfRenderWindow* sfRenderWindow_createUnicode(sfVideoMode mode, const sfUint32* title, sfUint32 style, const sfContextSettings* settings)
{
    // Convert video mode
    sf::VideoMode videoMode(mode.width, mode.height, mode.bitsPerPixel);

    // Convert context settings
    sf::ContextSettings params;
    if (settings)
    {
        priv::sfContextSettings_writeToCpp(*settings, params);
    }

    // Create the window
    sfRenderWindow* renderWindow = new sfRenderWindow;
    renderWindow->This.create(videoMode, title, style, params);
    renderWindow->DefaultView.This = renderWindow->This.getDefaultView();
    renderWindow->CurrentView.This = renderWindow->This.getView();

    return renderWindow;
}


////////////////////////////////////////////////////////////
sfRenderWindow* sfRenderWindow_createFromHandle(sfWindowHandle handle, const sfContextSettings* settings)
{
    // Convert context settings
    sf::ContextSettings params;
    if (settings)
    {
        priv::sfContextSettings_writeToCpp(*settings, params);
    }

    // Create the window
    sfRenderWindow* renderWindow = new sfRenderWindow;
    renderWindow->This.create(handle, params);
    renderWindow->DefaultView.This = renderWindow->This.getDefaultView();
    renderWindow->CurrentView.This = renderWindow->This.getView();

    return renderWindow;
}


////////////////////////////////////////////////////////////
void sfRenderWindow_destroy(sfRenderWindow* renderWindow)
{
    delete renderWindow;
}


////////////////////////////////////////////////////////////
void sfRenderWindow_close(sfRenderWindow* renderWindow)
{
    CSFML_CALL(renderWindow, close());
}


////////////////////////////////////////////////////////////
sfBool sfRenderWindow_isOpen(const sfRenderWindow* renderWindow)
{
    CSFML_CALL_RETURN(renderWindow, isOpen(), sfFalse);
}


////////////////////////////////////////////////////////////
sfContextSettings sfRenderWindow_getSettings(const sfRenderWindow* renderWindow)
{
    sfContextSettings settings = priv::sfContextSettings_null();
    CSFML_CHECK_RETURN(renderWindow, settings);

    const sf::ContextSettings& params = renderWindow->This.getSettings();
    priv::sfContextSettings_readFromCpp(params, settings);

    return settings;
}


////////////////////////////////////////////////////////////
sfBool sfRenderWindow_pollEvent(sfRenderWindow* renderWindow, sfEvent* event)
{
    CSFML_CHECK_RETURN(renderWindow, sfFalse);
    CSFML_CHECK_RETURN(event,        sfFalse);

    // Get the event
    sf::Event SFMLEvent;
    sfBool ret = renderWindow->This.pollEvent(SFMLEvent);

    // No event, return
    if (!ret)
        return sfFalse;

    // Convert the sf::Event event to a sfEvent
    convertEvent(SFMLEvent, event);

    return sfTrue;
}


////////////////////////////////////////////////////////////
sfBool sfRenderWindow_waitEvent(sfRenderWindow* renderWindow, sfEvent* event)
{
    CSFML_CHECK_RETURN(renderWindow, sfFalse);
    CSFML_CHECK_RETURN(event,        sfFalse);

    // Get the event
    sf::Event SFMLEvent;
    sfBool ret = renderWindow->This.waitEvent(SFMLEvent);

    // Error, return
    if (!ret)
        return sfFalse;

    // Convert the sf::Event event to a sfEvent
    convertEvent(SFMLEvent, event);

    return sfTrue;
}


////////////////////////////////////////////////////////////
sfVector2i sfRenderWindow_getPosition(const sfRenderWindow* renderWindow)
{
    sfVector2i position = {0, 0};
    CSFML_CHECK_RETURN(renderWindow, position);

    sf::Vector2i sfmlPos = renderWindow->This.getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}


////////////////////////////////////////////////////////////
void sfRenderWindow_setPosition(sfRenderWindow* renderWindow, sfVector2i position)
{
    CSFML_CALL(renderWindow, setPosition(sf::Vector2i(position.x, position.y)));
}


////////////////////////////////////////////////////////////
sfVector2u sfRenderWindow_getSize(const sfRenderWindow* renderWindow)
{
    sfVector2u size = {0, 0};
    CSFML_CHECK_RETURN(renderWindow, size);

    sf::Vector2u sfmlSize = renderWindow->This.getSize();
    size.x = sfmlSize.x;
    size.y = sfmlSize.y;

    return size;
}


////////////////////////////////////////////////////////////
void sfRenderWindow_setSize(sfRenderWindow* renderWindow, sfVector2u size)
{
    CSFML_CALL(renderWindow, setSize(sf::Vector2u(size.x, size.y)));
}


////////////////////////////////////////////////////////////
void sfRenderWindow_setTitle(sfRenderWindow* renderWindow, const char* title)
{
    CSFML_CALL(renderWindow, setTitle(title));
}


////////////////////////////////////////////////////////////
void sfRenderWindow_setUnicodeTitle(sfRenderWindow* renderWindow, const sfUint32* title)
{
    CSFML_CALL(renderWindow, setTitle(title));
}


////////////////////////////////////////////////////////////
void sfRenderWindow_setIcon(sfRenderWindow* renderWindow, unsigned int width, unsigned int height, const sfUint8* pixels)
{
    CSFML_CALL(renderWindow, setIcon(width, height, pixels));
}


////////////////////////////////////////////////////////////
void sfRenderWindow_setVisible(sfRenderWindow* renderWindow, sfBool visible)
{
    CSFML_CALL(renderWindow, setVisible(visible == sfTrue));
}


////////////////////////////////////////////////////////////
void sfRenderWindow_setVerticalSyncEnabled(sfRenderWindow* renderWindow, sfBool enabled)
{
    CSFML_CALL(renderWindow, setVerticalSyncEnabled(enabled == sfTrue));
}


////////////////////////////////////////////////////////////
void sfRenderWindow_setMouseCursorVisible(sfRenderWindow* renderWindow, sfBool visible)
{
    CSFML_CALL(renderWindow, setMouseCursorVisible(visible == sfTrue));
}


////////////////////////////////////////////////////////////
void sfRenderWindow_setMouseCursorGrabbed(sfRenderWindow* renderWindow, sfBool grabbed)
{
    CSFML_CALL(renderWindow, setMouseCursorGrabbed(grabbed == sfTrue));
}


////////////////////////////////////////////////////////////
void sfRenderWindow_setMouseCursor(sfRenderWindow* window, const sfCursor* cursor)
{
    CSFML_CHECK(cursor);

    CSFML_CALL(window, setMouseCursor(cursor->This));
}


////////////////////////////////////////////////////////////
void sfRenderWindow_setKeyRepeatEnabled(sfRenderWindow* renderWindow, sfBool enabled)
{
    CSFML_CALL(renderWindow, setKeyRepeatEnabled(enabled == sfTrue));
}


////////////////////////////////////////////////////////////
sfBool sfRenderWindow_setActive(sfRenderWindow* renderWindow, sfBool active)
{
    CSFML_CALL_RETURN(renderWindow, setActive(active == sfTrue), sfFalse);
}


////////////////////////////////////////////////////////////
void sfRenderWindow_requestFocus(sfRenderWindow* renderWindow)
{
    CSFML_CALL(renderWindow, requestFocus());
}


////////////////////////////////////////////////////////////
sfBool sfRenderWindow_hasFocus(const sfRenderWindow* renderWindow)
{
    CSFML_CALL_RETURN(renderWindow, hasFocus(), sfFalse);
}


////////////////////////////////////////////////////////////
void sfRenderWindow_display(sfRenderWindow* renderWindow)
{
    CSFML_CALL(renderWindow, display());
}


////////////////////////////////////////////////////////////
void sfRenderWindow_setFramerateLimit(sfRenderWindow* renderWindow, unsigned int limit)
{
    CSFML_CALL(renderWindow, setFramerateLimit(limit));
}


////////////////////////////////////////////////////////////
void sfRenderWindow_setJoystickThreshold(sfRenderWindow* renderWindow, float threshold)
{
    CSFML_CALL(renderWindow, setJoystickThreshold(threshold));
}


////////////////////////////////////////////////////////////
sfWindowHandle sfRenderWindow_getSystemHandle(const sfRenderWindow* renderWindow)
{
    CSFML_CHECK_RETURN(renderWindow, 0);

    return (sfWindowHandle)renderWindow->This.getSystemHandle();
}


////////////////////////////////////////////////////////////
void sfRenderWindow_clear(sfRenderWindow* renderWindow, sfColor color)
{
    sf::Color SFMLColor(color.r, color.g, color.b, color.a);

    CSFML_CALL(renderWindow, clear(SFMLColor));
}


////////////////////////////////////////////////////////////
void sfRenderWindow_setView(sfRenderWindow* renderWindow, const sfView* view)
{
    CSFML_CHECK(view);
    CSFML_CALL(renderWindow, setView(view->This));
    renderWindow->CurrentView.This = view->This;
}


////////////////////////////////////////////////////////////
const sfView* sfRenderWindow_getView(const sfRenderWindow* renderWindow)
{
    CSFML_CHECK_RETURN(renderWindow, NULL);

    return &renderWindow->CurrentView;
}


////////////////////////////////////////////////////////////
const sfView* sfRenderWindow_getDefaultView(const sfRenderWindow* renderWindow)
{
    CSFML_CHECK_RETURN(renderWindow, NULL);

    return &renderWindow->DefaultView;
}


////////////////////////////////////////////////////////////
sfIntRect sfRenderWindow_getViewport(const sfRenderWindow* renderWindow, const sfView* view)
{
    sfIntRect rect = {0, 0, 0, 0};
    CSFML_CHECK_RETURN(view, rect);
    CSFML_CHECK_RETURN(renderWindow, rect);

    sf::IntRect SFMLrect = renderWindow->This.getViewport(view->This);
    rect.left   = SFMLrect.left;
    rect.top    = SFMLrect.top;
    rect.width  = SFMLrect.width;
    rect.height = SFMLrect.height;

    return rect;
}


////////////////////////////////////////////////////////////
sfVector2f sfRenderWindow_mapPixelToCoords(const sfRenderWindow* renderWindow, sfVector2i point, const sfView* targetView)
{
    sfVector2f result = {0, 0};
    CSFML_CHECK_RETURN(renderWindow, result);

    sf::Vector2f sfmlPoint;
    if (targetView)
        sfmlPoint = renderWindow->This.mapPixelToCoords(sf::Vector2i(point.x, point.y), targetView->This);
    else
        sfmlPoint = renderWindow->This.mapPixelToCoords(sf::Vector2i(point.x, point.y));

    result.x = sfmlPoint.x;
    result.y = sfmlPoint.y;

    return result;
}


////////////////////////////////////////////////////////////
sfVector2i sfRenderWindow_mapCoordsToPixel(const sfRenderWindow* renderWindow, sfVector2f point, const sfView* targetView)
{
    sfVector2i result = {0, 0};
    CSFML_CHECK_RETURN(renderWindow, result);

    sf::Vector2i sfmlPoint;
    if (targetView)
        sfmlPoint = renderWindow->This.mapCoordsToPixel(sf::Vector2f(point.x, point.y), targetView->This);
    else
        sfmlPoint = renderWindow->This.mapCoordsToPixel(sf::Vector2f(point.x, point.y));

    result.x = sfmlPoint.x;
    result.y = sfmlPoint.y;

    return result;
}


////////////////////////////////////////////////////////////
void sfRenderWindow_drawSprite(sfRenderWindow* renderWindow, const sfSprite* object, const sfRenderStates* states)
{
    CSFML_CHECK(object);
    CSFML_CALL(renderWindow, draw(object->This, convertRenderStates(states)));
}
void sfRenderWindow_drawText(sfRenderWindow* renderWindow, const sfText* object, const sfRenderStates* states)
{
    CSFML_CHECK(object);
    CSFML_CALL(renderWindow, draw(object->This, convertRenderStates(states)));
}
void sfRenderWindow_drawShape(sfRenderWindow* renderWindow, const sfShape* object, const sfRenderStates* states)
{
    CSFML_CHECK(object);
    CSFML_CALL(renderWindow, draw(object->This, convertRenderStates(states)));
}
void sfRenderWindow_drawCircleShape(sfRenderWindow* renderWindow, const sfCircleShape* object, const sfRenderStates* states)
{
    CSFML_CHECK(object);
    CSFML_CALL(renderWindow, draw(object->This, convertRenderStates(states)));
}
void sfRenderWindow_drawConvexShape(sfRenderWindow* renderWindow, const sfConvexShape* object, const sfRenderStates* states)
{
    CSFML_CHECK(object);
    CSFML_CALL(renderWindow, draw(object->This, convertRenderStates(states)));
}
void sfRenderWindow_drawRectangleShape(sfRenderWindow* renderWindow, const sfRectangleShape* object, const sfRenderStates* states)
{
    CSFML_CHECK(object);
    CSFML_CALL(renderWindow, draw(object->This, convertRenderStates(states)));
}
void sfRenderWindow_drawVertexArray(sfRenderWindow* renderWindow, const sfVertexArray* object, const sfRenderStates* states)
{
    CSFML_CHECK(object);
    CSFML_CALL(renderWindow, draw(object->This, convertRenderStates(states)));
}
void sfRenderWindow_drawVertexBuffer(sfRenderWindow* renderWindow, const sfVertexBuffer* object, const sfRenderStates* states)
{
    CSFML_CHECK(object);
    CSFML_CALL(renderWindow, draw(object->This, convertRenderStates(states)));
}


////////////////////////////////////////////////////////////
void sfRenderWindow_drawPrimitives(sfRenderWindow* renderWindow,
                                   const sfVertex* vertices, size_t vertexCount,
                                   sfPrimitiveType type, const sfRenderStates* states)
{
    CSFML_CALL(renderWindow, draw(reinterpret_cast<const sf::Vertex*>(vertices), vertexCount,
               static_cast<sf::PrimitiveType>(type), convertRenderStates(states)));
}


////////////////////////////////////////////////////////////
void sfRenderWindow_pushGLStates(sfRenderWindow* renderWindow)
{
    CSFML_CALL(renderWindow, pushGLStates());
}


////////////////////////////////////////////////////////////
void sfRenderWindow_popGLStates(sfRenderWindow* renderWindow)
{
    CSFML_CALL(renderWindow, popGLStates());
}


////////////////////////////////////////////////////////////
void sfRenderWindow_resetGLStates(sfRenderWindow* renderWindow)
{
    CSFML_CALL(renderWindow, resetGLStates());
}


////////////////////////////////////////////////////////////
sfImage* sfRenderWindow_capture(const sfRenderWindow* renderWindow)
{
    CSFML_CHECK_RETURN(renderWindow, NULL);

    sfImage* image = new sfImage;
    image->This = renderWindow->This.capture();

    return image;
}


////////////////////////////////////////////////////////////
sfVector2i sfMouse_getPositionRenderWindow(const sfRenderWindow* relativeTo)
{
    sf::Vector2i sfmlPos;
    if (relativeTo)
        sfmlPos = sf::Mouse::getPosition(relativeTo->This);
    else
        sfmlPos = sf::Mouse::getPosition();

    sfVector2i position = {sfmlPos.x, sfmlPos.y};
    return position;
}


////////////////////////////////////////////////////////////
void sfMouse_setPositionRenderWindow(sfVector2i position, const sfRenderWindow* relativeTo)
{
    if (relativeTo)
        sf::Mouse::setPosition(sf::Vector2i(position.x, position.y), relativeTo->This);
    else
        sf::Mouse::setPosition(sf::Vector2i(position.x, position.y));
}

////////////////////////////////////////////////////////////
sfVector2i sfTouch_getPositionRenderWindow(unsigned int finger, const sfRenderWindow* relativeTo)
{
    sf::Vector2i sfmlPosition;

    if (relativeTo)
        sfmlPosition = sf::Touch::getPosition(finger, relativeTo->This);
    else
        sfmlPosition = sf::Touch::getPosition(finger);

    sfVector2i position = { sfmlPosition.x, sfmlPosition.y };
    return position;
}
