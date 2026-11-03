#ifndef SFML_SYSTEM_INPUTSTREAMSTRUCT_H
#define SFML_SYSTEM_INPUTSTREAMSTRUCT_H

#include <SFML/System/InputStream.hpp>
#include <cstdint>
#include <optional>

typedef int64_t (*sfInputStreamHelperReadCb)(void *data, int64_t size, void *userData);
typedef int64_t (*sfInputStreamHelperSeekCb)(int64_t position, void *userData);
typedef int64_t (*sfInputStreamHelperTellCb)(void *userData);
typedef int64_t (*sfInputStreamHelperGetSizeCb)(void *userData);

// A negative callback result signals an error. In particular, -1 must not
// become SIZE_MAX and be mistaken for a successful huge value.
[[nodiscard]] inline std::optional<std::size_t> convertInputStreamResult(int64_t value) {
    if (value < 0)
        return std::nullopt;
    return static_cast<std::size_t>(value);
}

struct sfInputStreamHelper final : public sf::InputStream {
    sfInputStreamHelper(sfInputStreamHelperReadCb read,
                        sfInputStreamHelperSeekCb seek,
                        sfInputStreamHelperTellCb tell,
                        sfInputStreamHelperGetSizeCb getSize, void *userData);
    virtual std::optional<std::size_t> read(void *data, std::size_t size) final {
        return convertInputStreamResult(readCb(data, static_cast<int64_t>(size), userData));
    }

    virtual std::optional<std::size_t> seek(std::size_t position) final {
        return convertInputStreamResult(seekCb(static_cast<int64_t>(position), userData));
    }

    virtual std::optional<std::size_t> tell() final {
        return convertInputStreamResult(tellCb(userData));
    }

    virtual std::optional<std::size_t> getSize() final {
        return convertInputStreamResult(getSizeCb(userData));
    }
    sfInputStreamHelperReadCb readCb;
    sfInputStreamHelperSeekCb seekCb;
    sfInputStreamHelperTellCb tellCb;
    sfInputStreamHelperGetSizeCb getSizeCb;
    void *userData;
};

#endif
