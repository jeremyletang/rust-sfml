#include "System/InputStreamHelper.hpp"

sfInputStreamHelper::sfInputStreamHelper(sfInputStreamHelperReadCb read,
                                         sfInputStreamHelperSeekCb seek,
                                         sfInputStreamHelperTellCb tell,
                                         sfInputStreamHelperGetSizeCb getSize, void *userData) : readCb(read), seekCb(seek), tellCb(tell), getSizeCb(getSize), userData(userData) {
}

extern "C" sfInputStreamHelper *sfInputStreamHelper_new(
    sfInputStreamHelperReadCb read,
    sfInputStreamHelperSeekCb seek,
    sfInputStreamHelperTellCb tell,
    sfInputStreamHelperGetSizeCb getSize,
    void *userData) {
    return new sfInputStreamHelper(read, seek, tell, getSize, userData);
}

extern "C" void sfInputStreamHelper_del(sfInputStreamHelper *stream) {
    delete stream;
}