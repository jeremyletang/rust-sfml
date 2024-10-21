#include "System/InputStreamHelper.hpp"

sfInputStreamHelper::sfInputStreamHelper(sfInputStreamReadFunc read,
                                         sfInputStreamSeekFunc seek,
                                         sfInputStreamTellFunc tell,
                                         sfInputStreamGetSizeFunc getSize, void *userData) : readFun(read), seekFun(seek), tellFun(tell), getSizeFun(getSize), userData(userData) {
}

extern "C" sfInputStreamHelper *sfInputStreamHelper_new(
    sfInputStreamReadFunc read,
    sfInputStreamSeekFunc seek,
    sfInputStreamTellFunc tell,
    sfInputStreamGetSizeFunc getSize,
    void *userData) {
    return new sfInputStreamHelper(read, seek, tell, getSize, userData);
}

extern "C" void sfInputStreamHelper_del(sfInputStreamHelper *stream) {
    delete stream;
}