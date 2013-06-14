/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The Original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
* 
* 3. This notice may not be removed or altered from any source distribution.
*/

/*!
*
*
*
*
*
*/

use std::str;

use system::time;

#[doc(hidden)]
pub mod csfml {
    
    use std::libc::{c_char, c_void};

    use network::http::Method;
    use network::http::Status;
    use system::time;

    pub struct sfHttpRequest {
        This : *c_void
    }

    pub struct sfHttpResponse {
        This : *c_void
    }

    pub struct sfHttp {
        This : *c_void
    }

    pub extern "C" {
        fn sfHttpRequest_create() -> *sfHttpRequest;
        fn sfHttpRequest_destroy(httpRequest : *sfHttpRequest) -> ();
        fn sfHttpRequest_setField(httpRequest : *sfHttpRequest, field : *c_char, value : *c_char) -> ();
        fn sfHttpRequest_setMethod(httpRequest : *sfHttpRequest, method : Method) -> ();
        fn sfHttpRequest_setUri(httpRequest : *sfHttpRequest, uri : *c_char) -> ();
        fn sfHttpRequest_setHttpVersion(httpRequest : *sfHttpRequest, major : u32, minor : u32) -> ();
        fn sfHttpRequest_setBody(httpRequest : *sfHttpRequest, body : *c_char) -> ();
        fn sfHttpResponse_destroy(httpResponse : *sfHttpResponse) -> ();
        fn sfHttpResponse_getField(httpResponse : *sfHttpResponse, field : *c_char) -> *c_char;
        fn sfHttpResponse_getStatus(httpResponse : *sfHttpResponse) -> Status;
        fn sfHttpResponse_getMajorVersion(httpResponse : *sfHttpResponse) -> u32;
        fn sfHttpResponse_getMinorVersion(httpResponse : *sfHttpResponse) -> u32;
        fn sfHttpResponse_getBody(httpResponse : *sfHttpResponse) -> *c_char;
        fn sfHttp_create() -> *sfHttp;
        fn sfHttp_destroy(http : *sfHttp) -> ();
        fn sfHttp_setHost(http : *sfHttp, host : *c_char, port : u16) -> ();
        fn sfHttp_sendRequest(http : *sfHttp, httpRequest : *sfHttpRequest, timeout : time::csfml::sfTime) -> *sfHttpResponse;
    }
}

pub enum Method {
    Get,  ///< Request in get mode, standard method to retrieve a page
    Post, ///< Request in post mode, usually to send data to a page
    Head  //< Request a page's header only
}

pub enum Status {
    // 2xx: success
    Ok             = 200, ///< Most common code returned when operation was successful
    Created        = 201, ///< The resource has successfully been created
    Accepted       = 202, ///< The request has been accepted, but will be processed later by the server
    NoContent      = 204, ///< Sent when the server didn't send any data in return
    ResetContent   = 205, ///< The server informs the client that it should clear the view (form) that caused the request to be sent
    PartialContent = 206, ///< The server has sent a part of the resource, as a response to a partial GET request

    // 3xx: redirection
    MultipleChoices  = 300, ///< The requested page can be accessed from several locations
    MovedPermanently = 301, ///< The requested page has permanently moved to a new location
    MovedTemporarily = 302, ///< The requested page has temporarily moved to a new location
    NotModified      = 304, ///< For conditionnal requests, means the requested page hasn't changed and doesn't need to be refreshed

    // 4xx: client error
    BadRequest          = 400, ///< The server couldn't understand the request (syntax error)
    Unauthorized        = 401, ///< The requested page needs an authentification to be accessed
    Forbidden           = 403, ///< The requested page cannot be accessed at all, even with authentification
    NotFound            = 404, ///< The requested page doesn't exist
    RangeNotSatisfiable = 407, ///< The server can't satisfy the partial GET request (with a "Range" header field)

