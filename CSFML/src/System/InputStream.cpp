#include "System/InputStreamStruct.h"

extern "C" sfInputStream *sfInputStream_new(
    sfInputStreamReadFunc read,
    sfInputStreamSeekFunc seek,
    sfInputStreamTellFunc tell,
    sfInputStreamGetSizeFunc getSize,
    void *userData) {
    sfInputStream *stream = new sfInputStream;
    stream->readFun = read;
    stream->seekFun = seek;
    stream->tellFun = tell;
    stream->getSizeFun = getSize;
    stream->userData = userData;
    return stream;
}

extern "C" void sfInputStream_destroy(sfInputStream *stream) {
    delete stream;
}