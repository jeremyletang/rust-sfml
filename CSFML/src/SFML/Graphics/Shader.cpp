
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

#include <SFML/Graphics/Shader.h>
#include <SFML/Graphics/ShaderStruct.h>
#include <SFML/Graphics/TextureStruct.h>
#include <SFML/Graphics/ConvertTransform.hpp>
#include <cstddef>
#include <SFML/CallbackStream.h>



sfShader* sfShader_createFromFile(const char* vertexShaderFilename, const char* geometryShaderFilename, const char* fragmentShaderFilename)
{
    bool success = false;
    sfShader* shader = new sfShader;
    if (vertexShaderFilename || geometryShaderFilename || fragmentShaderFilename)
    {
        if (!geometryShaderFilename)
        {
            if (!vertexShaderFilename)
            {
                // fragment shader only
                success = shader->This.loadFromFile(fragmentShaderFilename, sf::Shader::Fragment);
            }
            else if (!fragmentShaderFilename)
            {
                // vertex shader only
                success = shader->This.loadFromFile(vertexShaderFilename, sf::Shader::Vertex);
            }
            else
            {
                // vertex + fragment shaders
                success = shader->This.loadFromFile(vertexShaderFilename, fragmentShaderFilename);
            }
        }
        else
        {
            // vertex + geometry + fragment shaders
            success = shader->This.loadFromFile(vertexShaderFilename, geometryShaderFilename, fragmentShaderFilename);
        }
    }

    if (!success)
    {
        delete shader;
        shader = NULL;
    }

    return shader;
}



sfShader* sfShader_createFromMemory(const char* vertexShader, const char* geometryShader, const char* fragmentShader)
{
    bool success = false;
    sfShader* shader = new sfShader;
    if (vertexShader || geometryShader || fragmentShader)
    {
        if (!geometryShader)
        {
            if (!vertexShader)
            {
                // fragment shader only
                success = shader->This.loadFromMemory(fragmentShader, sf::Shader::Fragment);
            }
            else if (!fragmentShader)
            {
                // vertex shader only
                success = shader->This.loadFromMemory(vertexShader, sf::Shader::Vertex);
            }
            else
            {
                // vertex + fragment shaders
                success = shader->This.loadFromMemory(vertexShader, fragmentShader);
            }
        }
        else
        {
            // vertex + geometry + fragment shaders
            success = shader->This.loadFromMemory(vertexShader, geometryShader, fragmentShader);
        }
    }

    if (!success)
    {
        delete shader;
        shader = NULL;
    }

    return shader;
}



sfShader* sfShader_createFromStream(sfInputStream* vertexShaderStream, sfInputStream* geometryShaderStream, sfInputStream* fragmentShaderStream)
{
    bool success = false;
    sfShader* shader = new sfShader;
    if (vertexShaderStream || geometryShaderStream || fragmentShaderStream)
    {
        if (!geometryShaderStream)
        {
            if (!vertexShaderStream)
            {
                // fragment shader only
                CallbackStream stream(fragmentShaderStream);
                success = shader->This.loadFromStream(stream, sf::Shader::Fragment);
            }
            else if (!fragmentShaderStream)
            {
                // vertex shader only
                CallbackStream stream(vertexShaderStream);
                success = shader->This.loadFromStream(stream, sf::Shader::Vertex);
            }
            else
            {
                // vertex + fragment shaders
                CallbackStream vertexStream(vertexShaderStream);
                CallbackStream fragmentStream(fragmentShaderStream);
                success = shader->This.loadFromStream(vertexStream, fragmentStream);
            }
        }
        else
        {
            // vertex + geometry + fragment shaders
            CallbackStream vertexStream(vertexShaderStream);
            CallbackStream geometryStream(geometryShaderStream);
            CallbackStream fragmentStream(fragmentShaderStream);
            success = shader->This.loadFromStream(vertexStream, geometryStream, fragmentStream);
        }
    }

    if (!success)
    {
        delete shader;
        shader = NULL;
    }

    return shader;
}



