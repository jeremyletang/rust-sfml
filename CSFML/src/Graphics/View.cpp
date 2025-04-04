#include "Graphics/Rect.hpp"
#include "System/Vector2.hpp"
#include <SFML/Graphics/View.hpp>

extern "C" sf::View *sfView_new(void) {
    return new sf::View;
}

extern "C" void sfView_del(sf::View *view) {
    delete view;
}

extern "C" sf::View *sfView_cpy(const sf::View *view) {
    return new sf::View(*view);
}

extern "C" void sfView_setCenter(sf::View *view, sfVector2f center) {
    view->setCenter({center.x, center.y});
}

extern "C" void sfView_setSize(sf::View *view, sfVector2f size) {
    view->setSize({size.x, size.y});
}

extern "C" void sfView_setRotation(sf::View *view, float angle) {
    view->setRotation(sf::degrees(angle));
}

extern "C" void sfView_setViewport(sf::View *view, sfFloatRect viewport) {
    view->setViewport({{viewport.position.x, viewport.position.y}, {viewport.size.x, viewport.size.y}});
}

extern "C" sfVector2f sfView_getCenter(const sf::View *view) {
    sf::Vector2f vec2 = view->getCenter();
    return {vec2.x, vec2.y};
}

extern "C" sfVector2f sfView_getSize(const sf::View *view) {
    sf::Vector2f vec2 = view->getSize();
    return {vec2.x, vec2.y};
}

extern "C" float sfView_getRotation(const sf::View *view) {
    return view->getRotation().asDegrees();
}

extern "C" sfFloatRect sfView_getViewport(const sf::View *view) {
    sf::FloatRect rect = view->getViewport();
    return {{rect.position.x, rect.position.y}, {rect.size.x, rect.size.y}};
}

extern "C" void sfView_move(sf::View *view, sfVector2f offset) {
    view->move({offset.x, offset.y});
}

extern "C" void sfView_rotate(sf::View *view, float angle) {
    view->rotate(sf::degrees(angle));
}

extern "C" void sfView_zoom(sf::View *view, float factor) {
    view->zoom(factor);
}
