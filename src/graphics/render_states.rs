/*!
* Define the states used for drawing to a RenderTarget
*
* Don't WORK
*
*/

use graphics::blend_mode::*; 
use graphics::shader::*; 
use graphics::texture::*; 
use graphics::transform::*; 

#[doc(hidden)]
pub mod csfml {
    
    // use graphics::blend_mode::*; 
    use graphics::shader; 
    use graphics::texture::*; 
    use graphics::transform; 

    pub struct sfRenderStates {
        blendMode : uint,
        transform : transform::csfml::sfTransform,
        texture : *csfml::sfTexture,
        shader : *shader::csfml::sfShader
    }
}

#[doc(hidden)]
pub struct RenderStates {
    bendMode : BlendMode,
    transform : @Transform,
    texture : @Texture,
    shader : @Shader
}