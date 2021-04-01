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

#ifndef SFML_INTERNAL_H
#define SFML_INTERNAL_H

////////////////////////////////////////////////////////////
// Define macros to check the validity of CSFML objects in debug run
////////////////////////////////////////////////////////////
#include <stdio.h>

// this macro avoids the C4127 warning on VC++ ("conditional expression is constant")
#define ASSERT_FALSE (__LINE__ == -1)

#ifndef NDEBUG

    #define CSFML_CHECK(object_) \
        do \
        { \
            if (object_ == NULL) \
            { \
                fprintf(stderr, "SFML warning: trying to use a null " #object_ " object\n"); \
                return; \
            } \
        } \
        while (ASSERT_FALSE)

    #define CSFML_CALL(object_, function_) \
        do \
        { \
            if (object_) \
            { \
                (object_->This.function_); \
            } \
            else \
            { \
                fprintf(stderr, "SFML warning: trying to use a null " #object_ " object\n"); \
            } \
        } \
        while (ASSERT_FALSE)

    #define CSFML_CALL_PTR(object_, function_) \
        do \
        { \
            if (object_) \
            { \
                (object_->This->function_); \
            } \
            else \
            { \
                fprintf(stderr, "SFML warning: trying to use a null " #object_ " object\n"); \
            } \
        } \
        while (ASSERT_FALSE)

    #define CSFML_CHECK_RETURN(object_, default_) \
        do \
        { \
            if (object_ == NULL) \
            { \
                fprintf(stderr, "SFML warning: trying to use a null " #object_ " object\n"); \
                return default_; \
            } \
        } \
        while (ASSERT_FALSE)

    #define CSFML_CALL_RETURN(object_, function_, default_) \
        if (object_) \
        { \
            return (object_->This.function_); \
        } \
        else \
        { \
            fprintf(stderr, "SFML warning: trying to use a null " #object_ " object\n"); \
            return default_; \
        }

    #define CSFML_CALL_PTR_RETURN(object_, function_, default_) \
        if (object_) \
        { \
            return (object_->This->function_); \
        } \
        else \
        { \
            fprintf(stderr, "SFML warning: trying to use a null " #object_ " object\n"); \
            return default_; \
        }

#else

    #define CSFML_CHECK(object_)

    #define CSFML_CALL(object_, function_) \
        (object_->This.function_)

    #define CSFML_CALL_PTR(object_, function_) \
        (object_->This->function_)

    #define CSFML_CHECK_RETURN(object_, default_) \
        (void)default_;

    #define CSFML_CALL_RETURN(object_, function_, default_) \
        { \
            (void)default_; \
            return (object_->This.function_); \
        }

    #define CSFML_CALL_PTR_RETURN(object_, function_, default_) \
        { \
            (void)default_; \
            return (object_->This->function_); \
        }

#endif

#endif // SFML_INTERNAL_H
