#include "Graphics/Color.hpp"
#include "Graphics/Rect.hpp"
#include "System/Vector2.hpp"
#include <SFML/Graphics/CircleShape.hpp>
#include <SFML/Graphics/Color.hpp>
#include <cstddef>

extern "C" sf::CircleShape *sfCircleShape_new(void) {
    return new sf::CircleShape;
}

extern "C" sf::CircleShape *sfCircleShape_cpy(const sf::CircleShape *shape) {
    return new sf::CircleShape(*shape);
}

extern "C" void sfCircleShape_del(sf::CircleShape *shape) {
    delete shape;
}

extern "C" void sfCircleShape_setPosition(sf::CircleShape *shape, sfVector2f position) {
    shape->setPosition(convertVector2(position));
}

extern "C" void sfCircleShape_setRotation(sf::CircleShape *shape, float angle) {
    shape->setRotation(sf::degrees(angle));
}

extern "C" void sfCircleShape_setScale(sf::CircleShape *shape, sfVector2f scale) {
    shape->setScale(convertVector2(scale));
}

extern "C" void sfCircleShape_setOrigin(sf::CircleShape *shape, sfVector2f origin) {
    shape->setOrigin(convertVector2(origin));
}

extern "C" sfVector2f sfCircleShape_getPosition(const sf::CircleShape *shape) {
    return convertVector2(shape->getPosition());
}

extern "C" float sfCircleShape_getRotation(const sf::CircleShape *shape) {
    return shape->getRotation().asDegrees();
}

extern "C" sfVector2f sfCircleShape_getScale(const sf::CircleShape *shape) {
    return convertVector2(shape->getScale());
}

extern "C" sfVector2f sfCircleShape_getOrigin(const sf::CircleShape *shape) {
    return convertVector2(shape->getOrigin());
}

extern "C" void sfCircleShape_move(sf::CircleShape *shape, sfVector2f offset) {
    shape->move(convertVector2(offset));
}

extern "C" void sfCircleShape_rotate(sf::CircleShape *shape, float angle) {
    shape->rotate(sf::degrees(angle));
}

extern "C" void sfCircleShape_scale(sf::CircleShape *shape, sfVector2f factors) {
    shape->scale(convertVector2(factors));
}

extern "C" sf::Transform const *sfCircleShape_getTransform(const sf::CircleShape *shape) {
    return &shape->getTransform();
}

extern "C" sf::Transform const *sfCircleShape_getInverseTransform(const sf::CircleShape *shape) {
    return &shape->getInverseTransform();
}

extern "C" void sfCircleShape_setTexture(sf::CircleShape *shape, const sf::Texture *texture, bool resetRect) {
    shape->setTexture(texture, resetRect);
}

extern "C" void sfCircleShape_setTextureRect(sf::CircleShape *shape, sfIntRect rect) {
    shape->setTextureRect({sf::Vector2i(rect.position.x, rect.position.y), sf::Vector2i(rect.size.x, rect.size.y)});
}

extern "C" void sfCircleShape_setFillColor(sf::CircleShape *shape, sfColor color) {
    shape->setFillColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfCircleShape_setOutlineColor(sf::CircleShape *shape, sfColor color) {
    shape->setOutlineColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfCircleShape_setOutlineThickness(sf::CircleShape *shape, float thickness) {
    shape->setOutlineThickness(thickness);
}

extern "C" const sf::Texture *sfCircleShape_getTexture(const sf::CircleShape *shape) {
    return shape->getTexture();
}

extern "C" sfIntRect sfCircleShape_getTextureRect(const sf::CircleShape *shape) {
    return convertRect(shape->getTextureRect());
}

extern "C" sfColor sfCircleShape_getFillColor(const sf::CircleShape *shape) {
    return convertColor(shape->getFillColor());
}

extern "C" sfColor sfCircleShape_getOutlineColor(const sf::CircleShape *shape) {
    return convertColor(shape->getOutlineColor());
}

extern "C" float sfCircleShape_getOutlineThickness(const sf::CircleShape *shape) {
    return shape->getOutlineThickness();
}

extern "C" size_t sfCircleShape_getPointCount(const sf::CircleShape *shape) {
    return shape->getPointCount();
}

extern "C" sfVector2f sfCircleShape_getPoint(const sf::CircleShape *shape, size_t index) {
    return convertVector2(shape->getPoint(index));
}

extern "C" sfVector2f sfCircleShape_getGeometricCenter(const sf::CircleShape *shape) {
    return convertVector2(shape->getGeometricCenter());
}

extern "C" void sfCircleShape_setRadius(sf::CircleShape *shape, float radius) {
    shape->setRadius(radius);
}

extern "C" float sfCircleShape_getRadius(const sf::CircleShape *shape) {
    return shape->getRadius();
}

extern "C" void sfCircleShape_setPointCount(sf::CircleShape *shape, size_t count) {
    shape->setPointCount(count);
}

extern "C" sfFloatRect sfCircleShape_getLocalBounds(const sf::CircleShape *shape) {
    return convertRect(shape->getLocalBounds());
}

extern "C" sfFloatRect sfCircleShape_getGlobalBounds(const sf::CircleShape *shape) {
    return convertRect(shape->getGlobalBounds());
}
