////////////////////////////////////////////////////////////
/// \brief Define the states used for drawing to a RenderTarget
///
////////////////////////////////////////////////////////////
#include "Graphics/BlendMode.hpp"
#include "Graphics/CoordinateType.hpp"
#include "Graphics/StencilMode.hpp"
#include "Graphics/Transform.hpp"
#include <SFML/Graphics/RenderStates.hpp>
#include <SFML/Graphics/Texture.hpp>

typedef struct
{
    sfBlendMode blendMode;           ///< Blending mode
    sfStencilMode stencilMode;       //!< Stencil mode
    sfTransform transform;           ///< Transform
    sfCoordinateType coordinateType; //!< Texture coordinate type
    const sf::Texture *texture;      ///< Texture
    const sf::Shader *shader;        ///< Shader
} sfRenderStates;
