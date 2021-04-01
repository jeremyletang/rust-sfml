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

////////////////////////////////////////////////////////////
// Headers
////////////////////////////////////////////////////////////
#include <SFML/Graphics/Transformable.h>
#include <SFML/Graphics/TransformableStruct.h>
#include <SFML/Graphics/ConvertTransform.hpp>
#include <SFML/Internal.h>


////////////////////////////////////////////////////////////
sfTransformable* sfTransformable_create(void)
{
    sfTransformable* transformable = new sfTransformable;

    return transformable;
}


////////////////////////////////////////////////////////////
sfTransformable* sfTransformable_copy(const sfTransformable* transformable)
{
    CSFML_CHECK_RETURN(transformable, NULL);

    return new sfTransformable(*transformable);
}


////////////////////////////////////////////////////////////
void sfTransformable_destroy(sfTransformable* transformable)
{
    delete transformable;
}


////////////////////////////////////////////////////////////
void sfTransformable_setPosition(sfTransformable* transformable, sfVector2f position)
{
    CSFML_CALL(transformable, setPosition(position.x, position.y));
}


////////////////////////////////////////////////////////////
void sfTransformable_setRotation(sfTransformable* transformable, float angle)
{
    CSFML_CALL(transformable, setRotation(angle));
}


////////////////////////////////////////////////////////////
void sfTransformable_setScale(sfTransformable* transformable, sfVector2f scale)
{
    CSFML_CALL(transformable, setScale(scale.x, scale.y));
}


////////////////////////////////////////////////////////////
void sfTransformable_setOrigin(sfTransformable* transformable, sfVector2f origin)
{
    CSFML_CALL(transformable, setOrigin(origin.x, origin.y));
}


////////////////////////////////////////////////////////////
sfVector2f sfTransformable_getPosition(const sfTransformable* transformable)
{
    sfVector2f position = {0, 0};
    CSFML_CHECK_RETURN(transformable, position);

    sf::Vector2f sfmlPos = transformable->This.getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}


////////////////////////////////////////////////////////////
float sfTransformable_getRotation(const sfTransformable* transformable)
{
    CSFML_CALL_RETURN(transformable, getRotation(), 0.f);
}


////////////////////////////////////////////////////////////
sfVector2f sfTransformable_getScale(const sfTransformable* transformable)
{
    sfVector2f scale = {0, 0};
    CSFML_CHECK_RETURN(transformable, scale);

    sf::Vector2f sfmlScale = transformable->This.getScale();
    scale.x = sfmlScale.x;
    scale.y = sfmlScale.y;

    return scale;
}


////////////////////////////////////////////////////////////
sfVector2f sfTransformable_getOrigin(const sfTransformable* transformable)
{
    sfVector2f origin = {0, 0};
    CSFML_CHECK_RETURN(transformable, origin);

    sf::Vector2f sfmlOrigin = transformable->This.getOrigin();
    origin.x = sfmlOrigin.x;
    origin.y = sfmlOrigin.y;

    return origin;
}


////////////////////////////////////////////////////////////
void sfTransformable_move(sfTransformable* transformable, sfVector2f offset)
{
    CSFML_CALL(transformable, move(offset.x, offset.y));
}


////////////////////////////////////////////////////////////
void sfTransformable_rotate(sfTransformable* transformable, float angle)
{
    CSFML_CALL(transformable, rotate(angle));
}


////////////////////////////////////////////////////////////
void sfTransformable_scale(sfTransformable* transformable, sfVector2f factors)
{
    CSFML_CALL(transformable, scale(factors.x, factors.y));
}


////////////////////////////////////////////////////////////
sfTransform sfTransformable_getTransform(const sfTransformable* transformable)
{
    CSFML_CHECK_RETURN(transformable, sfTransform_Identity);

    transformable->Transform = convertTransform(transformable->This.getTransform());
    return transformable->Transform;
}


////////////////////////////////////////////////////////////
sfTransform sfTransformable_getInverseTransform(const sfTransformable* transformable)
{
    CSFML_CHECK_RETURN(transformable, sfTransform_Identity);

    transformable->InverseTransform = convertTransform(transformable->This.getInverseTransform());
    return transformable->InverseTransform;
}
