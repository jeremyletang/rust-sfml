#include <string>
#include <vector>

extern "C" void sfStdStringVector_del(std::vector<std::string> *vec) {
    delete vec;
}

extern "C" const std::string *sfStdStringVector_getData(const std::vector<std::string> *vec) {
    return vec->data();
}

extern "C" std::size_t sfStdStringVector_getLength(const std::vector<std::string> *vec) {
    return vec->size();
}
