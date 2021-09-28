
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

#ifndef SFML_SHADER_H
#define SFML_SHADER_H

// Headers

#include "Graphics/Color.h"

#include "Graphics/Glsl.h"
#include "Graphics/Transform.h"
#include "Graphics/Types.h"
#include "System/InputStream.h"
#include "System/Vector2.h"
#include "System/Vector3.h"
#include <stddef.h>

extern "C" void sfShader_destroy(sfShader *shader);

extern "C" void sfShader_setFloatUniform(sfShader *shader, const char *name, float x);

extern "C" void sfShader_setVec2Uniform(sfShader *shader, const char *name, sfGlslVec2 vector);

extern "C" void sfShader_setVec3Uniform(sfShader *shader, const char *name, sfGlslVec3 vector);

extern "C" void sfShader_setVec4Uniform(sfShader *shader, const char *name, sfGlslVec4 vector);

extern "C" void sfShader_setIntUniform(sfShader *shader, const char *name, int x);

extern "C" void sfShader_setIvec2Uniform(sfShader *shader, const char *name, sfGlslIvec2 vector);

extern "C" void sfShader_setIvec3Uniform(sfShader *shader, const char *name, sfGlslIvec3 vector);

extern "C" void sfShader_setIvec4Uniform(sfShader *shader, const char *name, sfGlslIvec4 vector);

extern "C" void sfShader_setBoolUniform(sfShader *shader, const char *name, sfBool x);

extern "C" void sfShader_setBvec2Uniform(sfShader *shader, const char *name, sfGlslBvec2 vector);

extern "C" void sfShader_setBvec3Uniform(sfShader *shader, const char *name, sfGlslBvec3 vector);

extern "C" void sfShader_setBvec4Uniform(sfShader *shader, const char *name, sfGlslBvec4 vector);

extern "C" void sfShader_setMat3Uniform(sfShader *shader, const char *name, const sfGlslMat3 *matrix);

extern "C" void sfShader_setMat4Uniform(sfShader *shader, const char *name, const sfGlslMat4 *matrix);

extern "C" void sfShader_setTextureUniform(sfShader *shader, const char *name, const sfTexture *texture);

extern "C" void sfShader_setCurrentTextureUniform(sfShader *shader, const char *name);

extern "C" void sfShader_setFloatUniformArray(sfShader *shader, const char *name, const float *scalarArray, size_t length);

extern "C" void sfShader_setVec2UniformArray(sfShader *shader, const char *name, const sfGlslVec2 *vectorArray, size_t length);

extern "C" void sfShader_setVec3UniformArray(sfShader *shader, const char *name, const sfGlslVec3 *vectorArray, size_t length);

extern "C" void sfShader_setVec4UniformArray(sfShader *shader, const char *name, const sfGlslVec4 *vectorArray, size_t length);

extern "C" void sfShader_setMat3UniformArray(sfShader *shader, const char *name, const sfGlslMat3 *matrixArray, size_t length);

extern "C" void sfShader_setMat4UniformArray(sfShader *shader, const char *name, const sfGlslMat4 *matrixArray, size_t length);

extern "C" unsigned int sfShader_getNativeHandle(const sfShader *shader);

extern "C" void sfShader_bind(const sfShader *shader);

extern "C" sfBool sfShader_isAvailable(void);

extern "C" sfBool sfShader_isGeometryAvailable(void);

#endif // SFML_SHADER_H
