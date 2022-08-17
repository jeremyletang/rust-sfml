#include "Graphics/Color.h"
#include "Graphics/Rect.h"
#include "System/Vector2.h"
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/ConvexShape.hpp>
#include <cstddef>

extern "C" sf::ConvexShape *sfConvexShape_create(void) {
    return new sf::ConvexShape;
}

extern "C" sf::ConvexShape *sfConvexShape_copy(const sf::ConvexShape *shape) {
    return new sf::ConvexShape(*shape);
}

extern "C" void sfConvexShape_destroy(sf::ConvexShape *shape) {
    delete shape;
}

extern "C" void sfConvexShape_setPosition(sf::ConvexShape *shape, sfVector2f position) {
    shape->setPosition(position.x, position.y);
}

extern "C" void sfConvexShape_setRotation(sf::ConvexShape *shape, float angle) {
    shape->setRotation(angle);
}

extern "C" void sfConvexShape_setScale(sf::ConvexShape *shape, sfVector2f scale) {
    shape->setScale(scale.x, scale.y);
}

extern "C" void sfConvexShape_setOrigin(sf::ConvexShape *shape, sfVector2f origin) {
    shape->setOrigin(origin.x, origin.y);
}

extern "C" sfVector2f sfConvexShape_getPosition(const sf::ConvexShape *shape) {
    sf::Vector2f vec2 = shape->getPosition();
    return {vec2.x, vec2.y};
}

extern "C" float sfConvexShape_getRotation(const sf::ConvexShape *shape) {
    return shape->getRotation();
}

extern "C" sfVector2f sfConvexShape_getScale(const sf::ConvexShape *shape) {
    sf::Vector2f vec2 = shape->getScale();
    return {vec2.x, vec2.y};
}

extern "C" sfVector2f sfConvexShape_getOrigin(const sf::ConvexShape *shape) {
    sf::Vector2f vec2 = shape->getOrigin();
    return {vec2.x, vec2.y};
}

extern "C" void sfConvexShape_move(sf::ConvexShape *shape, sfVector2f offset) {
    shape->move(offset.x, offset.y);
}

extern "C" void sfConvexShape_rotate(sf::ConvexShape *shape, float angle) {
    shape->rotate(angle);
}

extern "C" void sfConvexShape_scale(sf::ConvexShape *shape, sfVector2f factors) {
    shape->scale(factors.x, factors.y);
}

extern "C" sf::Transform const *sfConvexShape_getTransform(const sf::ConvexShape *shape) {
    return &shape->getTransform();
}

extern "C" sf::Transform const *sfConvexShape_getInverseTransform(const sf::ConvexShape *shape) {
    return &shape->getInverseTransform();
}

extern "C" void sfConvexShape_setTexture(sf::ConvexShape *shape, const sf::Texture *texture, bool resetRect) {
    shape->setTexture(texture, resetRect);
}

extern "C" void sfConvexShape_setTextureRect(sf::ConvexShape *shape, sfIntRect rect) {
    shape->setTextureRect(sf::IntRect(rect.left, rect.top, rect.width, rect.height));
}

extern "C" void sfConvexShape_setFillColor(sf::ConvexShape *shape, sfColor color) {
    shape->setFillColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfConvexShape_setOutlineColor(sf::ConvexShape *shape, sfColor color) {
    shape->setOutlineColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfConvexShape_setOutlineThickness(sf::ConvexShape *shape, float thickness) {
    shape->setOutlineThickness(thickness);
}

extern "C" const sf::Texture *sfConvexShape_getTexture(const sf::ConvexShape *shape) {
    return shape->getTexture();
}

extern "C" sfIntRect sfConvexShape_getTextureRect(const sf::ConvexShape *shape) {
    sf::IntRect rect = shape->getTextureRect();
    return {rect.left, rect.top, rect.width, rect.height};
}

extern "C" sfColor sfConvexShape_getFillColor(const sf::ConvexShape *shape) {
    sf::Color color = shape->getFillColor();
    return {color.r, color.g, color.b, color.a};
}

extern "C" sfColor sfConvexShape_getOutlineColor(const sf::ConvexShape *shape) {
    sf::Color color = shape->getOutlineColor();
    return {color.r, color.g, color.b, color.a};
}

extern "C" float sfConvexShape_getOutlineThickness(const sf::ConvexShape *shape) {
    return shape->getOutlineThickness();
}

extern "C" size_t sfConvexShape_getPointCount(const sf::ConvexShape *shape) {
    return shape->getPointCount();
}

extern "C" sfVector2f sfConvexShape_getPoint(const sf::ConvexShape *shape, size_t index) {
    sf::Vector2f vec2 = shape->getPoint(index);
    return {vec2.x, vec2.y};
}

extern "C" void sfConvexShape_setPointCount(sf::ConvexShape *shape, size_t count) {
    shape->setPointCount(count);
}

extern "C" void sfConvexShape_setPoint(sf::ConvexShape *shape, size_t index, sfVector2f point) {
    shape->setPoint(index, sf::Vector2f(point.x, point.y));
}

extern "C" sfFloatRect sfConvexShape_getLocalBounds(const sf::ConvexShape *shape) {
    sf::FloatRect rect = shape->getLocalBounds();
    return {rect.left, rect.top, rect.width, rect.height};
}

extern "C" sfFloatRect sfConvexShape_getGlobalBounds(const sf::ConvexShape *shape) {
    sf::FloatRect rect = shape->getGlobalBounds();
    return {rect.left, rect.top, rect.width, rect.height};
}
