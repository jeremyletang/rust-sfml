/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
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

//! A HTTP client

use std::mem;
use std::ffi::{CString, c_str_to_bytes_with_nul};
use std::str;

use traits::Wrappable;
use system::Time;

use ffi::network::http as ffi;

/// Method type to send the request
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Show, Copy)]
pub enum Method {
    /// Request in get mode, standard method to retrieve a page
    Get = ffi::GET as isize,
    /// Request in post mode, usually to send data to a page
    Post = ffi::POST as isize,
    /// Request a page's header only
    Head = ffi::HEAD as isize
}

/// Status code returned by a serveur.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Show, Copy)]
pub enum Status {
    // 2xx: success
    /// Most common code returned when operation was successful
    Ok                  = ffi::OK as isize,
    /// The resource has successfully been created
    Created             = ffi::CREATED as isize,
    /// The request has been accepted, but will be processed later by the server
    Accepted            = ffi::ACCEPTED as isize,
    /// Sent when the server didn't send any data in return
    NoContent           = ffi::NOCONTENT as isize,
    /// The server informs the client that it should clear the view (form) that caused the request to be sent
    ResetContent        = ffi::RESETCONTENT as isize,
    /// The server has sent a part of the resource, as a response to a partial GET request
    PartialContent      = ffi::PARTIALCONTENT as isize,

    // 3xx: redirection
    /// The requested page can be accessed from several locations
    MultipleChoices     = ffi::MULTIPLECHOICES as isize,
    /// The requested page has permanently moved to a new location
    MovedPermanently    = ffi::MOVEDPERMANENTLY as isize,
    /// The requested page has temporarily moved to a new location
    MovedTemporarily    = ffi::MOVEDTEMPORARILY as isize,
    /// For conditionnal requests, means the requested page hasn't changed and doesn't need to be refreshed
    NotModified         = ffi::NOTMODIFIED as isize,

    // 4xx: client error
    /// The server couldn't understand the request (syntax error)
    BadRequest          = ffi::BADREQUEST as isize,
    /// The requested page needs an authentification to be accessed
    Unauthorized        = ffi::UNAUTHORIZED as isize,
    /// The requested page cannot be accessed at all, even with authentification
    Forbidden           = ffi::FORBIDDEN as isize,
    /// The requested page doesn't exist
    NotFound            = ffi::NOTFOUND as isize,
    /// The server can't satisfy the partial GET request (with a "Range" header field)
    RangeNotSatisfiable = ffi::RANGENOTSATISFIABLE as isize,

    // 5xx: server error
    /// The server encountered an unexpected error
    InternalServerError = ffi::INTERNALSERVERERROR as isize,
    /// The server doesn't implement a requested feature
    NotImplemented      = ffi::NOTIMPLEMENTED as isize,
    /// The gateway server has received an error from the source server
    BadGateway          = ffi::BADGATEWAY as isize,
    /// The server is temporarily unavailable (overloaded, in maintenance, ...)
    ServiceNotAvailable = ffi::SERVICENOTAVAILABLE as isize,
    /// The gateway server couldn't receive a response from the source server
    GatewayTimeout      = ffi::GATEWAYTIMEOUT as isize,
    /// The server doesn't support the requested HTTP version
    VersionNotSupported = ffi::VERSIONNOTSUPPORTED as isize,

    // 10xx: SFML custom codes
    /// Response is not a valid HTTP one
    InvalidResponse     = ffi::INVALIDRESPONSE as isize,
    /// Connection with server failed
    ConnectionFailed    = ffi::CONNECTIONFAILED as isize
}

/// Encapsulation of an HTTP request
pub struct Request {
    #[doc(hidden)]
    request: *mut ffi::sfHttpRequest
}

/// Encapsulation of an HTTP response
pub struct Response {
    #[doc(hidden)]
    response: *mut ffi::sfHttpResponse
}

/// The HTTP client.
pub struct Http {
    #[doc(hidden)]
    http: *mut ffi::sfHttp
}

impl Request {
    /// Create a new HTTP request
    ///
    /// Return Some(Request) or None
    pub fn new() -> Option<Request> {
        let ptr = unsafe { ffi::sfHttpRequest_create() };
        if ptr.is_null() {
            None
        } else {
            Some(Request {
                request: ptr
            })
        }
    }

    /// Set the value of a header field of a HTTP request
    ///
    /// The field is created if it doesn't exist. The name of
    /// the field is case insensitive.
    /// By default, a request doesn't contain any field (but the
    /// mandatory fields are added later by the HTTP client when
    /// sending the request).
    ///
    /// # Arguments
    /// * field - Name of the field to set
    /// * value - Value of the field
    pub fn set_field(&self, field: &str, value: &str) -> () {
        let c_field = CString::from_slice(field.as_bytes()).as_ptr();
        let c_value = CString::from_slice(value.as_bytes()).as_ptr();
        unsafe {
            ffi::sfHttpRequest_setField(self.request,
                                        c_field,
                                        c_value)
        }
    }

    /// Set a HTTP request method
    ///
    /// See the Method enumeration for a complete list of all
    /// the availale methods.
    /// The method is Get by default.
    ///
    /// # Arguments
    /// * method - Method to use for the request
    pub fn set_method(&self, method: Method) -> () {
        unsafe {
            ffi::sfHttpRequest_setMethod(self.request, method as ffi::Method)
        }
    }

    /// Set a HTTP request URI
    ///
    /// The URI is the resource (usually a web page or a file)
    /// that you want to get or post.
    /// The URI is "/" (the root page) by default.
    ///
    /// # Arguments
    /// * uri - URI to request, relative to the host
    pub fn set_uri(&self, uri: &str) -> () {
        let c_uri = CString::from_slice(uri.as_bytes()).as_ptr();
        unsafe {
            ffi::sfHttpRequest_setUri(self.request, c_uri)
        }
    }

