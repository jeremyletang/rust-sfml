
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
    return new sf::View;
}

extern "C" sf::View *sfView_createFromRect(sfFloatRect rectangle) {
    sf::View *view = new sf::View;
    view->reset(sf::FloatRect(rectangle.left, rectangle.top, rectangle.width, rectangle.height));

    return view;
}

extern "C" sf::View *sfView_copy(const sf::View *view) {
    return new sf::View(*view);
}

extern "C" void sfView_destroy(sf::View *view) {
    delete view;
}

extern "C" void sfView_setCenter(sf::View *view, sfVector2f center) {
    view->setCenter(center.x, center.y);
}

extern "C" void sfView_setSize(sf::View *view, sfVector2f size) {
    view->setSize(size.x, size.y);
}

extern "C" void sfView_setRotation(sf::View *view, float angle) {
    view->setRotation(angle);
}

extern "C" void sfView_setViewport(sf::View *view, sfFloatRect viewport) {
    view->setViewport(sf::FloatRect(viewport.left, viewport.top, viewport.width, viewport.height));
}

extern "C" void sfView_reset(sf::View *view, sfFloatRect rectangle) {
    view->reset(sf::FloatRect(rectangle.left, rectangle.top, rectangle.width, rectangle.height));
}

extern "C" sfVector2f sfView_getCenter(const sf::View *view) {
    sf::Vector2f vec2 = view->getCenter();
    return {vec2.x, vec2.y};
}

extern "C" sfVector2f sfView_getSize(const sf::View *view) {
    sf::Vector2f vec2 = view->getSize();
    return {vec2.x, vec2.y};
}

extern "C" float sfView_getRotation(const sf::View *view) {
    return view->getRotation();
}

extern "C" sfFloatRect sfView_getViewport(const sf::View *view) {
    sf::FloatRect rect = view->getViewport();
    return {rect.left, rect.top, rect.width, rect.height};
}

extern "C" void sfView_move(sf::View *view, sfVector2f offset) {
    view->move(offset.x, offset.y);
}

extern "C" void sfView_rotate(sf::View *view, float angle) {
    view->rotate(angle);
}

extern "C" void sfView_zoom(sf::View *view, float factor) {
    view->zoom(factor);
}
