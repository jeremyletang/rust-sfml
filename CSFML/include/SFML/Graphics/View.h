
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

#ifndef SFML_VIEW_H
#define SFML_VIEW_H

// Headers

#include <SFML/Graphics/Rect.h>
#include <SFML/Graphics/Types.h>
#include <SFML/System/Vector2.h>

extern "C" sfView *sfView_create(void);

extern "C" sfView *sfView_createFromRect(sfFloatRect rectangle);

extern "C" sfView *sfView_copy(const sfView *view);

extern "C" void sfView_destroy(sfView *view);

extern "C" void sfView_setCenter(sfView *view, sfVector2f center);

extern "C" void sfView_setSize(sfView *view, sfVector2f size);

extern "C" void sfView_setRotation(sfView *view, float angle);

extern "C" void sfView_setViewport(sfView *view, sfFloatRect viewport);

extern "C" void sfView_reset(sfView *view, sfFloatRect rectangle);

extern "C" sfVector2f sfView_getCenter(const sfView *view);

extern "C" sfVector2f sfView_getSize(const sfView *view);

extern "C" float sfView_getRotation(const sfView *view);

extern "C" sfFloatRect sfView_getViewport(const sfView *view);

extern "C" void sfView_move(sfView *view, sfVector2f offset);

extern "C" void sfView_rotate(sfView *view, float angle);

extern "C" void sfView_zoom(sfView *view, float factor);

#endif // SFML_VIEW_H
