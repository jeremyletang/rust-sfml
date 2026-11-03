////////////////////////////////////////////////////////////
/// \brief Legacy 9-float transform struct
///
/// This is unused by the Rust bindings, which use a 16-float
/// `Transform`. Do not use it with the Rust `Transform` type.
///
////////////////////////////////////////////////////////////
#include <SFML/Graphics/Transform.hpp>
typedef struct
{
    float matrix[9];
} sfTransform;

////////////////////////////////////////////////////////////
// Convert sf::Transform to sfTransform
////////////////////////////////////////////////////////////
[[nodiscard]] inline sfTransform convertTransform(const sf::Transform &transform) {
    const float *m = transform.getMatrix();
    return {m[0], m[4], m[12], m[1], m[5], m[13], m[3], m[7], m[15]};
}

////////////////////////////////////////////////////////////
// Convert sfTransform to sf::Transform
////////////////////////////////////////////////////////////
[[nodiscard]] inline sf::Transform convertTransform(const sfTransform &transform) {
    const float *m = transform.matrix;

    // clang-format off
    return {m[0], m[1], m[2],
            m[3], m[4], m[5],
            m[6], m[7], m[8]};
    // clang-format on
}