void sfShader_destroy(sfShader* shader)
{
	delete shader;
}



void sfShader_setFloatUniform(sfShader* shader, const char* name, float x)
{
	shader->This.setUniform(name, x);
}



void sfShader_setVec2Uniform(sfShader* shader, const char* name, sfGlslVec2 vector)
{
	shader->This.setUniform(name, sf::Glsl::Vec2(vector.x, vector.y));
}



void sfShader_setVec3Uniform(sfShader* shader, const char* name, sfGlslVec3 vector)
{
	shader->This.setUniform(name, sf::Glsl::Vec3(vector.x, vector.y, vector.z));
}



void sfShader_setVec4Uniform(sfShader* shader, const char* name, sfGlslVec4 vector)
{
	shader->This.setUniform(name, sf::Glsl::Vec4(vector.x, vector.y, vector.z, vector.w));
}



void sfShader_setColorUniform(sfShader* shader, const char* name, sfColor color)
{
	sfGlslVec4 vec4;
	vec4.x = color.r / 255.f;
	vec4.y = color.g / 255.f;
	vec4.z = color.b / 255.f;
	vec4.w = color.a / 255.f;

	sfShader_setVec4Uniform(shader, name, vec4);
}


void sfShader_setIntUniform(sfShader* shader, const char* name, int x)
{
	shader->This.setUniform(name, x);
}



void sfShader_setIvec2Uniform(sfShader* shader, const char* name, sfGlslIvec2 vector)
{
	shader->This.setUniform(name, sf::Glsl::Ivec2(vector.x, vector.y));
}



void sfShader_setIvec3Uniform(sfShader* shader, const char* name, sfGlslIvec3 vector)
{
	shader->This.setUniform(name, sf::Glsl::Ivec3(vector.x, vector.y, vector.z));
}



void sfShader_setIvec4Uniform(sfShader* shader, const char* name, sfGlslIvec4 vector)
{
	shader->This.setUniform(name, sf::Glsl::Ivec4(vector.x, vector.y, vector.z, vector.w));
}



void sfShader_setIntColorUniform(sfShader* shader, const char* name, sfColor color)
{
	sfGlslIvec4 ivec4;
	ivec4.x = (int)color.r;
	ivec4.y = (int)color.g;
	ivec4.z = (int)color.b;
	ivec4.w = (int)color.a;

	sfShader_setIvec4Uniform(shader, name, ivec4);
}


void sfShader_setBoolUniform(sfShader* shader, const char* name, sfBool x)
{
	shader->This.setUniform(name, x != sfFalse);
}



void sfShader_setBvec2Uniform(sfShader* shader, const char* name, sfGlslBvec2 vector)
{
	shader->This.setUniform(name, sf::Glsl::Bvec2(vector.x != sfFalse, vector.y != sfFalse));
}



void sfShader_setBvec3Uniform(sfShader* shader, const char* name, sfGlslBvec3 vector)
{
	shader->This.setUniform(name, sf::Glsl::Bvec3(vector.x != sfFalse, vector.y != sfFalse, vector.z != sfFalse));
}



void sfShader_setBvec4Uniform(sfShader* shader, const char* name, sfGlslBvec4 vector)
{
	shader->This.setUniform(name, sf::Glsl::Bvec4(vector.x != sfFalse, vector.y != sfFalse, vector.z != sfFalse, vector.w != sfFalse));
}



void sfShader_setMat3Uniform(sfShader* shader, const char* name, const sfGlslMat3* matrix)
{
	shader->This.setUniform(name, sf::Glsl::Mat3(matrix->array));
}



void sfShader_setMat4Uniform(sfShader* shader, const char* name, const sfGlslMat4* matrix)
{
	shader->This.setUniform(name, sf::Glsl::Mat4(matrix->array));
}



