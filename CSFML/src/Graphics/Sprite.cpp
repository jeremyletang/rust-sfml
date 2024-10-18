#include "Graphics/Color.h"
#include "Graphics/Rect.h"
#include "System/Vector2.h"
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/Image.hpp>
#include <SFML/Graphics/Sprite.hpp>

extern "C" sf::Sprite *sfSprite_create(void) {
    return new sf::Sprite;
}

extern "C" sf::Sprite *sfSprite_copy(const sf::Sprite *sprite) {
    return new sf::Sprite(*sprite);
}

extern "C" void sfSprite_destroy(sf::Sprite *sprite) {
    delete sprite;
}

extern "C" void sfSprite_setPosition(sf::Sprite *sprite, sfVector2f position) {
    sprite->setPosition(position.x, position.y);
}

extern "C" void sfSprite_setRotation(sf::Sprite *sprite, float angle) {
    sprite->setRotation(angle);
}

extern "C" void sfSprite_setScale(sf::Sprite *sprite, sfVector2f scale) {
    sprite->setScale(scale.x, scale.y);
}

extern "C" void sfSprite_setOrigin(sf::Sprite *sprite, sfVector2f origin) {
    sprite->setOrigin(origin.x, origin.y);
}

extern "C" sfVector2f sfSprite_getPosition(const sf::Sprite *sprite) {
    sf::Vector2f vec2 = sprite->getPosition();
    return {vec2.x, vec2.y};
}

extern "C" float sfSprite_getRotation(const sf::Sprite *sprite) {
    return sprite->getRotation();
}

extern "C" sfVector2f sfSprite_getScale(const sf::Sprite *sprite) {
    sf::Vector2f vec2 = sprite->getScale();
    return {vec2.x, vec2.y};
}

extern "C" sfVector2f sfSprite_getOrigin(const sf::Sprite *sprite) {
    sf::Vector2f vec2 = sprite->getOrigin();
    return {vec2.x, vec2.y};
}

extern "C" void sfSprite_move(sf::Sprite *sprite, sfVector2f offset) {
    sprite->move(offset.x, offset.y);
}

extern "C" void sfSprite_rotate(sf::Sprite *sprite, float angle) {
    sprite->rotate(angle);
}

extern "C" void sfSprite_scale(sf::Sprite *sprite, sfVector2f factors) {
    sprite->scale(factors.x, factors.y);
}

extern "C" sf::Transform const *sfSprite_getTransform(const sf::Sprite *sprite) {
    return &sprite->getTransform();
}

extern "C" sf::Transform const *sfSprite_getInverseTransform(const sf::Sprite *sprite) {
    return &sprite->getInverseTransform();
}

extern "C" void sfSprite_setTexture(sf::Sprite *sprite, const sf::Texture *texture, bool resetRect) {
    sprite->setTexture(*texture, resetRect);
}

extern "C" void sfSprite_setTextureRect(sf::Sprite *sprite, sfIntRect rectangle) {
    sprite->setTextureRect(sf::IntRect(rectangle.left, rectangle.top, rectangle.width, rectangle.height));
}

extern "C" void sfSprite_setColor(sf::Sprite *sprite, sfColor color) {
    sprite->setColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" const sf::Texture *sfSprite_getTexture(const sf::Sprite *sprite) {
    return sprite->getTexture();
}

extern "C" sfIntRect sfSprite_getTextureRect(const sf::Sprite *sprite) {
    sf::IntRect rect = sprite->getTextureRect();
    return {rect.left, rect.top, rect.width, rect.height};
}

extern "C" sfColor sfSprite_getColor(const sf::Sprite *sprite) {
    sf::Color color = sprite->getColor();
    return {color.r, color.g, color.b, color.a};
}

extern "C" sfFloatRect sfSprite_getLocalBounds(const sf::Sprite *sprite) {
    sf::FloatRect rect = sprite->getLocalBounds();
    return {rect.left, rect.top, rect.width, rect.height};
}

extern "C" sfFloatRect sfSprite_getGlobalBounds(const sf::Sprite *sprite) {
    sf::FloatRect rect = sprite->getGlobalBounds();
    return {rect.left, rect.top, rect.width, rect.height};
}
