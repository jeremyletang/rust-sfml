#pragma once

////////////////////////////////////////////////////////////
/// \brief Types of texture coordinates that can be used for rendering.
///
////////////////////////////////////////////////////////////
typedef enum {
    sfCoordinateTypeNormalized, ///< sfTexture coordinates in range [0 .. 1].
    sfCoordinateTypePixels      ///< sfTexture coordinates in range [0 .. size].
} sfCoordinateType;
