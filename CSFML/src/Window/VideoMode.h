#ifndef SFML_VIDEOMODE_H
#define SFML_VIDEOMODE_H

#include <cstdint>

typedef struct
{
    unsigned int width;        ///< Video mode width, in pixels
    unsigned int height;       ///< Video mode height, in pixels
    unsigned int bitsPerPixel; ///< Video mode pixel depth, in bits per pixels
} sfVideoMode;

#endif // SFML_VIDEOMODE_H
