#include <cstddef>
#include <string>

extern "C" size_t sfStdString_getLength(const std::string *s) {
    return s->size();
}

extern "C" const char *sfStdString_getData(const std::string *s) {
    return s->data();
}

extern "C" void sfStdString_destroy(std::string *s) {
    delete s;
}
