#include <cstddef>
#include <string>

extern "C" void sfStdString_del(std::string *s) {
    delete s;
}

extern "C" size_t sfStdString_getLength(const std::string *s) {
    return s->size();
}

extern "C" const char *sfStdString_getData(const std::string *s) {
    return s->data();
}
