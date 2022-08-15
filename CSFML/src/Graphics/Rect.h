#ifndef SFML_RECT_H
#define SFML_RECT_H

typedef struct
{
    float left;
    float top;
    float width;
    float height;
} sfFloatRect;

typedef struct
{
    int left;
    int top;
    int width;
    int height;
} sfIntRect;

#endif // SFML_RECT_H
