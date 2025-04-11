#ifndef SFML_RECT_H
#define SFML_RECT_H

#include "System/Vector2.hpp"
struct sfFloatRect {
    sfVector2f position;
    sfVector2f size;
};

struct sfIntRect {
    sfVector2i position;
    sfVector2i size;
};

#endif // SFML_RECT_H
