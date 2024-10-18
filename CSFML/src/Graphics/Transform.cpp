#include "Graphics/Rect.h"
#include "System/Vector2.h"
#include <SFML/Graphics/Transform.hpp>
#include <cstring>

extern "C" sfVector2f sfTransform_transformPoint(const sf::Transform *transform, sfVector2f point) {
    sf::Vector2f vec2 = transform->transformPoint(point.x, point.y);
    return {vec2.x, vec2.y};
}

extern "C" sfFloatRect sfTransform_transformRect(const sf::Transform *transform, sfFloatRect rectangle) {
    sf::FloatRect rect = transform->transformRect(sf::FloatRect(rectangle.left, rectangle.top, rectangle.width, rectangle.height));
    return {rect.left, rect.top, rect.width, rect.height};
}

extern "C" void sfTransform_combine(sf::Transform *transform, const sf::Transform *other) {
    transform->combine(*other);
}

extern "C" void sfTransform_translate(sf::Transform *transform, float x, float y) {
    transform->translate(x, y);
}

extern "C" void sfTransform_rotate(sf::Transform *transform, float angle) {
    transform->rotate(angle);
}

extern "C" void sfTransform_rotateWithCenter(sf::Transform *transform, float angle, float centerX, float centerY) {
    transform->rotate(angle, centerX, centerY);
}

extern "C" void sfTransform_scale(sf::Transform *transform, float scaleX, float scaleY) {
    transform->scale(scaleX, scaleY);
}

extern "C" void sfTransform_scaleWithCenter(sf::Transform *transform, float scaleX, float scaleY, float centerX, float centerY) {
    transform->scale(scaleX, scaleY, centerX, centerY);
}
