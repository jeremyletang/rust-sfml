#include "System/InputStreamHelper.hpp"
#include "System/Vector2.hpp"
#include "System/Vector3.hpp"
#include <SFML/Graphics/Shader.hpp>
#include <cstddef>

// 2D vectors
typedef sfVector2f sfGlslVec2;
typedef sfVector2i sfGlslIvec2;

typedef struct
{
    bool x;
    bool y;
} sfGlslBvec2;

// 3D vectors
typedef sfVector3f sfGlslVec3;

typedef struct
{
    int x;
    int y;
    int z;
} sfGlslIvec3;

typedef struct
{
    bool x;
    bool y;
    bool z;
} sfGlslBvec3;

// 4D vectors
typedef struct
{
    float x;
    float y;
    float z;
    float w;
} sfGlslVec4;

typedef struct
{
    int x;
    int y;
    int z;
    int w;
} sfGlslIvec4;

typedef struct
{
    bool x;
    bool y;
    bool z;
    bool w;
} sfGlslBvec4;

// matrices
typedef struct
{
    float array[3 * 3];
} sfGlslMat3;

typedef struct
{
    float array[4 * 4];
} sfGlslMat4;

extern "C" sf::Shader *sfShader_defaultConstruct() {
    return new sf::Shader;
}

extern "C" bool sfShader_loadFromMemory_1(sf::Shader *shader, const char *content, sf::Shader::Type type) {
    return shader->loadFromMemory(content, type);
}

extern "C" bool sfShader_loadFromFile_1(sf::Shader *shader, const char *filename, sf::Shader::Type type) {
    return shader->loadFromFile(filename, type);
}

extern "C" bool sfShader_loadFromStream_1(sf::Shader *shader, sfInputStreamHelper *stream, sf::Shader::Type type) {
    return shader->loadFromStream(*stream, type);
}

extern "C" bool sfShader_loadFromMemory_vert_frag(sf::Shader *shader, const char *vert, const char *frag) {
    return shader->loadFromMemory(vert, frag);
}

extern "C" bool sfShader_loadFromFile_vert_frag(sf::Shader *shader, const char *vert, const char *frag) {
    return shader->loadFromFile(vert, frag);
}

extern "C" bool sfShader_loadFromStream_vert_frag(sf::Shader *shader, sfInputStreamHelper *vert, sfInputStreamHelper *frag) {
    return shader->loadFromStream(*vert, *frag);
}

extern "C" bool sfShader_loadFromMemory_all(sf::Shader *shader, const char *vert, const char *geom, const char *frag) {
    return shader->loadFromMemory(vert, geom, frag);
}

extern "C" bool sfShader_loadFromFile_all(sf::Shader *shader, const char *vert, const char *geom, const char *frag) {
    return shader->loadFromFile(vert, geom, frag);
}

extern "C" bool sfShader_loadFromStream_all(sf::Shader *shader, sfInputStreamHelper *vert, sfInputStreamHelper *geom, sfInputStreamHelper *frag) {
    return shader->loadFromStream(*vert, *geom, *frag);
}

extern "C" void sfShader_destroy(sf::Shader *shader) {
    delete shader;
}

extern "C" void sfShader_setFloatUniform(sf::Shader *shader, const char *name, float x) {
    shader->setUniform(name, x);
}

extern "C" void sfShader_setVec2Uniform(sf::Shader *shader, const char *name, sfGlslVec2 vector) {
    shader->setUniform(name, sf::Glsl::Vec2(vector.x, vector.y));
}

extern "C" void sfShader_setVec3Uniform(sf::Shader *shader, const char *name, sfGlslVec3 vector) {
    shader->setUniform(name, sf::Glsl::Vec3(vector.x, vector.y, vector.z));
}

extern "C" void sfShader_setVec4Uniform(sf::Shader *shader, const char *name, sfGlslVec4 vector) {
    shader->setUniform(name, sf::Glsl::Vec4(vector.x, vector.y, vector.z, vector.w));
}

extern "C" void sfShader_setIntUniform(sf::Shader *shader, const char *name, int x) {
    shader->setUniform(name, x);
}

