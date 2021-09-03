
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

#ifndef SFML_RENDERTEXTURE_H
#define SFML_RENDERTEXTURE_H

// Headers

#include <SFML/Graphics/Color.h>

#include <SFML/Graphics/PrimitiveType.h>
#include <SFML/Graphics/Rect.h>
#include <SFML/Graphics/RenderStates.h>
#include <SFML/Graphics/Types.h>
#include <SFML/Graphics/Vertex.h>
#include <SFML/System/Vector2.h>
#include <SFML/Window/Window.h>
#include <stddef.h>

extern "C" sfRenderTexture *sfRenderTexture_createWithSettings(unsigned int width, unsigned int height, const sfContextSettings *settings);

extern "C" void sfRenderTexture_destroy(sfRenderTexture *renderTexture);

extern "C" sfVector2u sfRenderTexture_getSize(const sfRenderTexture *renderTexture);

extern "C" sfBool sfRenderTexture_setActive(sfRenderTexture *renderTexture, sfBool active);

extern "C" void sfRenderTexture_display(sfRenderTexture *renderTexture);

extern "C" void sfRenderTexture_clear(sfRenderTexture *renderTexture, sfColor color);

extern "C" void sfRenderTexture_setView(sfRenderTexture *renderTexture, const sfView *view);

extern "C" const sfView *sfRenderTexture_getView(const sfRenderTexture *renderTexture);

extern "C" const sfView *sfRenderTexture_getDefaultView(const sfRenderTexture *renderTexture);

extern "C" sfIntRect sfRenderTexture_getViewport(const sfRenderTexture *renderTexture, const sfView *view);

extern "C" sfVector2f sfRenderTexture_mapPixelToCoords(const sfRenderTexture *renderTexture, sfVector2i point, const sfView *view);

extern "C" sfVector2i sfRenderTexture_mapCoordsToPixel(const sfRenderTexture *renderTexture, sfVector2f point, const sfView *view);

extern "C" void sfRenderTexture_drawSprite(sfRenderTexture *renderTexture, const sfSprite *object, const sfRenderStates *states);
extern "C" void sfRenderTexture_drawText(sfRenderTexture *renderTexture, const sfText *object, const sfRenderStates *states);
extern "C" void sfRenderTexture_drawShape(sfRenderTexture *renderTexture, const sfShape *object, const sfRenderStates *states);
extern "C" void sfRenderTexture_drawCircleShape(sfRenderTexture *renderTexture, const sfCircleShape *object, const sfRenderStates *states);
extern "C" void sfRenderTexture_drawConvexShape(sfRenderTexture *renderTexture, const sfConvexShape *object, const sfRenderStates *states);
extern "C" void sfRenderTexture_drawRectangleShape(sfRenderTexture *renderTexture, const sfRectangleShape *object, const sfRenderStates *states);
extern "C" void sfRenderTexture_drawVertexBuffer(sfRenderTexture *renderTexture, const sfVertexBuffer *object, const sfRenderStates *states);

extern "C" void sfRenderTexture_drawPrimitives(sfRenderTexture *renderTexture,
                                               const sfVertex *vertices, size_t vertexCount,
                                               sfPrimitiveType type, const sfRenderStates *states);

extern "C" void sfRenderTexture_pushGLStates(sfRenderTexture *renderTexture);

extern "C" void sfRenderTexture_popGLStates(sfRenderTexture *renderTexture);

extern "C" void sfRenderTexture_resetGLStates(sfRenderTexture *renderTexture);

extern "C" const sfTexture *sfRenderTexture_getTexture(const sfRenderTexture *renderTexture);

extern "C" unsigned int sfRenderTexture_getMaximumAntialiasingLevel();

extern "C" void sfRenderTexture_setSmooth(sfRenderTexture *renderTexture, sfBool smooth);

extern "C" sfBool sfRenderTexture_isSmooth(const sfRenderTexture *renderTexture);

extern "C" void sfRenderTexture_setRepeated(sfRenderTexture *renderTexture, sfBool repeated);

extern "C" sfBool sfRenderTexture_isRepeated(const sfRenderTexture *renderTexture);

extern "C" sfBool sfRenderTexture_generateMipmap(sfRenderTexture *renderTexture);

#endif // SFML_RENDERTEXTURE_H