void sfShader_setTextureUniform(sfShader* shader, const char* name, const sfTexture* texture)
{
	shader->This.setUniform(name, *texture->This);
}



void sfShader_setCurrentTextureUniform(sfShader* shader, const char* name)
{
	shader->This.setUniform(name, sf::Shader::CurrentTexture);
}



void sfShader_setFloatUniformArray(sfShader* shader, const char* name, const float* scalarArray, size_t length)
{
	shader->This.setUniformArray(name, scalarArray, length);
}



void sfShader_setVec2UniformArray(sfShader* shader, const char* name, const sfGlslVec2* vectorArray, size_t length)
{
	shader->This.setUniformArray(name, reinterpret_cast<const sf::Glsl::Vec2*>(vectorArray), length);
}



void sfShader_setVec3UniformArray(sfShader* shader, const char* name, const sfGlslVec3* vectorArray, size_t length)
{
	shader->This.setUniformArray(name, reinterpret_cast<const sf::Glsl::Vec3*>(vectorArray), length);
}



void sfShader_setVec4UniformArray(sfShader* shader, const char* name, const sfGlslVec4* vectorArray, size_t length)
{
	shader->This.setUniformArray(name, reinterpret_cast<const sf::Glsl::Vec4*>(vectorArray), length);
}



void sfShader_setMat3UniformArray(sfShader* shader, const char* name, const sfGlslMat3* matrixArray, size_t length)
{
	shader->This.setUniformArray(name, reinterpret_cast<const sf::Glsl::Mat3*>(matrixArray), length);
}



void sfShader_setMat4UniformArray(sfShader* shader, const char* name, const sfGlslMat4* matrixArray, size_t length)
{
	shader->This.setUniformArray(name, reinterpret_cast<const sf::Glsl::Mat4*>(matrixArray), length);
}



void sfShader_setFloatParameter(sfShader* shader, const char* name, float x)
{
	shader->This.setParameter(name, x);
}



void sfShader_setFloat2Parameter(sfShader* shader, const char* name, float x, float y)
{
	shader->This.setParameter(name, x, y);
}



void sfShader_setFloat3Parameter(sfShader* shader, const char* name, float x, float y, float z)
{
	shader->This.setParameter(name, x, y, z);
}



void sfShader_setFloat4Parameter(sfShader* shader, const char* name, float x, float y, float z, float w)
{
	shader->This.setParameter(name, x, y, z, w);
}



void sfShader_setVector2Parameter(sfShader* shader, const char* name, sfVector2f vector)
{
	shader->This.setParameter(name, sf::Vector2f(vector.x, vector.y));
}



void sfShader_setVector3Parameter(sfShader* shader, const char* name, sfVector3f vector)
{
	shader->This.setParameter(name, sf::Vector3f(vector.x, vector.y, vector.z));
}



void sfShader_setColorParameter(sfShader* shader, const char* name, sfColor color)
{
	shader->This.setParameter(name, sf::Color(color.r, color.g, color.b, color.a));
}



void sfShader_setTransformParameter(sfShader* shader, const char* name, sfTransform transform)
{
	shader->This.setParameter(name, convertTransform(transform));
}



void sfShader_setTextureParameter(sfShader* shader, const char* name, const sfTexture* texture)
{

	shader->This.setParameter(name,*texture->This);
}



void sfShader_setCurrentTextureParameter(sfShader* shader, const char* name)
{
	shader->This.setParameter(name, sf::Shader::CurrentTexture);
}



unsigned int sfShader_getNativeHandle(const sfShader* shader)
{
	return shader->This.getNativeHandle();
}



void sfShader_bind(const sfShader* shader)
{
	sf::Shader::bind(shader ? &shader->This : NULL);
}



sfBool sfShader_isAvailable(void)
{
	return sf::Shader::isAvailable() ? sfTrue : sfFalse;
}



sfBool sfShader_isGeometryAvailable(void)
{
    return sf::Shader::isGeometryAvailable() ? sfTrue : sfFalse;
}
