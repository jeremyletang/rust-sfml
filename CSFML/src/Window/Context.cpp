#include <SFML/Window/Context.hpp>
#include <cstdint>

extern "C" sf::Context *sfContext_new(void) {
    return new sf::Context;
}

extern "C" void sfContext_del(sf::Context *context) {
    delete context;
}

extern "C" bool sfContext_setActive(sf::Context *context, bool active) {
    return context->setActive(active);
}

extern "C" const sf::ContextSettings *sfContext_getSettings(const sf::Context *context) {
    return &context->getSettings();
}

extern "C" uint64_t sfContext_getActiveContextId() {
    return sf::Context::getActiveContextId();
}

extern "C" const sf::Context *sfContext_getActiveContext() {
    return sf::Context::getActiveContext();
}

extern "C" sf::GlFunctionPointer sfContext_getFunction(const char *name) {
    return sf::Context::getFunction(name);
}