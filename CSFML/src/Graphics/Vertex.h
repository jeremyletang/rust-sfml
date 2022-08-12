#ifndef SFML_VERTEX_H
#define SFML_VERTEX_H

#include "Graphics/Color.h"

#include "System/Vector2.h"

typedef struct
{
    sfVector2f position;  ///< Position of the vertex
    sfColor color;        ///< Color of the vertex
    sfVector2f texCoords; ///< Coordinates of the texture's pixel to map to the vertex
} sfVertex;

#endif // SFML_VERTEX_H
