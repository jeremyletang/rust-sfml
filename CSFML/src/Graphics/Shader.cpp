
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

#include "Config.h"
#include "Graphics/Glsl.h"
#include "System/InputStreamStruct.h"
#include <SFML/Graphics/Shader.hpp>
#include <cstddef>

extern "C" sf::Shader *sfShader_defaultConstruct() {
    return new sf::Shader;
}

extern "C" bool sfShader_loadFromMemory_1(sf::Shader *shader, const char *content, sf::Shader::Type type) {
    return shader->loadFromMemory(content, type);
}

extern "C" bool sfShader_loadFromFile_1(sf::Shader *shader, const char *filename, sf::Shader::Type type) {
    return shader->loadFromFile(filename, type);
}

extern "C" bool sfShader_loadFromStream_1(sf::Shader *shader, sfInputStream *stream, sf::Shader::Type type) {
    return shader->loadFromStream(*stream, type);
}

extern "C" bool sfShader_loadFromMemory_vert_frag(sf::Shader *shader, const char *vert, const char *frag) {
    return shader->loadFromMemory(vert, frag);
}

extern "C" bool sfShader_loadFromFile_vert_frag(sf::Shader *shader, const char *vert, const char *frag) {
    return shader->loadFromFile(vert, frag);
}

extern "C" bool sfShader_loadFromStream_vert_frag(sf::Shader *shader, sfInputStream *vert, sfInputStream *frag) {
    return shader->loadFromStream(*vert, *frag);
}

extern "C" bool sfShader_loadFromMemory_all(sf::Shader *shader, const char *vert, const char *geom, const char *frag) {
    return shader->loadFromMemory(vert, geom, frag);
}

extern "C" bool sfShader_loadFromFile_all(sf::Shader *shader, const char *vert, const char *geom, const char *frag) {
    return shader->loadFromFile(vert, geom, frag);
}

extern "C" bool sfShader_loadFromStream_all(sf::Shader *shader, sfInputStream *vert, sfInputStream *geom, sfInputStream *frag) {
    return shader->loadFromStream(*vert, *geom, *frag);
}

extern "C" void sfShader_destroy(sf::Shader *shader) {
    delete reinterpret_cast<sf::Shader *>(shader);
}

extern "C" void sfShader_setFloatUniform(sf::Shader *shader, const char *name, float x) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, x);
}

extern "C" void sfShader_setVec2Uniform(sf::Shader *shader, const char *name, sfGlslVec2 vector) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Glsl::Vec2(vector.x, vector.y));
}

extern "C" void sfShader_setVec3Uniform(sf::Shader *shader, const char *name, sfGlslVec3 vector) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Glsl::Vec3(vector.x, vector.y, vector.z));
}

extern "C" void sfShader_setVec4Uniform(sf::Shader *shader, const char *name, sfGlslVec4 vector) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Glsl::Vec4(vector.x, vector.y, vector.z, vector.w));
}

extern "C" void sfShader_setIntUniform(sf::Shader *shader, const char *name, int x) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, x);
}

extern "C" void sfShader_setIvec2Uniform(sf::Shader *shader, const char *name, sfGlslIvec2 vector) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Glsl::Ivec2(vector.x, vector.y));
}

extern "C" void sfShader_setIvec3Uniform(sf::Shader *shader, const char *name, sfGlslIvec3 vector) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Glsl::Ivec3(vector.x, vector.y, vector.z));
}

extern "C" void sfShader_setIvec4Uniform(sf::Shader *shader, const char *name, sfGlslIvec4 vector) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Glsl::Ivec4(vector.x, vector.y, vector.z, vector.w));
}

extern "C" void sfShader_setBoolUniform(sf::Shader *shader, const char *name, bool x) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, x);
}

extern "C" void sfShader_setBvec2Uniform(sf::Shader *shader, const char *name, sfGlslBvec2 vector) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Glsl::Bvec2(vector.x, vector.y));
}

extern "C" void sfShader_setBvec3Uniform(sf::Shader *shader, const char *name, sfGlslBvec3 vector) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Glsl::Bvec3(vector.x, vector.y, vector.z));
}

extern "C" void sfShader_setBvec4Uniform(sf::Shader *shader, const char *name, sfGlslBvec4 vector) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Glsl::Bvec4(vector.x, vector.y, vector.z, vector.w));
}

extern "C" void sfShader_setMat3Uniform(sf::Shader *shader, const char *name, const sfGlslMat3 *matrix) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Glsl::Mat3(matrix->array));
}

extern "C" void sfShader_setMat4Uniform(sf::Shader *shader, const char *name, const sfGlslMat4 *matrix) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Glsl::Mat4(matrix->array));
}

extern "C" void sfShader_setTextureUniform(sf::Shader *shader, const char *name, const sf::Texture *texture) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, *reinterpret_cast<const sf::Texture *>(texture));
}

extern "C" void sfShader_setCurrentTextureUniform(sf::Shader *shader, const char *name) {
    reinterpret_cast<sf::Shader *>(shader)->setUniform(name, sf::Shader::CurrentTexture);
}

extern "C" void sfShader_setFloatUniformArray(sf::Shader *shader, const char *name, const float *scalarArray, size_t length) {
    reinterpret_cast<sf::Shader *>(shader)->setUniformArray(name, scalarArray, length);
}

extern "C" void sfShader_setVec2UniformArray(sf::Shader *shader, const char *name, const sfGlslVec2 *vectorArray, size_t length) {
    reinterpret_cast<sf::Shader *>(shader)->setUniformArray(name, reinterpret_cast<const sf::Glsl::Vec2 *>(vectorArray), length);
}

extern "C" void sfShader_setVec3UniformArray(sf::Shader *shader, const char *name, const sfGlslVec3 *vectorArray, size_t length) {
    reinterpret_cast<sf::Shader *>(shader)->setUniformArray(name, reinterpret_cast<const sf::Glsl::Vec3 *>(vectorArray), length);
}

extern "C" void sfShader_setVec4UniformArray(sf::Shader *shader, const char *name, const sfGlslVec4 *vectorArray, size_t length) {
    reinterpret_cast<sf::Shader *>(shader)->setUniformArray(name, reinterpret_cast<const sf::Glsl::Vec4 *>(vectorArray), length);
}

extern "C" void sfShader_setMat3UniformArray(sf::Shader *shader, const char *name, const sfGlslMat3 *matrixArray, size_t length) {
    reinterpret_cast<sf::Shader *>(shader)->setUniformArray(name, reinterpret_cast<const sf::Glsl::Mat3 *>(matrixArray), length);
}

extern "C" void sfShader_setMat4UniformArray(sf::Shader *shader, const char *name, const sfGlslMat4 *matrixArray, size_t length) {
    reinterpret_cast<sf::Shader *>(shader)->setUniformArray(name, reinterpret_cast<const sf::Glsl::Mat4 *>(matrixArray), length);
}

extern "C" unsigned int sfShader_getNativeHandle(const sf::Shader *shader) {
    return reinterpret_cast<const sf::Shader *>(shader)->getNativeHandle();
}

extern "C" void sfShader_bind(const sf::Shader *shader) {
    sf::Shader::bind(reinterpret_cast<const sf::Shader *>(shader));
}

extern "C" bool sfShader_isAvailable(void) {
    return sf::Shader::isAvailable();
}

extern "C" bool sfShader_isGeometryAvailable(void) {
    return sf::Shader::isGeometryAvailable();
}
