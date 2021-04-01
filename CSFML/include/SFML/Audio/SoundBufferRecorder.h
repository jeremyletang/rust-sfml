////////////////////////////////////////////////////////////
//
// SFML - Simple and Fast Multimedia Library
// Copyright (C) 2007-2018 Laurent Gomila (laurent@sfml-dev.org)
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it freely,
// subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented;
//    you must not claim that you wrote the original software.
//    If you use this software in a product, an acknowledgment
//    in the product documentation would be appreciated but is not required.
//
// 2. Altered source versions must be plainly marked as such,
//    and must not be misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//
////////////////////////////////////////////////////////////

#ifndef SFML_SOUNDBUFFERRECORDER_H
#define SFML_SOUNDBUFFERRECORDER_H

////////////////////////////////////////////////////////////
// Headers
////////////////////////////////////////////////////////////
#include <SFML/Audio/Export.h>
#include <SFML/Audio/Types.h>


////////////////////////////////////////////////////////////
/// \brief Create a new sound buffer recorder
///
/// \return A new sfSoundBufferRecorder object (NULL if failed)
///
////////////////////////////////////////////////////////////
CSFML_AUDIO_API sfSoundBufferRecorder* sfSoundBufferRecorder_create(void);

////////////////////////////////////////////////////////////
/// \brief Destroy a sound buffer recorder
///
/// \param soundBufferRecorder Sound buffer recorder to destroy
///
////////////////////////////////////////////////////////////
CSFML_AUDIO_API void sfSoundBufferRecorder_destroy(sfSoundBufferRecorder* soundBufferRecorder);

////////////////////////////////////////////////////////////
/// \brief Start the capture of a sound recorder recorder
///
/// The \a sampleRate parameter defines the number of audio samples
/// captured per second. The higher, the better the quality
/// (for example, 44100 samples/sec is CD quality).
/// This function uses its own thread so that it doesn't block
/// the rest of the program while the capture runs.
/// Please note that only one capture can happen at the same time.
///
/// \param soundBufferRecorder Sound buffer recorder object
/// \param sampleRate          Desired capture rate, in number of samples per second
///
/// \return sfTrue, if it was able to start recording
///
////////////////////////////////////////////////////////////
CSFML_AUDIO_API sfBool sfSoundBufferRecorder_start(sfSoundBufferRecorder* soundBufferRecorder, unsigned int sampleRate);

////////////////////////////////////////////////////////////
/// \brief Stop the capture of a sound recorder
///
/// \param soundBufferRecorder Sound buffer recorder object
///
////////////////////////////////////////////////////////////
CSFML_AUDIO_API void sfSoundBufferRecorder_stop(sfSoundBufferRecorder* soundBufferRecorder);

////////////////////////////////////////////////////////////
/// \brief Get the sample rate of a sound buffer recorder
///
/// The sample rate defines the number of audio samples
/// captured per second. The higher, the better the quality
/// (for example, 44100 samples/sec is CD quality).
///
/// \param soundBufferRecorder Sound buffer recorder object
///
/// \return Sample rate, in samples per second
///
////////////////////////////////////////////////////////////
CSFML_AUDIO_API unsigned int sfSoundBufferRecorder_getSampleRate(const sfSoundBufferRecorder* soundBufferRecorder);

////////////////////////////////////////////////////////////
/// \brief Get the sound buffer containing the captured audio data
///
/// The sound buffer is valid only after the capture has ended.
/// This function provides a read-only access to the internal
/// sound buffer, but it can be copied if you need to
/// make any modification to it.
///
/// \param soundBufferRecorder Sound buffer recorder object
///
/// \return Read-only access to the sound buffer
///
////////////////////////////////////////////////////////////
CSFML_AUDIO_API const sfSoundBuffer* sfSoundBufferRecorder_getBuffer(const sfSoundBufferRecorder* soundBufferRecorder);

////////////////////////////////////////////////////////////
/// \brief Set the audio capture device
///
/// This function sets the audio capture device to the device
/// with the given name. It can be called on the fly (i.e:
/// while recording). If you do so while recording and
/// opening the device fails, it stops the recording.
///
/// \param soundBufferRecorder Sound buffer recorder object
/// \param name                The name of the audio capture device
///
/// \return sfTrue, if it was able to set the requested device
///
////////////////////////////////////////////////////////////
CSFML_AUDIO_API sfBool sfSoundBufferRecorder_setDevice(sfSoundBufferRecorder* soundBufferRecorder, const char* name);

////////////////////////////////////////////////////////////
/// \brief Get the name of the current audio capture device
///
/// \param soundBufferRecorder Sound buffer recorder object
///
/// \return The name of the current audio capture device
///
////////////////////////////////////////////////////////////
CSFML_AUDIO_API const char* sfSoundBufferRecorder_getDevice(sfSoundBufferRecorder* soundBufferRecorder);


#endif // SFML_SOUNDBUFFERRECORDER_H
