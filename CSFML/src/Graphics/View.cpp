
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

#include "Graphics/Rect.h"
#include "System/Vector2.h"
#include <SFML/Graphics/View.hpp>
#include <cstddef>

extern "C" sf::View *sfView_create(void) {
    return reinterpret_cast<sf::View *>(new sf::View);
}

extern "C" sf::View *sfView_createFromRect(sfFloatRect rectangle) {
    sf::View *view = new sf::View;
    view->reset(sf::FloatRect(rectangle.left, rectangle.top, rectangle.width, rectangle.height));

    return reinterpret_cast<sf::View *>(view);
}

extern "C" sf::View *sfView_copy(const sf::View *view) {
    const sf::View *src = reinterpret_cast<const sf::View *>(view);
    sf::View *newView = new sf::View(*src);
    return reinterpret_cast<sf::View *>(newView);
}

extern "C" void sfView_destroy(sf::View *view) {
    delete reinterpret_cast<sf::View *>(view);
}

extern "C" void sfView_setCenter(sf::View *view, sfVector2f center) {
    reinterpret_cast<sf::View *>(view)->setCenter(center.x, center.y);
}

extern "C" void sfView_setSize(sf::View *view, sfVector2f size) {
    reinterpret_cast<sf::View *>(view)->setSize(size.x, size.y);
}

extern "C" void sfView_setRotation(sf::View *view, float angle) {
    reinterpret_cast<sf::View *>(view)->setRotation(angle);
}

extern "C" void sfView_setViewport(sf::View *view, sfFloatRect viewport) {
    reinterpret_cast<sf::View *>(view)->setViewport(sf::FloatRect(viewport.left, viewport.top, viewport.width, viewport.height));
}

extern "C" void sfView_reset(sf::View *view, sfFloatRect rectangle) {
    reinterpret_cast<sf::View *>(view)->reset(sf::FloatRect(rectangle.left, rectangle.top, rectangle.width, rectangle.height));
}

extern "C" sfVector2f sfView_getCenter(const sf::View *view) {
    sfVector2f center = {0, 0};

    sf::Vector2f sfmlCenter = reinterpret_cast<const sf::View *>(view)->getCenter();
    center.x = sfmlCenter.x;
    center.y = sfmlCenter.y;

    return center;
}

extern "C" sfVector2f sfView_getSize(const sf::View *view) {
    sfVector2f size = {0, 0};

    sf::Vector2f sfmlSize = reinterpret_cast<const sf::View *>(view)->getSize();
    size.x = sfmlSize.x;
    size.y = sfmlSize.y;

    return size;
}

extern "C" float sfView_getRotation(const sf::View *view) {
    return reinterpret_cast<const sf::View *>(view)->getRotation();
}

extern "C" sfFloatRect sfView_getViewport(const sf::View *view) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect SFMLRect = reinterpret_cast<const sf::View *>(view)->getViewport();
    rect.left = SFMLRect.left;
    rect.top = SFMLRect.top;
    rect.width = SFMLRect.width;
    rect.height = SFMLRect.height;

    return rect;
}

extern "C" void sfView_move(sf::View *view, sfVector2f offset) {
    reinterpret_cast<sf::View *>(view)->move(offset.x, offset.y);
}

extern "C" void sfView_rotate(sf::View *view, float angle) {
    reinterpret_cast<sf::View *>(view)->rotate(angle);
}

extern "C" void sfView_zoom(sf::View *view, float factor) {
    reinterpret_cast<sf::View *>(view)->zoom(factor);
}
