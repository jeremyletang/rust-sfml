#pragma once

#include <SFML/Audio/SoundSource.hpp>

typedef struct
{
    float innerAngle; //!< Inner angle, in degrees
    float outerAngle; //!< Outer angle, in degrees
    float outerGain;  //!< Outer gain
} sfSoundSourceCone;

////////////////////////////////////////////////////////////
// Convert sf::SoundSource::Cone to sfSoundSourceCone
////////////////////////////////////////////////////////////
[[nodiscard]] inline sfSoundSourceCone convertCone(const sf::SoundSource::Cone cone) {
    return {cone.innerAngle.asDegrees(), cone.outerAngle.asDegrees(), cone.outerGain};
}

////////////////////////////////////////////////////////////
// Convert sfSoundSourceCone to sf::SoundSource::Cone
////////////////////////////////////////////////////////////
[[nodiscard]] inline sf::SoundSource::Cone convertCone(const sfSoundSourceCone cone) {
    return {sf::degrees(cone.innerAngle), sf::degrees(cone.outerAngle), cone.outerGain};
}