extern "C" void sfShader_setIvec2Uniform(sf::Shader *shader, const char *name, sfGlslIvec2 vector) {
    shader->setUniform(name, sf::Glsl::Ivec2(vector.x, vector.y));
}

extern "C" void sfShader_setIvec3Uniform(sf::Shader *shader, const char *name, sfGlslIvec3 vector) {
    shader->setUniform(name, sf::Glsl::Ivec3(vector.x, vector.y, vector.z));
}

extern "C" void sfShader_setIvec4Uniform(sf::Shader *shader, const char *name, sfGlslIvec4 vector) {
    shader->setUniform(name, sf::Glsl::Ivec4(vector.x, vector.y, vector.z, vector.w));
}

extern "C" void sfShader_setBoolUniform(sf::Shader *shader, const char *name, bool x) {
    shader->setUniform(name, x);
}

extern "C" void sfShader_setBvec2Uniform(sf::Shader *shader, const char *name, sfGlslBvec2 vector) {
    shader->setUniform(name, sf::Glsl::Bvec2(vector.x, vector.y));
}

extern "C" void sfShader_setBvec3Uniform(sf::Shader *shader, const char *name, sfGlslBvec3 vector) {
    shader->setUniform(name, sf::Glsl::Bvec3(vector.x, vector.y, vector.z));
}

extern "C" void sfShader_setBvec4Uniform(sf::Shader *shader, const char *name, sfGlslBvec4 vector) {
    shader->setUniform(name, sf::Glsl::Bvec4(vector.x, vector.y, vector.z, vector.w));
}

extern "C" void sfShader_setMat3Uniform(sf::Shader *shader, const char *name, const sfGlslMat3 *matrix) {
    shader->setUniform(name, sf::Glsl::Mat3(matrix->array));
}

extern "C" void sfShader_setMat4Uniform(sf::Shader *shader, const char *name, const sfGlslMat4 *matrix) {
    shader->setUniform(name, sf::Glsl::Mat4(matrix->array));
}

extern "C" void sfShader_setTextureUniform(sf::Shader *shader, const char *name, const sf::Texture *texture) {
    shader->setUniform(name, *texture);
}

extern "C" void sfShader_setCurrentTextureUniform(sf::Shader *shader, const char *name) {
    shader->setUniform(name, sf::Shader::CurrentTexture);
}

extern "C" void sfShader_setFloatUniformArray(sf::Shader *shader, const char *name, const float *scalarArray, size_t length) {
    shader->setUniformArray(name, scalarArray, length);
}

extern "C" void sfShader_setVec2UniformArray(sf::Shader *shader, const char *name, const sf::Glsl::Vec2 *vectorArray, size_t length) {
    shader->setUniformArray(name, vectorArray, length);
}

extern "C" void sfShader_setVec3UniformArray(sf::Shader *shader, const char *name, const sf::Glsl::Vec3 *vectorArray, size_t length) {
    shader->setUniformArray(name, vectorArray, length);
}

extern "C" void sfShader_setVec4UniformArray(sf::Shader *shader, const char *name, const sf::Glsl::Vec4 *vectorArray, size_t length) {
    shader->setUniformArray(name, vectorArray, length);
}

extern "C" void sfShader_setMat3UniformArray(sf::Shader *shader, const char *name, const sf::Glsl::Mat3 *matrixArray, size_t length) {
    shader->setUniformArray(name, matrixArray, length);
}

extern "C" void sfShader_setMat4UniformArray(sf::Shader *shader, const char *name, const sf::Glsl::Mat4 *matrixArray, size_t length) {
    shader->setUniformArray(name, matrixArray, length);
}

extern "C" unsigned int sfShader_getNativeHandle(const sf::Shader *shader) {
    return shader->getNativeHandle();
}

extern "C" void sfShader_bind(const sf::Shader *shader) {
    sf::Shader::bind(shader);
}

extern "C" bool sfShader_isAvailable(void) {
    return sf::Shader::isAvailable();
}

extern "C" bool sfShader_isGeometryAvailable(void) {
    return sf::Shader::isGeometryAvailable();
}