    /// Set the HTTP version of a HTTP request
    ///
    /// The HTTP version is 1.0 by default.
    ///
    /// # Arguments
    /// * major - Major HTTP version number
    /// * param minor - Minor HTTP version number
    pub fn set_http_version(&self, major: u32, minor: u32) -> () {
        unsafe {
            ffi::sfHttpRequest_setHttpVersion(self.request, major, minor)
        }
    }

    /// Set the body of a HTTP request
    ///
    /// The body of a request is optional and only makes sense
    /// for POST requests. It is ignored for all other methods.
    /// The body is empty by default.
    /// # Arguments
    /// * body - Content of the body
    pub fn set_body(&self, body: &str) -> () {
        let c_body = CString::from_slice(body.as_bytes()).as_ptr();
        unsafe {
            ffi::sfHttpRequest_setBody(self.request, c_body)
        }
    }

    #[doc(hidden)]
    fn unwrap(&self) -> *mut ffi::sfHttpRequest {
        self.request
    }
}

impl Drop for Request {
    fn drop(&mut self) -> () {
        unsafe {
            ffi::sfHttpRequest_destroy(self.request)
        }
    }
}

impl Response {
    /// Get the value of a field of a HTTP response
    ///
    /// If the field field is not found in the response header,
    /// the empty string is returned. This function uses
    /// case-insensitive comparisons.
    ///
    /// # Arguments
    /// * field - Name of the field to get
    ///
    /// Return Value of the field, or empty string if not found
    pub fn get_field(&self, field: &str) -> String {
        let c_field = CString::from_slice(field.as_bytes()).as_ptr();
        unsafe {
            let string = ffi::sfHttpResponse_getField(self.response, c_field);
            str::from_utf8(c_str_to_bytes_with_nul(&string)).unwrap().to_string()
        }
    }

    /// Get the status code of a HTTP reponse
    ///
    /// The status code should be the first thing to be checked
    /// after receiving a response, it defines whether it is a
    /// success, a failure or anything else (see the sfHttpStatus
    /// enumeration).
    ///
    /// Return the status code
    pub fn get_status(&self) -> Status {
        unsafe {
            mem::transmute(ffi::sfHttpResponse_getStatus(self.response) as i16)
        }
    }

    /// Get the major HTTP version number of a HTTP response
    ///
    /// Return Major HTTP version number
    pub fn get_major_version(&self) -> u32 {
        unsafe {
            ffi::sfHttpResponse_getMajorVersion(self.response)
        }
    }

    /// Get the minor HTTP version number of a HTTP response
    ///
    /// Return the minor HTTP version number
    pub fn get_minor_version(&self) -> u32 {
        unsafe {
            ffi::sfHttpResponse_getMinorVersion(self.response)
        }
    }

    /// Get the body of a HTTP response
    ///
    /// The body of a response may contain:
    /// the requested page (for GET requests),
    /// a response from the server (for POST requests),
    /// nothing (for HEAD requests),
    /// an error message (in case of an error)
    ///
    /// Return the response body
    pub fn get_body(&self) -> String {
        unsafe {
            let string = ffi::sfHttpResponse_getBody(self.response);
            str::from_utf8(c_str_to_bytes_with_nul(&string)).unwrap().to_string()
        }
    }
}

impl Drop for Response {
    fn drop(&mut self) -> () {
        unsafe {
            ffi::sfHttpResponse_destroy(self.response)
        }
    }
}

impl Http {
    /// Create a new Http object
    ///
    /// Return Some(Http) or None
    pub fn new() -> Option<Http> {
        let ptr = unsafe{ ffi::sfHttp_create() };
        if ptr.is_null() {
            None
        } else {
            Some(Http {
                http: ptr
            })
        }
    }

    /// Set the target host of a HTTP object
    ///
    /// This function just stores the host address and port, it
    /// doesn't actually connect to it until you send a request.
    /// If the port is 0, it means that the HTTP client will use
    /// the right port according to the protocol used
    /// (80 for HTTP, 443 for HTTPS). You should
    /// leave it like this unless you really need a port other
    /// than the standard one, or use an unknown protocol.
    ///
    /// # Arguments
    /// * host - Web server to connect to
    /// * port - Port to use for connection
    pub fn set_host(&self, host: &str, port: u16) -> () {
        let c_host = CString::from_slice(host.as_bytes()).as_ptr();
        unsafe {
            ffi::sfHttp_setHost(self.http, c_host, port)
        }
    }

    /// Send a HTTP request and return the server's response.
    ///
    /// You must have a valid host before sending a request (see sfHttp_setHost).
    /// Any missing mandatory header field in the request will be added
    /// with an appropriate value.
    /// Warning: this function waits for the server's response and may
    /// not return instantly; use a thread if you don't want to block your
    /// application, or use a timeout to limit the time to wait. A value
    /// of 0 means that the client will use the system defaut timeout
    /// (which is usually pretty long).
    ///
    /// # Arguments
    /// * request - Request to send
    /// * timeout - Maximum time to wait
    pub fn send_request(&self, request: &Request, timeout: &Time) -> Response {
        Response {
            response: unsafe { ffi::sfHttp_sendRequest(self.http, request.unwrap(), timeout.unwrap()) }
        }
    }
}

impl Drop for Http {
    fn drop(&mut self) -> () {
        unsafe {
            ffi::sfHttp_destroy(self.http)
        }
    }
}
