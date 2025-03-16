#ifndef SFML_SYSTEM_INPUTSTREAMSTRUCT_H
#define SFML_SYSTEM_INPUTSTREAMSTRUCT_H

#include <SFML/System/InputStream.hpp>
#include <cstdint>

typedef int64_t (*sfInputStreamHelperReadCb)(void *data, size_t size, void *userData);
typedef int64_t (*sfInputStreamHelperSeekCb)(size_t position, void *userData);
typedef int64_t (*sfInputStreamHelperTellCb)(void *userData);
typedef int64_t (*sfInputStreamHelperGetSizeCb)(void *userData);

struct sfInputStreamHelper final : public sf::InputStream {
    sfInputStreamHelper(sfInputStreamHelperReadCb read,
                        sfInputStreamHelperSeekCb seek,
                        sfInputStreamHelperTellCb tell,
                        sfInputStreamHelperGetSizeCb getSize, void *userData);
    virtual std::optional<std::size_t> read(void *data, std::size_t size) final {
        return readCb(data, size, userData);
    }

    virtual std::optional<std::size_t> seek(std::size_t position) final {
        return seekCb(position, userData);
    }

    virtual std::optional<std::size_t> tell() final {
        return tellCb(userData);
    }

    virtual std::optional<std::size_t> getSize() final {
        return getSizeCb(userData);
    }
    sfInputStreamHelperReadCb readCb;
    sfInputStreamHelperSeekCb seekCb;
    sfInputStreamHelperTellCb tellCb;
    sfInputStreamHelperGetSizeCb getSizeCb;
    void *userData;
};

#endif
