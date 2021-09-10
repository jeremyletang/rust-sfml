#include "System/Types.h"
#include <cstddef>
#include <string>

extern "C" size_t sfStdString_getLength(const sfStdString *s) {
    return reinterpret_cast<const std::string *>(s)->size();
}

extern "C" const char *sfStdString_getData(const sfStdString *s) {
    return reinterpret_cast<const std::string *>(s)->data();
}
