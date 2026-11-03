#include <SFML/Audio/PlaybackDevice.hpp>

#include <cstdint>
#include <optional>
#include <string>
#include <vector>

typedef void (*sfPlaybackDeviceNotificationCallback)(int notification, void *userData);

extern "C" std::vector<std::string> *sfPlaybackDevice_getAvailableDevices(void) {
    return new std::vector<std::string>(sf::PlaybackDevice::getAvailableDevices());
}

extern "C" std::string *sfPlaybackDevice_getDefaultDevice(void) {
    const auto name = sf::PlaybackDevice::getDefaultDevice();
    return name ? new std::string(*name) : nullptr;
}

extern "C" std::string *sfPlaybackDevice_getDevice(void) {
    const auto name = sf::PlaybackDevice::getDevice();
    return name ? new std::string(*name) : nullptr;
}

extern "C" bool sfPlaybackDevice_setDevice(const char *name) {
    return sf::PlaybackDevice::setDevice(name);
}

extern "C" bool sfPlaybackDevice_setDeviceToDefault(void) {
    return sf::PlaybackDevice::setDeviceToDefault();
}

extern "C" bool sfPlaybackDevice_setDeviceToNull(void) {
    return sf::PlaybackDevice::setDeviceToNull();
}

extern "C" bool sfPlaybackDevice_isDefaultDevice(void) {
    return sf::PlaybackDevice::isDefaultDevice();
}

extern "C" bool sfPlaybackDevice_getDeviceSampleRate(uint32_t *sampleRate) {
    const auto rate = sf::PlaybackDevice::getDeviceSampleRate();
    if (!rate)
        return false;
    *sampleRate = *rate;
    return true;
}

extern "C" void sfPlaybackDevice_setNotificationCallback(sfPlaybackDeviceNotificationCallback callback,
                                                         void *userData) {
    if (!callback) {
        sf::PlaybackDevice::setNotificationCallback(nullptr);
        return;
    }
    sf::PlaybackDevice::setNotificationCallback(
        [callback, userData](sf::PlaybackDevice::Notification notification) {
            callback(static_cast<int>(notification), userData);
        });
}
