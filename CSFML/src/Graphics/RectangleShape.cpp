#include "Graphics/Color.hpp"
#include "Graphics/Rect.hpp"
#include "System/Vector2.hpp"
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/RectangleShape.hpp>
#include <cstddef>

extern "C" sf::RectangleShape *sfRectangleShape_create(void) {
    return new sf::RectangleShape;
}

extern "C" sf::RectangleShape *sfRectangleShape_copy(const sf::RectangleShape *shape) {
    return new sf::RectangleShape(*shape);
}

extern "C" void sfRectangleShape_destroy(sf::RectangleShape *shape) {
    delete shape;
}

extern "C" void sfRectangleShape_setPosition(sf::RectangleShape *shape, sfVector2f position) {
    shape->setPosition(position.x, position.y);
}

extern "C" void sfRectangleShape_setRotation(sf::RectangleShape *shape, float angle) {
    shape->setRotation(angle);
}

extern "C" void sfRectangleShape_setScale(sf::RectangleShape *shape, sfVector2f scale) {
    shape->setScale(scale.x, scale.y);
}

extern "C" void sfRectangleShape_setOrigin(sf::RectangleShape *shape, sfVector2f origin) {
    shape->setOrigin(origin.x, origin.y);
}

extern "C" sfVector2f sfRectangleShape_getPosition(const sf::RectangleShape *shape) {
    sf::Vector2f vec2 = shape->getPosition();
    return {vec2.x, vec2.y};
}

extern "C" float sfRectangleShape_getRotation(const sf::RectangleShape *shape) {
    return shape->getRotation();
}

extern "C" sfVector2f sfRectangleShape_getScale(const sf::RectangleShape *shape) {
    sf::Vector2f vec2 = shape->getScale();
    return {vec2.x, vec2.y};
}

extern "C" sfVector2f sfRectangleShape_getOrigin(const sf::RectangleShape *shape) {
    sf::Vector2f vec2 = shape->getOrigin();
    return {vec2.x, vec2.y};
}

extern "C" void sfRectangleShape_move(sf::RectangleShape *shape, sfVector2f offset) {
    shape->move(offset.x, offset.y);
}

extern "C" void sfRectangleShape_rotate(sf::RectangleShape *shape, float angle) {
    shape->rotate(angle);
}

extern "C" void sfRectangleShape_scale(sf::RectangleShape *shape, sfVector2f factors) {
    shape->scale(factors.x, factors.y);
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
    shape->setTextureRect(sf::IntRect(rect.left, rect.top, rect.width, rect.height));
}

extern "C" void sfRectangleShape_setFillColor(sf::RectangleShape *shape, sfColor color) {
    shape->setFillColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfRectangleShape_setOutlineColor(sf::RectangleShape *shape, sfColor color) {
    shape->setOutlineColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfRectangleShape_setOutlineThickness(sf::RectangleShape *shape, float thickness) {
    shape->setOutlineThickness(thickness);
}

extern "C" const sf::Texture *sfRectangleShape_getTexture(const sf::RectangleShape *shape) {
    return shape->getTexture();
}

extern "C" sfIntRect sfRectangleShape_getTextureRect(const sf::RectangleShape *shape) {
    sf::IntRect rect = shape->getTextureRect();
    return {rect.left, rect.top, rect.width, rect.height};
}

extern "C" sfColor sfRectangleShape_getFillColor(const sf::RectangleShape *shape) {
    sf::Color color = shape->getFillColor();
    return {color.r, color.g, color.b, color.a};
}

extern "C" sfColor sfRectangleShape_getOutlineColor(const sf::RectangleShape *shape) {
    sf::Color color = shape->getOutlineColor();
    return {color.r, color.g, color.b, color.a};
}

extern "C" float sfRectangleShape_getOutlineThickness(const sf::RectangleShape *shape) {
    return shape->getOutlineThickness();
}

extern "C" size_t sfRectangleShape_getPointCount(const sf::RectangleShape *shape) {
    return shape->getPointCount();
}

extern "C" sfVector2f sfRectangleShape_getPoint(const sf::RectangleShape *shape, size_t index) {
    sf::Vector2f vec2 = shape->getPoint(index);
    return {vec2.x, vec2.y};
}

extern "C" void sfRectangleShape_setSize(sf::RectangleShape *shape, sfVector2f size) {
    shape->setSize(sf::Vector2f(size.x, size.y));
}

extern "C" sfVector2f sfRectangleShape_getSize(const sf::RectangleShape *shape) {
    sf::Vector2f vec2 = shape->getSize();
    return {vec2.x, vec2.y};
}

extern "C" sfFloatRect sfRectangleShape_getLocalBounds(const sf::RectangleShape *shape) {
    sf::FloatRect rect = shape->getLocalBounds();
    return {rect.left, rect.top, rect.width, rect.height};
}

extern "C" sfFloatRect sfRectangleShape_getGlobalBounds(const sf::RectangleShape *shape) {
    sf::FloatRect rect = shape->getGlobalBounds();
    return {rect.left, rect.top, rect.width, rect.height};
}
