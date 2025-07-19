#include "Graphics/Color.hpp"
#include "Graphics/Rect.hpp"
#include "SFML/System/Angle.hpp"
#include "SFML/System/Vector2.hpp"
#include "System/Vector2.hpp"
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/Shape.hpp>
#include <cstddef>

typedef size_t (*sfCustomShapeGetPointCountCb)(void *);
typedef sfVector2f (*sfCustomShapeGetPointCb)(size_t, void *);

class sfCustomShape final : public sf::Shape {
  public:
    sfCustomShape(sfCustomShapeGetPointCountCb getPointCount,
                  sfCustomShapeGetPointCb getPoint,
                  void *userData) : myGetPointCountCb(getPointCount),
                                    myGetPointCb(getPoint),
                                    myUserData(userData) {
    }

    virtual std::size_t getPointCount() const final {
        return myGetPointCountCb(myUserData);
    }

    virtual sf::Vector2f getPoint(std::size_t index) const final {
        sfVector2f point = myGetPointCb(index, myUserData);
        return convertVector2(point);
    }

    using sf::Shape::update;

  private:
    sfCustomShapeGetPointCountCb myGetPointCountCb;
    sfCustomShapeGetPointCb myGetPointCb;
    void *myUserData;
};

extern "C" sfCustomShape *sfCustomShape_new(sfCustomShapeGetPointCountCb getPointCount,
                                            sfCustomShapeGetPointCb getPoint,
                                            void *userData) {
    return new sfCustomShape(getPointCount, getPoint, userData);
}

extern "C" void sfCustomShape_del(sfCustomShape *shape) {
    delete shape;
}

extern "C" void sfCustomShape_setPosition(sfCustomShape *shape, sfVector2f position) {
    shape->setPosition(convertVector2(position));
}

extern "C" void sfCustomShape_setRotation(sfCustomShape *shape, float angle) {
    shape->setRotation(sf::degrees(angle));
}

extern "C" void sfCustomShape_setScale(sfCustomShape *shape, sfVector2f scale) {
    shape->setScale(convertVector2(scale));
}

extern "C" void sfCustomShape_setOrigin(sfCustomShape *shape, sfVector2f origin) {
    shape->setOrigin(convertVector2(origin));
}

extern "C" sfVector2f sfCustomShape_getPosition(const sfCustomShape *shape) {
    return convertVector2(shape->getPosition());
}

extern "C" float sfCustomShape_getRotation(const sfCustomShape *shape) {
    return shape->getRotation().asDegrees();
}

extern "C" sfVector2f sfCustomShape_getScale(const sfCustomShape *shape) {
    return convertVector2(shape->getScale());
}

extern "C" sfVector2f sfCustomShape_getOrigin(const sfCustomShape *shape) {
    return convertVector2(shape->getOrigin());
}

extern "C" void sfCustomShape_move(sfCustomShape *shape, sfVector2f offset) {
    shape->move(convertVector2(offset));
}

extern "C" void sfCustomShape_rotate(sfCustomShape *shape, float angle) {
    shape->rotate(sf::degrees(angle));
}

extern "C" void sfCustomShape_scale(sfCustomShape *shape, sfVector2f factors) {
    shape->scale(convertVector2(factors));
}

extern "C" sf::Transform const *sfCustomShape_getTransform(const sfCustomShape *shape) {
    return &shape->getTransform();
}

extern "C" sf::Transform const *sfCustomShape_getInverseTransform(const sfCustomShape *shape) {
    return &shape->getInverseTransform();
}

extern "C" void sfCustomShape_setTexture(sfCustomShape *shape, const sf::Texture *texture, bool resetRect) {
    shape->setTexture(texture, resetRect);
}

extern "C" void sfCustomShape_setTextureRect(sfCustomShape *shape, sfIntRect rect) {
    shape->setTextureRect(convertRect(rect));
}

extern "C" void sfCustomShape_setFillColor(sfCustomShape *shape, sfColor color) {
    shape->setFillColor(convertColor(color));
}

extern "C" void sfCustomShape_setOutlineColor(sfCustomShape *shape, sfColor color) {
    shape->setOutlineColor(convertColor(color));
}

extern "C" void sfCustomShape_setOutlineThickness(sfCustomShape *shape, float thickness) {
    shape->setOutlineThickness(thickness);
}

extern "C" const sf::Texture *sfCustomShape_getTexture(const sfCustomShape *shape) {
    return shape->getTexture();
}

extern "C" sfIntRect sfCustomShape_getTextureRect(const sfCustomShape *shape) {
    return convertRect(shape->getTextureRect());
}

extern "C" sfColor sfCustomShape_getFillColor(const sfCustomShape *shape) {
    return convertColor(shape->getFillColor());
}

extern "C" sfColor sfCustomShape_getOutlineColor(const sfCustomShape *shape) {
    return convertColor(shape->getOutlineColor());
}

extern "C" float sfCustomShape_getOutlineThickness(const sfCustomShape *shape) {
    return shape->getOutlineThickness();
}

extern "C" size_t sfCustomShape_getPointCount(const sfCustomShape *shape) {
    return shape->getPointCount();
}

extern "C" sfVector2f sfCustomShape_getGeometricCenter(const sfCustomShape *shape) {
    return convertVector2(shape->getGeometricCenter());
}

extern "C" sfVector2f sfCustomShape_getPoint(const sfCustomShape *shape, size_t index) {
    return convertVector2(shape->getPoint(index));
}

extern "C" sfFloatRect sfCustomShape_getLocalBounds(const sfCustomShape *shape) {
    return convertRect(shape->getLocalBounds());
}

extern "C" sfFloatRect sfCustomShape_getGlobalBounds(const sfCustomShape *shape) {
    return convertRect(shape->getGlobalBounds());
}

extern "C" void sfCustomShape_update(sfCustomShape *shape) {
    shape->update();
}
