#include "Graphics/Rect.hpp"
#include "System/Vector2.hpp"
#include <SFML/Graphics/Transform.hpp>

extern "C" sfVector2f sfTransform_transformPoint(const sf::Transform *transform, sfVector2f point) {
    return convertVector2(transform->transformPoint(convertVector2(point)));
}

extern "C" sfFloatRect sfTransform_transformRect(const sf::Transform *transform, sfFloatRect rectangle) {
    return convertRect(transform->transformRect(convertRect(rectangle)));
}

extern "C" void sfTransform_combine(sf::Transform *transform, const sf::Transform *other) {
    transform->combine(*other);
}

extern "C" void sfTransform_translate(sf::Transform *transform, sfVector2f offset) {
    transform->translate(convertVector2(offset));
}

extern "C" void sfTransform_rotate(sf::Transform *transform, float angle) {
    transform->rotate(sf::degrees(angle));
}

extern "C" void sfTransform_rotateWithCenter(sf::Transform *transform, float angle, sfVector2f center) {
    transform->rotate(sf::degrees(angle), convertVector2(center));
}

extern "C" void sfTransform_scale(sf::Transform *transform, sfVector2f scale) {
    transform->scale(convertVector2(scale));
}

extern "C" void sfTransform_scaleWithCenter(sf::Transform *transform, sfVector2f scale, sfVector2f center) {
    transform->scale(convertVector2(scale), convertVector2(center));
}
