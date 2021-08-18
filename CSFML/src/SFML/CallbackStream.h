
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


#ifndef SFML_CALLBACKSTREAM_H
#define SFML_CALLBACKSTREAM_H


// Headers

#include <SFML/System/InputStream.hpp>
#include <SFML/System/InputStream.h>



/// \brief Adapts a CSFML input stream to a SFML input stream
///

class CallbackStream : public sf::InputStream
{
public:

    
    /// \brief Default constructor
    ///
    
    CallbackStream()
    {
    }

    
    /// \brief Constructor
    ///
    /// \param stream CSFML input stream callbacks
    ///
    
    CallbackStream(sfInputStream* stream) :
    myStream(*stream)
    {
    }

    
    /// \brief Read data from the stream
    ///
    /// \param data Buffer where to copy the read data
    /// \param size Desired number of bytes to read
    ///
    /// \return The number of bytes actually read
    ///
    
    virtual sf::Int64 read(void* data, sf::Int64 size)
    {
        return myStream.read ? myStream.read(data, size, myStream.userData) : -1;
    }

    
    /// \brief Change the current reading position
    ///
    /// \param position The position to seek to, from the beginning
    ///
    /// \return The position actually seeked to, or -1 on error
    ///
    
    virtual sf::Int64 seek(sf::Int64 position)
    {
        return myStream.seek ? myStream.seek(position, myStream.userData) : -1;
    }

    
    /// \brief Return the current reading position in the stream
    ///
    /// \return The current position, or -1 on error.
    ///
    
    virtual sf::Int64 tell()
    {
        return myStream.tell ? myStream.tell(myStream.userData) : -1;
    }

    
    /// \brief Return the size of the stream
    ///
    /// \return The total number of bytes available in the stream, or -1 on error
    ///
    
    virtual sf::Int64 getSize()
    {
        return myStream.getSize ? myStream.getSize(myStream.userData) : -1;
    }

private:

    
    // Member data
    
    sfInputStream myStream; ///< The source CSFML input stream
};


#endif // SFML_CALLBACKSTREAM_H
