#ifndef SFML_SYSTEM_INPUTSTREAMSTRUCT_H
#define SFML_SYSTEM_INPUTSTREAMSTRUCT_H

#include "System/InputStream.h"
#include <SFML/System/InputStream.hpp>

struct sfInputStream : public sf::InputStream {
    virtual sf::Int64 read(void *data, sf::Int64 size) {
        return readFun(data, size, userData);
    }

    virtual sf::Int64 seek(sf::Int64 position) {
        return seekFun(position, userData);
    }

    virtual sf::Int64 tell() {
        return tellFun(userData);
    }

    virtual sf::Int64 getSize() {
        return getSizeFun(userData);
    }
    sfInputStreamReadFunc readFun;
    sfInputStreamSeekFunc seekFun;
    sfInputStreamTellFunc tellFun;
    sfInputStreamGetSizeFunc getSizeFun;
    void *userData;
};

#endif
