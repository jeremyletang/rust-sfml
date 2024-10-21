#ifndef SFML_SYSTEM_INPUTSTREAMSTRUCT_H
#define SFML_SYSTEM_INPUTSTREAMSTRUCT_H

#include <SFML/System/InputStream.hpp>
#include <cstdint>

typedef int64_t (*sfInputStreamHelperReadCb)(void *data, int64_t size, void *userData);
typedef int64_t (*sfInputStreamHelperSeekCb)(int64_t position, void *userData);
typedef int64_t (*sfInputStreamHelperTellCb)(void *userData);
typedef int64_t (*sfInputStreamHelperGetSizeCb)(void *userData);

struct sfInputStreamHelper final : public sf::InputStream {
    sfInputStreamHelper(sfInputStreamHelperReadCb read,
                        sfInputStreamHelperSeekCb seek,
                        sfInputStreamHelperTellCb tell,
                        sfInputStreamHelperGetSizeCb getSize, void *userData);
    virtual sf::Int64 read(void *data, sf::Int64 size) final {
        return readCb(data, size, userData);
    }

    virtual sf::Int64 seek(sf::Int64 position) final {
        return seekCb(position, userData);
    }

    virtual sf::Int64 tell() final {
        return tellCb(userData);
    }

    virtual sf::Int64 getSize() final {
        return getSizeCb(userData);
    }
    sfInputStreamHelperReadCb readCb;
    sfInputStreamHelperSeekCb seekCb;
    sfInputStreamHelperTellCb tellCb;
    sfInputStreamHelperGetSizeCb getSizeCb;
    void *userData;
};

#endif
