#ifndef SFML_GLSL_H
#define SFML_GLSL_H

#include "Graphics/Color.h"
#include "System/Vector2.h"
#include "System/Vector3.h"

// 2D vectors
typedef sfVector2f sfGlslVec2;
typedef sfVector2i sfGlslIvec2;

typedef struct
{
    bool x;
    bool y;
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
    bool x;
    bool y;
    bool z;
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
    bool x;
    bool y;
    bool z;
    bool w;
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
