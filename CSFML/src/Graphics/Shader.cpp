
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

// Headers

#include "Graphics/Shader.h"
#include "System/InputStreamStruct.h"
#include <SFML/Graphics/Shader.hpp>
#include <cstddef>

sfShader *sfShader_createFromFile(const char *vertexShaderFilename, const char *geometryShaderFilename, const char *fragmentShaderFilename) {
    bool success = false;
    sf::Shader *shader = new sf::Shader;
    if (vertexShaderFilename || geometryShaderFilename || fragmentShaderFilename) {
        if (!geometryShaderFilename) {
            if (!vertexShaderFilename) {
                // fragment shader only
                success = shader->loadFromFile(fragmentShaderFilename, sf::Shader::Fragment);
            } else if (!fragmentShaderFilename) {
                // vertex shader only
                success = shader->loadFromFile(vertexShaderFilename, sf::Shader::Vertex);
            } else {
                // vertex + fragment shaders
                success = shader->loadFromFile(vertexShaderFilename, fragmentShaderFilename);
            }
        } else {
            // vertex + geometry + fragment shaders
            success = shader->loadFromFile(vertexShaderFilename, geometryShaderFilename, fragmentShaderFilename);
        }
    }

    if (!success) {
        delete shader;
        shader = NULL;
    }

    return reinterpret_cast<sfShader *>(shader);
}

sfShader *sfShader_createFromMemory(const char *vertexShader, const char *geometryShader, const char *fragmentShader) {
    bool success = false;
    sf::Shader *shader = new sf::Shader;
    if (vertexShader || geometryShader || fragmentShader) {
        if (!geometryShader) {
            if (!vertexShader) {
                // fragment shader only
                success = shader->loadFromMemory(fragmentShader, sf::Shader::Fragment);
            } else if (!fragmentShader) {
                // vertex shader only
                success = shader->loadFromMemory(vertexShader, sf::Shader::Vertex);
            } else {
                // vertex + fragment shaders
                success = shader->loadFromMemory(vertexShader, fragmentShader);
            }
        } else {
            // vertex + geometry + fragment shaders
            success = shader->loadFromMemory(vertexShader, geometryShader, fragmentShader);
        }
    }

    if (!success) {
        delete shader;
        shader = NULL;
    }

    return reinterpret_cast<sfShader *>(shader);
}

sfShader *sfShader_createFromStream(sfInputStream *vertexShaderStream, sfInputStream *geometryShaderStream, sfInputStream *fragmentShaderStream) {
    bool success = false;
    sf::Shader *shader = new sf::Shader;
    if (vertexShaderStream || geometryShaderStream || fragmentShaderStream) {
        if (!geometryShaderStream) {
            if (!vertexShaderStream) {
                // fragment shader only
                success = shader->loadFromStream(*fragmentShaderStream, sf::Shader::Fragment);
            } else if (!fragmentShaderStream) {
                // vertex shader only
                success = shader->loadFromStream(*vertexShaderStream, sf::Shader::Vertex);
            } else {
                // vertex + fragment shaders
                success = shader->loadFromStream(*vertexShaderStream, *fragmentShaderStream);
            }
        } else {
            success = shader->loadFromStream(*vertexShaderStream, *geometryShaderStream, *fragmentShaderStream);
        }
    }

    if (!success) {
        delete shader;
        shader = NULL;
    }

    return reinterpret_cast<sfShader *>(shader);
}

void sfShader_destroy(sfShader *shader) {
    delete reinterpret_cast<sf::Shader *>(shader);
}

void sfShader_setFloatUniform(sfShader *shader, const char *name, float x) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, x);
}

void sfShader_setVec2Uniform(sfShader *shader, const char *name, sfGlslVec2 vector) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Glsl::Vec2(vector.x, vector.y));
}

void sfShader_setVec3Uniform(sfShader *shader, const char *name, sfGlslVec3 vector) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Glsl::Vec3(vector.x, vector.y, vector.z));
}

void sfShader_setVec4Uniform(sfShader *shader, const char *name, sfGlslVec4 vector) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Glsl::Vec4(vector.x, vector.y, vector.z, vector.w));
}

void sfShader_setIntUniform(sfShader *shader, const char *name, int x) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, x);
}

