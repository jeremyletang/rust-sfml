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

////////////////////////////////////////////////////////////
// Headers
////////////////////////////////////////////////////////////
#include <SFML/Window/Clipboard.h>
#include <SFML/Window/Clipboard.hpp>
#include <SFML/Internal.h>


namespace
{
    sf::String ClipboardString;
    std::string ClipboardStringAnsi;
}


////////////////////////////////////////////////////////////
const char* sfClipboard_getString()
{
    ClipboardStringAnsi = sf::Clipboard::getString().toAnsiString();
    return ClipboardStringAnsi.c_str();
}


////////////////////////////////////////////////////////////
const sfUint32* sfClipboard_getUnicodeString()
{
    ClipboardString = sf::Clipboard::getString();
    return ClipboardString.getData();
}


////////////////////////////////////////////////////////////
void sfClipboard_setString(const char* text)
{
    sf::Clipboard::setString(text);
}


////////////////////////////////////////////////////////////
void sfClipboard_setUnicodeString(const sfUint32* text)
{
    sf::Clipboard::setString(text);
}
