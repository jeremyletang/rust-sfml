#ifndef SFML_SYSTEM_INPUTSTREAMSTRUCT_H
#define SFML_SYSTEM_INPUTSTREAMSTRUCT_H

#include <SFML/System/InputStream.hpp>
#include <cstdint>

typedef int64_t (*sfInputStreamReadFunc)(void *data, int64_t size, void *userData);
typedef int64_t (*sfInputStreamSeekFunc)(int64_t position, void *userData);
typedef int64_t (*sfInputStreamTellFunc)(void *userData);
typedef int64_t (*sfInputStreamGetSizeFunc)(void *userData);

struct sfInputStreamHelper : public sf::InputStream {
    sfInputStreamHelper(sfInputStreamReadFunc read,
                        sfInputStreamSeekFunc seek,
                        sfInputStreamTellFunc tell,
                        sfInputStreamGetSizeFunc getSize, void *userData);
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
