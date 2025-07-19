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
    view->setCenter(convertVector2(center));
}

extern "C" void sfView_setSize(sf::View *view, sfVector2f size) {
    view->setSize(convertVector2(size));
}

extern "C" void sfView_setRotation(sf::View *view, float angle) {
    view->setRotation(sf::degrees(angle));
}

extern "C" void sfView_setViewport(sf::View *view, sfFloatRect viewport) {
    view->setViewport(convertRect(viewport));
}

extern "C" void sfView_setScissor(sf::View *view, sfFloatRect scissor) {
    view->setScissor(convertRect(scissor));
}

extern "C" sfVector2f sfView_getCenter(const sf::View *view) {
    return convertVector2(view->getCenter());
}

extern "C" sfVector2f sfView_getSize(const sf::View *view) {
    return convertVector2(view->getSize());
}

extern "C" float sfView_getRotation(const sf::View *view) {
    return view->getRotation().asDegrees();
}

extern "C" sfFloatRect sfView_getViewport(const sf::View *view) {
    return convertRect(view->getViewport());
}

extern "C" sfFloatRect sfView_getScissor(const sf::View *view) {
    return convertRect(view->getScissor());
}

extern "C" void sfView_move(sf::View *view, sfVector2f offset) {
    view->move(convertVector2(offset));
}

extern "C" void sfView_rotate(sf::View *view, float angle) {
    view->rotate(sf::degrees(angle));
}

extern "C" void sfView_zoom(sf::View *view, float factor) {
    view->zoom(factor);
}