void sfShader_setIvec2Uniform(sfShader *shader, const char *name, sfGlslIvec2 vector) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Glsl::Ivec2(vector.x, vector.y));
}

void sfShader_setIvec3Uniform(sfShader *shader, const char *name, sfGlslIvec3 vector) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Glsl::Ivec3(vector.x, vector.y, vector.z));
}

void sfShader_setIvec4Uniform(sfShader *shader, const char *name, sfGlslIvec4 vector) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Glsl::Ivec4(vector.x, vector.y, vector.z, vector.w));
}

void sfShader_setBoolUniform(sfShader *shader, const char *name, sfBool x) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, x != sfFalse);
}

void sfShader_setBvec2Uniform(sfShader *shader, const char *name, sfGlslBvec2 vector) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Glsl::Bvec2(vector.x != sfFalse, vector.y != sfFalse));
}

void sfShader_setBvec3Uniform(sfShader *shader, const char *name, sfGlslBvec3 vector) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Glsl::Bvec3(vector.x != sfFalse, vector.y != sfFalse, vector.z != sfFalse));
}

void sfShader_setBvec4Uniform(sfShader *shader, const char *name, sfGlslBvec4 vector) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Glsl::Bvec4(vector.x != sfFalse, vector.y != sfFalse, vector.z != sfFalse, vector.w != sfFalse));
}

void sfShader_setMat3Uniform(sfShader *shader, const char *name, const sfGlslMat3 *matrix) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Glsl::Mat3(matrix->array));
}

void sfShader_setMat4Uniform(sfShader *shader, const char *name, const sfGlslMat4 *matrix) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Glsl::Mat4(matrix->array));
}

void sfShader_setTextureUniform(sfShader *shader, const char *name, const sfTexture *texture) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, *reinterpret_cast<const sf::Texture *>(texture));
}

void sfShader_setCurrentTextureUniform(sfShader *shader, const char *name) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Shader::CurrentTexture);
}

void sfShader_setFloatUniformArray(sfShader *shader, const char *name, const float *scalarArray, size_t length) {
    reinterpret_cast<sf::Shader *>(shader)->setUniformArray(name, scalarArray, length);
}

void sfShader_setVec2UniformArray(sfShader *shader, const char *name, const sfGlslVec2 *vectorArray, size_t length) {
    reinterpret_cast<sf::Shader *>(shader)->setUniformArray(name, reinterpret_cast<const sf::Glsl::Vec2 *>(vectorArray), length);
}

void sfShader_setVec3UniformArray(sfShader *shader, const char *name, const sfGlslVec3 *vectorArray, size_t length) {
    reinterpret_cast<sf::Shader *>(shader)->setUniformArray(name, reinterpret_cast<const sf::Glsl::Vec3 *>(vectorArray), length);
}

void sfShader_setVec4UniformArray(sfShader *shader, const char *name, const sfGlslVec4 *vectorArray, size_t length) {
    reinterpret_cast<sf::Shader *>(shader)->setUniformArray(name, reinterpret_cast<const sf::Glsl::Vec4 *>(vectorArray), length);
}

void sfShader_setMat3UniformArray(sfShader *shader, const char *name, const sfGlslMat3 *matrixArray, size_t length) {
    reinterpret_cast<sf::Shader *>(shader)->setUniformArray(name, reinterpret_cast<const sf::Glsl::Mat3 *>(matrixArray), length);
}

void sfShader_setMat4UniformArray(sfShader *shader, const char *name, const sfGlslMat4 *matrixArray, size_t length) {
    reinterpret_cast<sf::Shader *>(shader)->setUniformArray(name, reinterpret_cast<const sf::Glsl::Mat4 *>(matrixArray), length);
}

unsigned int sfShader_getNativeHandle(const sfShader *shader) {
    return reinterpret_cast<const sf::Shader *>(shader)->getNativeHandle();
}

void sfShader_bind(const sfShader *shader) {
    sf::Shader::bind(reinterpret_cast<const sf::Shader *>(shader));
}

sfBool sfShader_isAvailable(void) {
    return sf::Shader::isAvailable() ? sfTrue : sfFalse;
}

sfBool sfShader_isGeometryAvailable(void) {
    return sf::Shader::isGeometryAvailable() ? sfTrue : sfFalse;
}
