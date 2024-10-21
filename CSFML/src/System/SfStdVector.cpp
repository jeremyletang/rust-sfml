#include <string>
#include <vector>

extern "C" void sfStdStringVector_del(std::vector<std::string> *vec) {
    delete vec;
}

extern "C" std::size_t sfStdStringVector_getLength(const std::vector<std::string> *vec) {
    return vec->size();
}

extern "C" const std::string *sfStdStringVector_index(const std::vector<std::string> *vec, std::size_t index) {
    return &(*vec)[index];
}
