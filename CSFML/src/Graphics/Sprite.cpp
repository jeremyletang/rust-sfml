#include "Graphics/Color.hpp"
#include "Graphics/Rect.hpp"
#include "SFML/System/Vector2.hpp"
#include "System/Vector2.hpp"
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/Image.hpp>
#include <SFML/Graphics/Sprite.hpp>

extern "C" sf::Sprite *sfSprite_new(const sf::Texture *texture, const sfIntRect rect) {
    return new sf::Sprite(*texture, convertRect(rect));
}

extern "C" sf::Sprite *sfSprite_cpy(const sf::Sprite *sprite) {
    return new sf::Sprite(*sprite);
}

extern "C" void sfSprite_del(sf::Sprite *sprite) {
    delete sprite;
}

extern "C" void sfSprite_setPosition(sf::Sprite *sprite, sfVector2f position) {
    sprite->setPosition(convertVector2(position));
}

extern "C" void sfSprite_setRotation(sf::Sprite *sprite, float angle) {
    sprite->setRotation(sf::degrees(angle));
}

extern "C" void sfSprite_setScale(sf::Sprite *sprite, sfVector2f scale) {
    sprite->setScale(convertVector2(scale));
}

extern "C" void sfSprite_setOrigin(sf::Sprite *sprite, sfVector2f origin) {
    sprite->setOrigin(convertVector2(origin));
}

extern "C" sfVector2f sfSprite_getPosition(const sf::Sprite *sprite) {
    return convertVector2(sprite->getPosition());
}

extern "C" float sfSprite_getRotation(const sf::Sprite *sprite) {
    return sprite->getRotation().asDegrees();
}

extern "C" sfVector2f sfSprite_getScale(const sf::Sprite *sprite) {
    return convertVector2(sprite->getScale());
}

extern "C" sfVector2f sfSprite_getOrigin(const sf::Sprite *sprite) {
    return convertVector2(sprite->getOrigin());
}

extern "C" void sfSprite_move(sf::Sprite *sprite, sfVector2f offset) {
    sprite->move(convertVector2(offset));
}

extern "C" void sfSprite_rotate(sf::Sprite *sprite, float angle) {
    sprite->rotate(sf::degrees(angle));
}

extern "C" void sfSprite_scale(sf::Sprite *sprite, sfVector2f factors) {
    sprite->scale(convertVector2(factors));
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

extern "C" void sfSprite_setTextureRect(sf::Sprite *sprite, sfIntRect rect) {
    sprite->setTextureRect(convertRect(rect));
}

extern "C" void sfSprite_setColor(sf::Sprite *sprite, sfColor color) {
    sprite->setColor(convertColor(color));
}

extern "C" const sf::Texture *sfSprite_getTexture(const sf::Sprite *sprite) {
    return &sprite->getTexture();
}

extern "C" sfIntRect sfSprite_getTextureRect(const sf::Sprite *sprite) {
    return convertRect(sprite->getTextureRect());
}

extern "C" sfColor sfSprite_getColor(const sf::Sprite *sprite) {
    return convertColor(sprite->getColor());
}

extern "C" sfFloatRect sfSprite_getLocalBounds(const sf::Sprite *sprite) {
    return convertRect(sprite->getLocalBounds());
}

extern "C" sfFloatRect sfSprite_getGlobalBounds(const sf::Sprite *sprite) {
    return convertRect(sprite->getGlobalBounds());
}
