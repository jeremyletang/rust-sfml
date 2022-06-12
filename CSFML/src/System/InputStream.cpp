#include "System/InputStream.h"
#include "System/InputStreamStruct.h"

sfInputStream *sfInputStream_new(
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

void sfInputStream_destroy(sfInputStream *stream) {
    delete stream;
}