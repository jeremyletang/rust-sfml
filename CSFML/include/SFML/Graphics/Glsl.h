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

#ifndef SFML_GLSL_H
#define SFML_GLSL_H

////////////////////////////////////////////////////////////
// Headers
////////////////////////////////////////////////////////////
#include <SFML/Graphics/Transform.h>
#include <SFML/Graphics/Color.h>
#include <SFML/System/Vector2.h>
#include <SFML/System/Vector3.h>

// 2D vectors
typedef sfVector2f sfGlslVec2;
typedef sfVector2i sfGlslIvec2;

typedef struct
{
    sfBool x;
    sfBool y;
} sfGlslBvec2;

// 3D vectors
typedef sfVector3f sfGlslVec3;

typedef struct
{
    int x;
    int y;
    int z;
} sfGlslIvec3;

typedef struct
{
    sfBool x;
    sfBool y;
    sfBool z;
} sfGlslBvec3;

// 4D vectors
typedef struct
{
    float x;
    float y;
    float z;
    float w;
} sfGlslVec4;

typedef struct
{
    int x;
    int y;
    int z;
    int w;
} sfGlslIvec4;

typedef struct
{
    sfBool x;
    sfBool y;
    sfBool z;
    sfBool w;
} sfGlslBvec4;

// matrices
typedef struct
{
    float array[3 * 3];
} sfGlslMat3;

typedef struct
{
    float array[4 * 4];
} sfGlslMat4;


#endif // SFML_GLSL_H
