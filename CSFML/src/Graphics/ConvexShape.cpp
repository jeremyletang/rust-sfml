#include "Graphics/Color.hpp"
#include "Graphics/Rect.hpp"
#include "System/Vector2.hpp"
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/ConvexShape.hpp>
#include <cstddef>

extern "C" sf::ConvexShape *sfConvexShape_new(void) {
    return new sf::ConvexShape;
}

extern "C" sf::ConvexShape *sfConvexShape_cpy(const sf::ConvexShape *shape) {
    return new sf::ConvexShape(*shape);
}

extern "C" void sfConvexShape_del(sf::ConvexShape *shape) {
    delete shape;
}

extern "C" void sfConvexShape_setPosition(sf::ConvexShape *shape, sfVector2f position) {
    shape->setPosition(convertVector2(position));
}

extern "C" void sfConvexShape_setRotation(sf::ConvexShape *shape, float angle) {
    shape->setRotation(sf::degrees(angle));
}

extern "C" void sfConvexShape_setScale(sf::ConvexShape *shape, sfVector2f scale) {
    shape->setScale(convertVector2(scale));
}

extern "C" void sfConvexShape_setOrigin(sf::ConvexShape *shape, sfVector2f origin) {
    shape->setOrigin(convertVector2(origin));
}

extern "C" sfVector2f sfConvexShape_getPosition(const sf::ConvexShape *shape) {
    return convertVector2(shape->getPosition());
}

extern "C" float sfConvexShape_getRotation(const sf::ConvexShape *shape) {
    return shape->getRotation().asDegrees();
}

extern "C" sfVector2f sfConvexShape_getScale(const sf::ConvexShape *shape) {
    return convertVector2(shape->getScale());
}

extern "C" sfVector2f sfConvexShape_getOrigin(const sf::ConvexShape *shape) {
    return convertVector2(shape->getOrigin());
}

extern "C" void sfConvexShape_move(sf::ConvexShape *shape, sfVector2f offset) {
    shape->move(convertVector2(offset));
}

extern "C" void sfConvexShape_rotate(sf::ConvexShape *shape, float angle) {
    shape->rotate(sf::degrees(angle));
}

extern "C" void sfConvexShape_scale(sf::ConvexShape *shape, sfVector2f factors) {
    shape->scale(convertVector2(factors));
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
    shape->setTextureRect(convertRect(rect));
}

extern "C" void sfConvexShape_setFillColor(sf::ConvexShape *shape, sfColor color) {
    shape->setFillColor(convertColor(color));
}

extern "C" void sfConvexShape_setOutlineColor(sf::ConvexShape *shape, sfColor color) {
    shape->setOutlineColor(convertColor(color));
}

extern "C" void sfConvexShape_setOutlineThickness(sf::ConvexShape *shape, float thickness) {
    shape->setOutlineThickness(thickness);
}

extern "C" const sf::Texture *sfConvexShape_getTexture(const sf::ConvexShape *shape) {
    return shape->getTexture();
}

extern "C" sfIntRect sfConvexShape_getTextureRect(const sf::ConvexShape *shape) {
    return convertRect(shape->getTextureRect());
}

extern "C" sfColor sfConvexShape_getFillColor(const sf::ConvexShape *shape) {
    return convertColor(shape->getFillColor());
}

extern "C" sfColor sfConvexShape_getOutlineColor(const sf::ConvexShape *shape) {
    return convertColor(shape->getOutlineColor());
}

extern "C" float sfConvexShape_getOutlineThickness(const sf::ConvexShape *shape) {
    return shape->getOutlineThickness();
}

extern "C" size_t sfConvexShape_getPointCount(const sf::ConvexShape *shape) {
    return shape->getPointCount();
}

extern "C" sfVector2f sfConvexShape_getPoint(const sf::ConvexShape *shape, size_t index) {
    return convertVector2(shape->getPoint(index));
}

extern "C" sfVector2f sfConvexShape_getGeometricCenter(const sf::ConvexShape *shape) {
    return convertVector2(shape->getGeometricCenter());
}

extern "C" void sfConvexShape_setPointCount(sf::ConvexShape *shape, size_t count) {
    shape->setPointCount(count);
}

extern "C" void sfConvexShape_setPoint(sf::ConvexShape *shape, size_t index, sfVector2f point) {
    shape->setPoint(index, convertVector2(point));
}

extern "C" sfFloatRect sfConvexShape_getLocalBounds(const sf::ConvexShape *shape) {
    return convertRect(shape->getLocalBounds());
}

extern "C" sfFloatRect sfConvexShape_getGlobalBounds(const sf::ConvexShape *shape) {
    return convertRect(shape->getGlobalBounds());
}
