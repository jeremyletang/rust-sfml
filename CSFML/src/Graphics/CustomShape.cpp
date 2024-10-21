#include "Graphics/Color.hpp"
#include "Graphics/Rect.hpp"
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
        return sf::Vector2f(point.x, point.y);
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
    shape->setPosition(position.x, position.y);
}

extern "C" void sfCustomShape_setRotation(sfCustomShape *shape, float angle) {
    shape->setRotation(angle);
}

extern "C" void sfCustomShape_setScale(sfCustomShape *shape, sfVector2f scale) {
    shape->setScale(scale.x, scale.y);
}

extern "C" void sfCustomShape_setOrigin(sfCustomShape *shape, sfVector2f origin) {
    shape->setOrigin(origin.x, origin.y);
}

extern "C" sfVector2f sfCustomShape_getPosition(const sfCustomShape *shape) {
    sf::Vector2f vec2 = shape->getPosition();
    return {vec2.x, vec2.y};
}

extern "C" float sfCustomShape_getRotation(const sfCustomShape *shape) {
    return shape->getRotation();
}

extern "C" sfVector2f sfCustomShape_getScale(const sfCustomShape *shape) {
    sf::Vector2f vec2 = shape->getScale();
    return {vec2.x, vec2.y};
}

extern "C" sfVector2f sfCustomShape_getOrigin(const sfCustomShape *shape) {
    sf::Vector2f vec2 = shape->getOrigin();
    return {vec2.x, vec2.y};
}

extern "C" void sfCustomShape_move(sfCustomShape *shape, sfVector2f offset) {
    shape->move(offset.x, offset.y);
}

extern "C" void sfCustomShape_rotate(sfCustomShape *shape, float angle) {
    shape->rotate(angle);
}

extern "C" void sfCustomShape_scale(sfCustomShape *shape, sfVector2f factors) {
    shape->scale(factors.x, factors.y);
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
    shape->setTextureRect(sf::IntRect(rect.left, rect.top, rect.width, rect.height));
}

extern "C" void sfCustomShape_setFillColor(sfCustomShape *shape, sfColor color) {
    shape->setFillColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfCustomShape_setOutlineColor(sfCustomShape *shape, sfColor color) {
    shape->setOutlineColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfCustomShape_setOutlineThickness(sfCustomShape *shape, float thickness) {
    shape->setOutlineThickness(thickness);
}

extern "C" const sf::Texture *sfCustomShape_getTexture(const sfCustomShape *shape) {
    return shape->getTexture();
}

extern "C" sfIntRect sfCustomShape_getTextureRect(const sfCustomShape *shape) {
    sf::IntRect rect = shape->getTextureRect();
    return {rect.left, rect.top, rect.width, rect.height};
}

extern "C" sfColor sfCustomShape_getFillColor(const sfCustomShape *shape) {
    sf::Color color = shape->getFillColor();
    return {color.r, color.g, color.b, color.a};
}

extern "C" sfColor sfCustomShape_getOutlineColor(const sfCustomShape *shape) {
    sf::Color color = shape->getOutlineColor();
    return {color.r, color.g, color.b, color.a};
}

extern "C" float sfCustomShape_getOutlineThickness(const sfCustomShape *shape) {
    return shape->getOutlineThickness();
}

extern "C" size_t sfCustomShape_getPointCount(const sfCustomShape *shape) {
    return shape->getPointCount();
}

extern "C" sfVector2f sfCustomShape_getPoint(const sfCustomShape *shape, size_t index) {
    sf::Vector2f vec2 = shape->getPoint(index);
    return {vec2.x, vec2.y};
}

extern "C" sfFloatRect sfCustomShape_getLocalBounds(const sfCustomShape *shape) {
    sf::FloatRect rect = shape->getLocalBounds();
    return {rect.left, rect.top, rect.width, rect.height};
}

extern "C" sfFloatRect sfCustomShape_getGlobalBounds(const sfCustomShape *shape) {
    sf::FloatRect rect = shape->getGlobalBounds();
    return {rect.left, rect.top, rect.width, rect.height};
}

extern "C" void sfCustomShape_update(sfCustomShape *shape) {
    shape->update();
}
