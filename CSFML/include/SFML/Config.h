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

#ifndef SFML_CONFIG_H
#define SFML_CONFIG_H


////////////////////////////////////////////////////////////
// Define the CSFML version
////////////////////////////////////////////////////////////
#define CSFML_VERSION_MAJOR 2
#define CSFML_VERSION_MINOR 5
#define CSFML_VERSION_PATCH 0


////////////////////////////////////////////////////////////
// Check if we need to mark functions as extern "C"
////////////////////////////////////////////////////////////
#ifdef __cplusplus
    #define CSFML_EXTERN_C extern "C"
#else
    #define CSFML_EXTERN_C extern
#endif


////////////////////////////////////////////////////////////
// Identify the operating system
////////////////////////////////////////////////////////////
#if defined(_WIN32) || defined(__WIN32__)

    // Windows
    #define CSFML_SYSTEM_WINDOWS

#elif defined(linux) || defined(__linux)

    // Linux
    #define CSFML_SYSTEM_LINUX

#elif defined(__APPLE__) || defined(MACOSX) || defined(macintosh) || defined(Macintosh)

    // MacOS
    #define CSFML_SYSTEM_MACOS

#elif defined(__FreeBSD__) || defined(__FreeBSD_kernel__)

    // FreeBSD
    #define CSFML_SYSTEM_FREEBSD

#else

    // Unsupported system
    #error This operating system is not supported by SFML library

#endif


////////////////////////////////////////////////////////////
// Define helpers to create portable import / export macros for each module
////////////////////////////////////////////////////////////
#if defined(CSFML_SYSTEM_WINDOWS)

    // Windows compilers need specific (and different) keywords for export and import
    #define CSFML_API_EXPORT extern "C" __declspec(dllexport)
    #define CSFML_API_IMPORT CSFML_EXTERN_C __declspec(dllimport)

    // For Visual C++ compilers, we also need to turn off this annoying C4251 warning
    #ifdef _MSC_VER

        #pragma warning(disable : 4251)

    #endif

#else // Linux, FreeBSD, Mac OS X

    #if __GNUC__ >= 4

        // GCC 4 has special keywords for showing/hidding symbols,
        // the same keyword is used for both importing and exporting
        #define CSFML_API_EXPORT extern "C" __attribute__ ((__visibility__ ("default")))
        #define CSFML_API_IMPORT CSFML_EXTERN_C __attribute__ ((__visibility__ ("default")))

    #else

        // GCC < 4 has no mechanism to explicitely hide symbols, everything's exported
        #define CSFML_API_EXPORT extern "C"
        #define CSFML_API_IMPORT CSFML_EXTERN_C

    #endif

#endif

////////////////////////////////////////////////////////////
// Cross-platform warning for deprecated functions and classes
//
// Usage:
// struct CSFML_DEPRECATED MyStruct
// {
//     ...
// };
//
// CSFML_DEPRECATED void globalFunc();
////////////////////////////////////////////////////////////
#if defined(CSFML_NO_DEPRECATED_WARNINGS)

    // User explicitly requests to disable deprecation warnings
    #define CSFML_DEPRECATED

#elif defined(_MSC_VER)

    // Microsoft C++ compiler
    // Note: On newer MSVC versions, using deprecated functions causes a compiler error. In order to
    // trigger a warning instead of an error, the compiler flag /sdl- (instead of /sdl) must be specified.
    #define CSFML_DEPRECATED __declspec(deprecated)

#elif defined(__GNUC__)

    // g++ and Clang
    #define CSFML_DEPRECATED __attribute__ ((deprecated))

#else

    // Other compilers are not supported, leave class or function as-is.
    // With a bit of luck, the #pragma directive works, otherwise users get a warning (no error!) for unrecognized #pragma.
    #pragma message("CSFML_DEPRECATED is not supported for your compiler, please contact the CSFML team")
    #define CSFML_DEPRECATED

#endif

////////////////////////////////////////////////////////////
// Define a portable boolean type
////////////////////////////////////////////////////////////
typedef int sfBool;
#define sfFalse 0
#define sfTrue  1


////////////////////////////////////////////////////////////
// Define portable fixed-size types
////////////////////////////////////////////////////////////

// All "common" platforms use the same size for char, short and int
// (basically there are 3 types for 3 sizes, so no other match is possible),
// we can use them without doing any kind of check

// 8 bits integer types
typedef signed   char sfInt8;
typedef unsigned char sfUint8;

// 16 bits integer types
typedef signed   short sfInt16;
typedef unsigned short sfUint16;

// 32 bits integer types
typedef signed   int sfInt32;
typedef unsigned int sfUint32;

// 64 bits integer types
#if defined(_MSC_VER)
    typedef signed   __int64 sfInt64;
    typedef unsigned __int64 sfUint64;
#else
    typedef signed   long long sfInt64;
    typedef unsigned long long sfUint64;
#endif


#endif // SFML_CONFIG_H
