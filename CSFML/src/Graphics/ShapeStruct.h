#ifndef SFML_SHAPESTRUCT_H
#define SFML_SHAPESTRUCT_H

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
