#include "System/Buffer.hpp"
#include <cstddef>
#include <cstdint>

extern "C" void sfBuffer_destroy(sfBuffer *buffer) {
    delete buffer;
}

extern "C" const uint8_t *sfBuffer_getData(const sfBuffer *buffer) {
    return buffer->data();
}

extern "C" std::size_t sfBuffer_getSize(const sfBuffer *buffer) {
    return buffer->size();
}
