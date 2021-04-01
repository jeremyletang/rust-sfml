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

#ifndef SFML_TRANSFORM_H
#define SFML_TRANSFORM_H

////////////////////////////////////////////////////////////
// Headers
////////////////////////////////////////////////////////////
#include <SFML/Graphics/Export.h>
#include <SFML/Graphics/Rect.h>
#include <SFML/Graphics/Types.h>
#include <SFML/System/Vector2.h>



////////////////////////////////////////////////////////////
/// \brief Encapsulate a 3x3 transform matrix
///
////////////////////////////////////////////////////////////
typedef struct
{
    float matrix[9];
} sfTransform;


////////////////////////////////////////////////////////////
/// \brief Identity transform (does nothing)
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API const sfTransform sfTransform_Identity;

////////////////////////////////////////////////////////////
/// \brief Create a new transform from a matrix
///
/// \param a00 Element (0, 0) of the matrix
/// \param a01 Element (0, 1) of the matrix
/// \param a02 Element (0, 2) of the matrix
/// \param a10 Element (1, 0) of the matrix
/// \param a11 Element (1, 1) of the matrix
/// \param a12 Element (1, 2) of the matrix
/// \param a20 Element (2, 0) of the matrix
/// \param a21 Element (2, 1) of the matrix
/// \param a22 Element (2, 2) of the matrix
///
/// \return A new sfTransform object
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfTransform sfTransform_fromMatrix(float a00, float a01, float a02,
                                                      float a10, float a11, float a12,
                                                      float a20, float a21, float a22);

////////////////////////////////////////////////////////////
/// \brief Return the 4x4 matrix of a transform
///
/// This function fills an array of 16 floats with the transform
/// converted as a 4x4 matrix, which is directly compatible with
/// OpenGL functions.
///
/// \code
/// sfTransform transform = ...;
/// float matrix[16];
/// sfTransform_getMatrix(&transform, matrix)
/// glLoadMatrixf(matrix);
/// \endcode
///
/// \param transform Transform object
/// \param matrix Pointer to the 16-element array to fill with the matrix
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfTransform_getMatrix(const sfTransform* transform, float* matrix);

////////////////////////////////////////////////////////////
/// \brief Return the inverse of a transform
///
/// If the inverse cannot be computed, a new identity transform
/// is returned.
///
/// \param transform Transform object
/// \return The inverse matrix
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfTransform sfTransform_getInverse(const sfTransform* transform);

////////////////////////////////////////////////////////////
/// \brief Apply a transform to a 2D point
///
/// \param transform Transform object
/// \param point     Point to transform
///
/// \return Transformed point
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfVector2f sfTransform_transformPoint(const sfTransform* transform, sfVector2f point);

////////////////////////////////////////////////////////////
/// \brief Apply a transform to a rectangle
///
/// Since SFML doesn't provide support for oriented rectangles,
/// the result of this function is always an axis-aligned
/// rectangle. Which means that if the transform contains a
/// rotation, the bounding rectangle of the transformed rectangle
/// is returned.
///
/// \param transform Transform object
/// \param rectangle Rectangle to transform
///
/// \return Transformed rectangle
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfFloatRect sfTransform_transformRect(const sfTransform* transform, sfFloatRect rectangle);

////////////////////////////////////////////////////////////
/// \brief Combine two transforms
///
/// The result is a transform that is equivalent to applying
/// \a transform followed by \a other. Mathematically, it is
/// equivalent to a matrix multiplication.
///
/// \param transform Transform object
/// \param other     Transform to combine to \a transform
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfTransform_combine(sfTransform* transform, const sfTransform* other);

////////////////////////////////////////////////////////////
/// \brief Combine a transform with a translation
///
/// \param transform Transform object
/// \param x         Offset to apply on X axis
/// \param y         Offset to apply on Y axis
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfTransform_translate(sfTransform* transform, float x, float y);

////////////////////////////////////////////////////////////
/// \brief Combine the current transform with a rotation
///
/// \param transform Transform object
/// \param angle     Rotation angle, in degrees
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfTransform_rotate(sfTransform* transform, float angle);

////////////////////////////////////////////////////////////
/// \brief Combine the current transform with a rotation
///
/// The center of rotation is provided for convenience as a second
/// argument, so that you can build rotations around arbitrary points
/// more easily (and efficiently) than the usual
/// [translate(-center), rotate(angle), translate(center)].
///
/// \param transform Transform object
/// \param angle     Rotation angle, in degrees
/// \param centerX   X coordinate of the center of rotation
/// \param centerY   Y coordinate of the center of rotation
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfTransform_rotateWithCenter(sfTransform* transform, float angle, float centerX, float centerY);

////////////////////////////////////////////////////////////
/// \brief Combine the current transform with a scaling
///
/// \param transform Transform object
/// \param scaleX    Scaling factor on the X axis
/// \param scaleY    Scaling factor on the Y axis
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfTransform_scale(sfTransform* transform, float scaleX, float scaleY);

////////////////////////////////////////////////////////////
/// \brief Combine the current transform with a scaling
///
/// The center of scaling is provided for convenience as a second
/// argument, so that you can build scaling around arbitrary points
/// more easily (and efficiently) than the usual
/// [translate(-center), scale(factors), translate(center)]
///
/// \param transform Transform object
/// \param scaleX    Scaling factor on X axis
/// \param scaleY    Scaling factor on Y axis
/// \param centerX   X coordinate of the center of scaling
/// \param centerY   Y coordinate of the center of scaling
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API void sfTransform_scaleWithCenter(sfTransform* transform, float scaleX, float scaleY, float centerX, float centerY);

////////////////////////////////////////////////////////////
/// \brief Compare two transforms for equality
///
/// Performs an element-wise comparison of the elements of the
/// left transform with the elements of the right transform.
///
/// \param left Left operand (the first transform)
/// \param right Right operand (the second transform)
///
/// \return true if the transforms are equal, false otherwise
///
////////////////////////////////////////////////////////////
CSFML_GRAPHICS_API sfBool sfTransform_equal(sfTransform* left, sfTransform* right);

#endif // SFML_TRANSFORM_H
