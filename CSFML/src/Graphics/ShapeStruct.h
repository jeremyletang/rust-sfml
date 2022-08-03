
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

#ifndef SFML_SHAPESTRUCT_H
#define SFML_SHAPESTRUCT_H

// Headers

#include "System/Vector2.h"
#include <SFML/Graphics/Shape.hpp>

typedef size_t (*sfShapeGetPointCountCallback)(void *);        ///< Type of the callback used to get the number of points in a shape
typedef sfVector2f (*sfShapeGetPointCallback)(size_t, void *); ///< Type of the callback used to get a point of a shape

// Helper class implementing the callback forwarding from
// C++ to C in sfShape

class sfShape : public sf::Shape {
  public:
    sfShape(sfShapeGetPointCountCallback getPointCount,
            sfShapeGetPointCallback getPoint,
            void *userData) : myGetPointCountCallback(getPointCount),
                              myGetPointCallback(getPoint),
                              myUserData(userData) {
    }

    virtual std::size_t getPointCount() const {
        return myGetPointCountCallback(myUserData);
    }

    virtual sf::Vector2f getPoint(std::size_t index) const {
        sfVector2f point = myGetPointCallback(index, myUserData);
        return sf::Vector2f(point.x, point.y);
    }

    using sf::Shape::update;

  private:
    sfShapeGetPointCountCallback myGetPointCountCallback;
    sfShapeGetPointCallback myGetPointCallback;
    void *myUserData;
};

#endif // SFML_SHAPESTRUCT_H
