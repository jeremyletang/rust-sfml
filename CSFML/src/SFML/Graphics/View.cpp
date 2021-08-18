
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

#include <SFML/Graphics/View.h>
#include <SFML/Graphics/ViewStruct.h>
#include <cstddef>

sfView *sfView_create(void) {
    return new sfView;
}

sfView *sfView_createFromRect(sfFloatRect rectangle) {
    sfView *view = new sfView;
    sfView_reset(view, rectangle);

    return view;
}

sfView *sfView_copy(const sfView *view) {

    return new sfView(*view);
}

void sfView_destroy(sfView *view) {
    delete view;
}

void sfView_setCenter(sfView *view, sfVector2f center) {
    view->This.setCenter(center.x, center.y);
}

void sfView_setSize(sfView *view, sfVector2f size) {
    view->This.setSize(size.x, size.y);
}

void sfView_setRotation(sfView *view, float angle) {
    view->This.setRotation(angle);
}

void sfView_setViewport(sfView *view, sfFloatRect viewport) {
    view->This.setViewport(sf::FloatRect(viewport.left, viewport.top, viewport.width, viewport.height));
}

void sfView_reset(sfView *view, sfFloatRect rectangle) {
    view->This.reset(sf::FloatRect(rectangle.left, rectangle.top, rectangle.width, rectangle.height));
}

sfVector2f sfView_getCenter(const sfView *view) {
    sfVector2f center = {0, 0};

    sf::Vector2f sfmlCenter = view->This.getCenter();
    center.x = sfmlCenter.x;
    center.y = sfmlCenter.y;

    return center;
}

sfVector2f sfView_getSize(const sfView *view) {
    sfVector2f size = {0, 0};

    sf::Vector2f sfmlSize = view->This.getSize();
    size.x = sfmlSize.x;
    size.y = sfmlSize.y;

    return size;
}

float sfView_getRotation(const sfView *view) {
    return view->This.getRotation();
}

sfFloatRect sfView_getViewport(const sfView *view) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect SFMLRect = view->This.getViewport();
    rect.left = SFMLRect.left;
    rect.top = SFMLRect.top;
    rect.width = SFMLRect.width;
    rect.height = SFMLRect.height;

    return rect;
}

void sfView_move(sfView *view, sfVector2f offset) {
    view->This.move(offset.x, offset.y);
}

void sfView_rotate(sfView *view, float angle) {
    view->This.rotate(angle);
}

void sfView_zoom(sfView *view, float factor) {
    view->This.zoom(factor);
}
