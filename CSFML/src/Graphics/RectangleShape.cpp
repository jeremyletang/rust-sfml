#include "Graphics/Color.hpp"
#include "Graphics/Rect.hpp"
#include "System/Vector2.hpp"
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/RectangleShape.hpp>
#include <cstddef>

extern "C" sf::RectangleShape *sfRectangleShape_new(void) {
    return new sf::RectangleShape;
}

extern "C" sf::RectangleShape *sfRectangleShape_cpy(const sf::RectangleShape *shape) {
    return new sf::RectangleShape(*shape);
}

extern "C" void sfRectangleShape_del(sf::RectangleShape *shape) {
    delete shape;
}

extern "C" void sfRectangleShape_setPosition(sf::RectangleShape *shape, sfVector2f position) {
    shape->setPosition(convertVector2(position));
}

extern "C" void sfRectangleShape_setRotation(sf::RectangleShape *shape, float angle) {
    shape->setRotation(sf::degrees(angle));
}

extern "C" void sfRectangleShape_setScale(sf::RectangleShape *shape, sfVector2f scale) {
    shape->setScale(convertVector2(scale));
}

extern "C" void sfRectangleShape_setOrigin(sf::RectangleShape *shape, sfVector2f origin) {
    shape->setOrigin(convertVector2(origin));
}

extern "C" sfVector2f sfRectangleShape_getPosition(const sf::RectangleShape *shape) {
    return convertVector2(shape->getPosition());
}

extern "C" float sfRectangleShape_getRotation(const sf::RectangleShape *shape) {
    return shape->getRotation().asDegrees();
}

extern "C" sfVector2f sfRectangleShape_getScale(const sf::RectangleShape *shape) {
    return convertVector2(shape->getScale());
}

extern "C" sfVector2f sfRectangleShape_getOrigin(const sf::RectangleShape *shape) {
    return convertVector2(shape->getOrigin());
}

extern "C" void sfRectangleShape_move(sf::RectangleShape *shape, sfVector2f offset) {
    shape->move(convertVector2(offset));
}

extern "C" void sfRectangleShape_rotate(sf::RectangleShape *shape, float angle) {
    shape->rotate(sf::degrees(angle));
}

extern "C" void sfRectangleShape_scale(sf::RectangleShape *shape, sfVector2f factors) {
    shape->scale(convertVector2(factors));
}

extern "C" sf::Transform const *sfRectangleShape_getTransform(const sf::RectangleShape *shape) {
    return &shape->getTransform();
}

extern "C" sf::Transform const *sfRectangleShape_getInverseTransform(const sf::RectangleShape *shape) {
    return &shape->getInverseTransform();
}

extern "C" void sfRectangleShape_setTexture(sf::RectangleShape *shape, const sf::Texture *texture, bool resetRect) {
    shape->setTexture(texture, resetRect);
}

extern "C" void sfRectangleShape_setTextureRect(sf::RectangleShape *shape, sfIntRect rect) {
    shape->setTextureRect(convertRect(rect));
}

extern "C" void sfRectangleShape_setFillColor(sf::RectangleShape *shape, sfColor color) {
    shape->setFillColor(convertColor(color));
}

extern "C" void sfRectangleShape_setOutlineColor(sf::RectangleShape *shape, sfColor color) {
    shape->setOutlineColor(convertColor(color));
}

extern "C" void sfRectangleShape_setOutlineThickness(sf::RectangleShape *shape, float thickness) {
    shape->setOutlineThickness(thickness);
}

extern "C" const sf::Texture *sfRectangleShape_getTexture(const sf::RectangleShape *shape) {
    return shape->getTexture();
}

extern "C" sfIntRect sfRectangleShape_getTextureRect(const sf::RectangleShape *shape) {
    return convertRect(shape->getTextureRect());
}

extern "C" sfColor sfRectangleShape_getFillColor(const sf::RectangleShape *shape) {
    return convertColor(shape->getFillColor());
}

extern "C" sfColor sfRectangleShape_getOutlineColor(const sf::RectangleShape *shape) {
    return convertColor(shape->getOutlineColor());
}

extern "C" float sfRectangleShape_getOutlineThickness(const sf::RectangleShape *shape) {
    return shape->getOutlineThickness();
}

extern "C" size_t sfRectangleShape_getPointCount(const sf::RectangleShape *shape) {
    return shape->getPointCount();
}

extern "C" sfVector2f sfRectangleShape_getPoint(const sf::RectangleShape *shape, size_t index) {
    return convertVector2(shape->getPoint(index));
}

extern "C" sfVector2f sfRectangleShape_getGeometricCenter(const sf::RectangleShape *shape) {
    return convertVector2(shape->getGeometricCenter());
}

extern "C" void sfRectangleShape_setSize(sf::RectangleShape *shape, sfVector2f size) {
    shape->setSize(convertVector2(size));
}

extern "C" sfVector2f sfRectangleShape_getSize(const sf::RectangleShape *shape) {
    return convertVector2(shape->getSize());
}

extern "C" sfFloatRect sfRectangleShape_getLocalBounds(const sf::RectangleShape *shape) {
    return convertRect(shape->getLocalBounds());
}

extern "C" sfFloatRect sfRectangleShape_getGlobalBounds(const sf::RectangleShape *shape) {
    return convertRect(shape->getGlobalBounds());
}
