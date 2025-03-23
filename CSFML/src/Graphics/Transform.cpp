#include "Graphics/Rect.hpp"
#include "System/Vector2.hpp"
#include <SFML/Graphics/Transform.hpp>
#include <cstring>

extern "C" sfVector2f sfTransform_transformPoint(const sf::Transform *transform, sfVector2f point) {
    sf::Vector2f vec2 = transform->transformPoint({point.x, point.y});
    return {vec2.x, vec2.y};
}

extern "C" sfFloatRect sfTransform_transformRect(const sf::Transform *transform, sfFloatRect rectangle) {
    sf::FloatRect rect = transform->transformRect({{rectangle.position.x, rectangle.position.y}, {rectangle.size.x, rectangle.size.y}});
    return {{rect.position.x, rect.position.y}, {rect.size.x, rect.size.y}};
}

extern "C" void sfTransform_combine(sf::Transform *transform, const sf::Transform *other) {
    transform->combine(*other);
}

extern "C" void sfTransform_translate(sf::Transform *transform, sfVector2f offset) {
    transform->translate({offset.x, offset.y});
}

extern "C" void sfTransform_rotate(sf::Transform *transform, float angle) {
    transform->rotate(sf::degrees(angle));
}

extern "C" void sfTransform_rotateWithCenter(sf::Transform *transform, float angle, sfVector2f center) {
    transform->rotate(sf::degrees(angle), {center.x, center.y});
}

extern "C" void sfTransform_scale(sf::Transform *transform, sfVector2f scale) {
    transform->scale({scale.x, scale.y});
}

extern "C" void sfTransform_scaleWithCenter(sf::Transform *transform, sfVector2f scale, sfVector2f center) {
    transform->scale({scale.x, scale.y}, {center.x, center.y});
}
