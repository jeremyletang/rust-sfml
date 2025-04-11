#pragma once
#include <SFML/Graphics/StencilMode.hpp>

////////////////////////////////////////////////////////
/// \brief Enumeration of the stencil test comparisons that can be performed
///
/// The comparisons are mapped directly to their OpenGL equivalents,
/// specified by `glStencilFunc()`.
////////////////////////////////////////////////////////
typedef enum {
    sfStencilComparisonNever,        //!< The stencil test never passes
    sfStencilComparisonLess,         //!< The stencil test passes if the new value is less than the value in the stencil buffer
    sfStencilComparisonLessEqual,    //!< The stencil test passes if the new value is less than or equal to the value in the stencil buffer
    sfStencilComparisonGreater,      //!< The stencil test passes if the new value is greater than the value in the stencil buffer
    sfStencilComparisonGreaterEqual, //!< The stencil test passes if the new value is greater than or equal to the value in the stencil buffer
    sfStencilComparisonEqual,        //!< The stencil test passes if the new value is strictly equal to the value in the stencil buffer
    sfStencilComparisonNotEqual,     //!< The stencil test passes if the new value is strictly unequal to the value in the stencil buffer
    sfStencilComparisonAlways        //!< The stencil test always passes
} sfStencilComparison;

////////////////////////////////////////////////////////
/// \brief Enumeration of the stencil buffer update operations
///
/// The update operations are mapped directly to their OpenGL equivalents,
/// specified by `glStencilOp()`.
////////////////////////////////////////////////////////
typedef enum {
    sfStencilUpdateOperationKeep,      //!< If the stencil test passes, the value in the stencil buffer is not modified
    sfStencilUpdateOperationZero,      //!< If the stencil test passes, the value in the stencil buffer is set to zero
    sfStencilUpdateOperationReplace,   //!< If the stencil test passes, the value in the stencil buffer is set to the new value
    sfStencilUpdateOperationIncrement, //!< If the stencil test passes, the value in the stencil buffer is incremented and if required clamped
    sfStencilUpdateOperationDecrement, //!< If the stencil test passes, the value in the stencil buffer is decremented and if required clamped
    sfStencilUpdateOperationInvert,    //!< If the stencil test passes, the value in the stencil buffer is bitwise inverted
} sfStencilUpdateOperation;

////////////////////////////////////////////////////////
/// \brief Stencil value type (also used as a mask)
///
////////////////////////////////////////////////////////
typedef struct
{
    unsigned int value; //!< The stored stencil value
} sfStencilValue;

////////////////////////////////////////////////////////////
/// \brief Stencil modes for drawing
///
////////////////////////////////////////////////////////////
typedef struct
{
    sfStencilComparison stencilComparison;           //!< The comparison we're performing the stencil test with
    sfStencilUpdateOperation stencilUpdateOperation; //!< The update operation to perform if the stencil test passes
    sfStencilValue stencilReference;                 //!< The reference value we're performing the stencil test with
    sfStencilValue stencilMask;                      //!< The mask to apply to both the reference value and the value in the stencil buffer
    bool stencilOnly;                                //!< Whether we should update the color buffer in addition to the stencil buffer
} sfStencilMode;

////////////////////////////////////////////////////////////
// Convert sf::StencilValue to sfStencilValue
////////////////////////////////////////////////////////////
[[nodiscard]] inline sfStencilValue convertStencilValue(const sf::StencilValue value) {
    return {value.value};
}

////////////////////////////////////////////////////////////
// Convert sfStencilValue to sf::StencilValue
////////////////////////////////////////////////////////////
[[nodiscard]] inline sf::StencilValue convertStencilValue(const sfStencilValue value) {
    return {value.value};
}

////////////////////////////////////////////////////////////
// Convert sf::StencilMode to sfStencilMode
////////////////////////////////////////////////////////////
[[nodiscard]] inline sfStencilMode convertStencilMode(const sf::StencilMode value) {
    return {static_cast<sfStencilComparison>(value.stencilComparison),
            static_cast<sfStencilUpdateOperation>(value.stencilUpdateOperation),
            convertStencilValue(value.stencilReference),
            convertStencilValue(value.stencilMask),
            value.stencilOnly};
}

////////////////////////////////////////////////////////////
// Convert sfStencilMode to sf::StencilMode
////////////////////////////////////////////////////////////
[[nodiscard]] inline sf::StencilMode convertStencilMode(const sfStencilMode value) {
    return {static_cast<sf::StencilComparison>(value.stencilComparison),
            static_cast<sf::StencilUpdateOperation>(value.stencilUpdateOperation),
            convertStencilValue(value.stencilReference),
            convertStencilValue(value.stencilMask),
            value.stencilOnly};
}