    // 5xx: server error
    InternalServerError = 500, ///< The server encountered an unexpected error
    NotImplemented      = 501, ///< The server doesn't implement a requested feature
    BadGateway          = 502, ///< The gateway server has received an error from the source server
    ServiceNotAvailable = 503, ///< The server is temporarily unavailable (overloaded, in maintenance, ...)
    GatewayTimeout      = 504, ///< The gateway server couldn't receive a response from the source server
    VersionNotSupported = 505, ///< The server doesn't support the requested HTTP version

    // 10xx: SFML custom codes
    InvalidResponse  = 1000, ///< Response is not a valid HTTP one
    ConnectionFailed = 1001  //< Connection with server failed
}

#[doc(hidden)]
pub struct Request {
    priv request : *csfml::sfHttpRequest
}

#[doc(hidden)]
pub struct Response {
    priv response : *csfml::sfHttpResponse
}

#[doc(hidden)]
pub struct Http {
    priv http : *csfml::sfHttp
}

impl Request {
    pub fn new() -> Request {
        Request { request : unsafe {csfml::sfHttpRequest_create()} }
    }

    pub fn set_field(&self, field : ~str, value : ~str) -> () {
        unsafe {
            do str::as_c_str(field) |f| {
                do str::as_c_str(value) |v| {
                    csfml::sfHttpRequest_setField(self.request, f, v)
                }
            }
        }
    }

    pub fn set_method(&self, method : Method) -> () {
        unsafe {
            csfml::sfHttpRequest_setMethod(self.request, method)
        }
    }

    pub fn set_uri(&self, uri : ~str) -> () {
        unsafe {
            do str::as_c_str(uri) |Uri| {
                csfml::sfHttpRequest_setUri(self.request, Uri)
            }
        }
    }

    pub fn set_http_version(&self, major : u32, minor : u32) -> () {
        unsafe {
            csfml::sfHttpRequest_setHttpVersion(self.request, major, minor)
        }
    }

    pub fn set_body(&self, body : ~str) -> () {
        unsafe {
            do str::as_c_str(body) |Body| {
            csfml::sfHttpRequest_setBody(self.request, Body)
            }
        }
    }

    #[doc(hidden)]
    pub fn unwrap(&self) -> *csfml::sfHttpRequest {
        self.request
    }
}

impl Drop for Request {
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfHttpRequest_destroy(self.request)
        }
    }
}

impl Response {
    pub fn get_field(&self, field : ~str) -> ~str {
        unsafe {
            do str::as_c_str(field) |f| {
                str::raw::from_c_str(csfml::sfHttpResponse_getField(self.response, f))
            }
        }
    }

    pub fn get_status(&self) -> Status {
        unsafe {
            csfml::sfHttpResponse_getStatus(self.response)
        }
    }

    pub fn get_major_version(&self) -> u32 {
        unsafe {
            csfml::sfHttpResponse_getMajorVersion(self.response)
        }
    }
    
    pub fn get_minor_version(&self) -> u32 {
        unsafe {
            csfml::sfHttpResponse_getMinorVersion(self.response)
        }
    }

    pub fn get_body(&self) -> ~str {
        unsafe {
            str::raw::from_c_str(csfml::sfHttpResponse_getBody(self.response))
        }
    } 
}

impl Drop for Response {
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfHttpResponse_destroy(self.response)
        }
    }
}

impl Http {
    pub fn create() -> Http {
        Http { http : unsafe{csfml::sfHttp_create()} }
    }

    pub fn set_host(&self, host : ~str, port : u16) -> () {
        unsafe {
            do str::as_c_str(host) |h| {
                csfml::sfHttp_setHost(self.http, h, port)
            }
        }
    }

    pub fn send_request(&self, request : &Request, timeout : &time::Time) -> Response {
        Response { response : unsafe {csfml::sfHttp_sendRequest(self.http, request.unwrap(), timeout.unwrap())}}
    }
}

impl Drop for Http {
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfHttp_destroy(self.http)
        }
    }
}
