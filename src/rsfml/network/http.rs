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

/*!
* A HTTP client
*
*
*
*
*/

use std::{str, cast};
use std::libc::c_int;

use traits::wrappable::Wrappable;
use system::Time;

#[doc(hidden)]
pub mod ffi {
    
    use std::libc::{c_char, c_void, c_int};

    use ffi::system::time::sfTime;

    pub type Method = c_int;
    pub static GET : Method = 0;
    pub static POST : Method = 1;
    pub static HEAD : Method = 2;

    pub type Status = c_int;
    pub static OK:                  Status = 200;
    pub static CREATED:             Status = 201;
    pub static ACCEPTED:            Status = 202;
    pub static NOCONTENT:           Status = 204;
    pub static RESETCONTENT:        Status = 205;
    pub static PARTIALCONTENT:      Status = 206;
   
    pub static MULTIPLECHOICES:     Status = 300;
    pub static MOVEDPERMANENTLY:    Status = 301;
    pub static MOVEDTEMPORARILY:    Status = 302;
    pub static NOTMODIFIED:         Status = 304;


    pub static BADREQUEST:          Status = 400;
    pub static UNAUTHORIZED:        Status = 401;
    pub static FORBIDDEN:           Status = 403;
    pub static NOTFOUND:            Status = 404;
    pub static RANGENOTSATISFIABLE: Status = 407;

    pub static INTERNALSERVERERROR: Status = 500;
    pub static NOTIMPLEMENTED:      Status = 501;
    pub static BADGATEWAY:          Status = 502;
    pub static SERVICENOTAVAILABLE: Status = 503;
    pub static GATEWAYTIMEOUT:      Status = 504;
    pub static VERSIONNOTSUPPORTED: Status = 505;

    pub static INVALIDRESPONSE:     Status = 1000;
    pub static CONNECTIONFAILED:    Status = 1001;

    pub struct sfHttpRequest {
        This : *c_void
    }

    pub struct sfHttpResponse {
        This : *c_void
    }

    pub struct sfHttp {
        This : *c_void
    }

    extern "C" {
        pub fn sfHttpRequest_create() -> *sfHttpRequest;
        pub fn sfHttpRequest_destroy(httpRequest : *sfHttpRequest) -> ();
        pub fn sfHttpRequest_setField(httpRequest : *sfHttpRequest, field : *c_char, value : *c_char) -> ();
        pub fn sfHttpRequest_setMethod(httpRequest : *sfHttpRequest, method : Method) -> ();
        pub fn sfHttpRequest_setUri(httpRequest : *sfHttpRequest, uri : *c_char) -> ();
        pub fn sfHttpRequest_setHttpVersion(httpRequest : *sfHttpRequest, major : u32, minor : u32) -> ();
        pub fn sfHttpRequest_setBody(httpRequest : *sfHttpRequest, body : *c_char) -> ();
        pub fn sfHttpResponse_destroy(httpResponse : *sfHttpResponse) -> ();
        pub fn sfHttpResponse_getField(httpResponse : *sfHttpResponse, field : *c_char) -> *c_char;
        pub fn sfHttpResponse_getStatus(httpResponse : *sfHttpResponse) -> Status;
        pub fn sfHttpResponse_getMajorVersion(httpResponse : *sfHttpResponse) -> u32;
        pub fn sfHttpResponse_getMinorVersion(httpResponse : *sfHttpResponse) -> u32;
        pub fn sfHttpResponse_getBody(httpResponse : *sfHttpResponse) -> *c_char;
        pub fn sfHttp_create() -> *sfHttp;
        pub fn sfHttp_destroy(http : *sfHttp) -> ();
        pub fn sfHttp_setHost(http : *sfHttp, host : *c_char, port : u16) -> ();
        pub fn sfHttp_sendRequest(http : *sfHttp, httpRequest : *sfHttpRequest, timeout : sfTime) -> *sfHttpResponse;
    }
}

#[deriving(Eq, Ord)]
pub enum Method {
    /// Request in get mode, standard method to retrieve a page
    Get = ffi::GET as c_int,
    /// Request in post mode, usually to send data to a page
    Post = ffi::POST as c_int,
    /// Request a page's header only
    Head = ffi::HEAD as c_int
}

#[deriving(Eq, Ord)]
pub enum Status {
    // 2xx: success
    /// Most common code returned when operation was successful
    Ok                  = ffi::OK as c_int,
    /// The resource has successfully been created
    Created             = ffi::CREATED as c_int,
    /// The request has been accepted, but will be processed later by the server
    Accepted            = ffi::ACCEPTED as c_int,
    /// Sent when the server didn't send any data in return
    NoContent           = ffi::NOCONTENT as c_int,
    /// The server informs the client that it should clear the view (form) that caused the request to be sent
    ResetContent        = ffi::RESETCONTENT as c_int,
    /// The server has sent a part of the resource, as a response to a partial GET request
    PartialContent      = ffi::PARTIALCONTENT as c_int,

    // 3xx: redirection
    /// The requested page can be accessed from several locations
    MultipleChoices     = ffi::MULTIPLECHOICES as c_int,
    /// The requested page has permanently moved to a new location
    MovedPermanently    = ffi::MOVEDPERMANENTLY as c_int,
    /// The requested page has temporarily moved to a new location
    MovedTemporarily    = ffi::MOVEDTEMPORARILY as c_int,
    /// For conditionnal requests, means the requested page hasn't changed and doesn't need to be refreshed
    NotModified         = ffi::NOTMODIFIED as c_int, 

    // 4xx: client error
    /// The server couldn't understand the request (syntax error)
    BadRequest          = ffi::BADREQUEST as c_int,
    /// The requested page needs an authentification to be accessed
    Unauthorized        = ffi::UNAUTHORIZED as c_int,
    /// The requested page cannot be accessed at all, even with authentification
    Forbidden           = ffi::FORBIDDEN as c_int,
    /// The requested page doesn't exist
    NotFound            = ffi::NOTFOUND as c_int,
    /// The server can't satisfy the partial GET request (with a "Range" header field)
    RangeNotSatisfiable = ffi::RANGENOTSATISFIABLE as c_int, 

    // 5xx: server error
    /// The server encountered an unexpected error
    InternalServerError = ffi::INTERNALSERVERERROR as c_int,
    /// The server doesn't implement a requested feature
    NotImplemented      = ffi::NOTIMPLEMENTED as c_int,
    /// The gateway server has received an error from the source server
    BadGateway          = ffi::BADGATEWAY as c_int,
    /// The server is temporarily unavailable (overloaded, in maintenance, ...)
    ServiceNotAvailable = ffi::SERVICENOTAVAILABLE as c_int,
    /// The gateway server couldn't receive a response from the source server
    GatewayTimeout      = ffi::GATEWAYTIMEOUT as c_int,
    /// The server doesn't support the requested HTTP version
    VersionNotSupported = ffi::VERSIONNOTSUPPORTED as c_int, 

    // 10xx: SFML custom codes
    /// Response is not a valid HTTP one
    InvalidResponse     = ffi::INVALIDRESPONSE as c_int,
    /// Connection with server failed
    ConnectionFailed    = ffi::CONNECTIONFAILED as c_int 
}

pub struct Request {
    #[doc(hidden)]
    priv request : *ffi::sfHttpRequest
}

pub struct Response {
    #[doc(hidden)]
    priv response : *ffi::sfHttpResponse
}

pub struct Http {
    #[doc(hidden)]
    priv http : *ffi::sfHttp
}

impl Request {
    /**
    * Create a new HTTP request
    *
    * Return a new option to HttpRequest object, or None
    */
    pub fn new() -> Request {
        Request { 
            request : unsafe { ffi::sfHttpRequest_create() }
        }
    }

    /**
    * Set the value of a header field of a HTTP request
    *
    * The field is created if it doesn't exist. The name of
    * the field is case insensitive.
    * By default, a request doesn't contain any field (but the
    * mandatory fields are added later by the HTTP client when
    * sending the request).
    *
    * # Arguments
    * * field - Name of the field to set
    * * value - Value of the field
    */
    pub fn set_field(&self, field : &str, value : &str) -> () {
        let c_field = field.to_c_str();
        let c_value = value.to_c_str();
        unsafe {
            ffi::sfHttpRequest_setField(self.request, c_field.unwrap(), c_value.unwrap())
        }
    }

    /**
    * Set a HTTP request method
    *
    * See the Method enumeration for a complete list of all
    * the availale methods.
    * The method is Get by default.
    *
    * # Arguments
    * * method - Method to use for the request
    */
    pub fn set_method(&self, method : Method) -> () {
        unsafe {
            ffi::sfHttpRequest_setMethod(self.request, method as ffi::Method)
        }
    }

    /**
    * Set a HTTP request URI
    *
    * The URI is the resource (usually a web page or a file)
    * that you want to get or post.
    * The URI is "/" (the root page) by default.
    *
    * # Arguments
    * * uri - URI to request, relative to the host
    */
    pub fn set_uri(&self, uri : &str) -> () {
        let c_uri = uri.to_c_str();
        unsafe {
            ffi::sfHttpRequest_setUri(self.request, c_uri.unwrap())
        }
    }

    /**
    * Set the HTTP version of a HTTP request
    *
    * The HTTP version is 1.0 by default.
    *
    * # Arguments
    * * major - Major HTTP version number
    * * param minor - Minor HTTP version number
    */
    pub fn set_http_version(&self, major : u32, minor : u32) -> () {
        unsafe {
            ffi::sfHttpRequest_setHttpVersion(self.request, major, minor)
        }
    }

    /**
    * Set the body of a HTTP request
    *
    * The body of a request is optional and only makes sense
    * for POST requests. It is ignored for all other methods. 
    * The body is empty by default.
    * # Arguments
    * * body - Content of the body
    */
    pub fn set_body(&self, body : &str) -> () {
        let c_body = body.to_c_str();
        unsafe {
            ffi::sfHttpRequest_setBody(self.request, c_body.unwrap())
        }
    }

    #[doc(hidden)]
    fn unwrap(&self) -> *ffi::sfHttpRequest {
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
    /**
    * Get the value of a field of a HTTP response
    *
    * If the field field is not found in the response header,
    * the empty string is returned. This function uses
    * case-insensitive comparisons.
    *
    * # Arguments
    * * field - Name of the field to get
    * 
    * Return Value of the field, or empty string if not found
    */
    pub fn get_field(&self, field : &str) -> ~str {
        let c_field = field.to_c_str();
        unsafe {
            str::raw::from_c_str(ffi::sfHttpResponse_getField(self.response, c_field.unwrap()))
        }
    }

    /**
    * Get the status code of a HTTP reponse
    *
    * The status code should be the first thing to be checked
    * after receiving a response, it defines whether it is a
    * success, a failure or anything else (see the sfHttpStatus
    * enumeration).
    *
    * Return the status code
    */
    pub fn get_status(&self) -> Status {
        unsafe {
            cast::transmute(ffi::sfHttpResponse_getStatus(self.response) as i16)
        }
    }

    /**
    * Get the major HTTP version number of a HTTP response
    * 
    * Return Major HTTP version number
    */
    pub fn get_major_version(&self) -> u32 {
        unsafe {
            ffi::sfHttpResponse_getMajorVersion(self.response)
        }
    }
    
    /**
    * Get the minor HTTP version number of a HTTP response
    *
    * Return the minor HTTP version number
    */
    pub fn get_minor_version(&self) -> u32 {
        unsafe {
            ffi::sfHttpResponse_getMinorVersion(self.response)
        }
    }

    /**
    * Get the body of a HTTP response
    *
    * The body of a response may contain:
    * the requested page (for GET requests),
    * a response from the server (for POST requests),
    * nothing (for HEAD requests),
    * an error message (in case of an error)
    *
    * Return the response body
    */
    pub fn get_body(&self) -> ~str {
        unsafe {
            str::raw::from_c_str(ffi::sfHttpResponse_getBody(self.response))
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
    /**
    * Create a new Http object
    *
    * Return a new option to Http object or None
    */
    pub fn create() -> Http {
        Http { 
            http : unsafe{ ffi::sfHttp_create() }
        }
    }

    /**
    * Set the target host of a HTTP object
    *
    * This function just stores the host address and port, it
    * doesn't actually connect to it until you send a request.
    * If the port is 0, it means that the HTTP client will use
    * the right port according to the protocol used
    * (80 for HTTP, 443 for HTTPS). You should
    * leave it like this unless you really need a port other
    * than the standard one, or use an unknown protocol.
    *
    * # Arguments
    * * host - Web server to connect to
    * * port - Port to use for connection
    */
    pub fn set_host(&self, host : &str, port : u16) -> () {
        let c_host = host.to_c_str();
        unsafe {
                ffi::sfHttp_setHost(self.http, c_host.unwrap(), port)
        }
    }

    /**
    * Send a HTTP request and return the server's response.
    *
    * You must have a valid host before sending a request (see sfHttp_setHost).
    * Any missing mandatory header field in the request will be added
    * with an appropriate value.
    * Warning: this function waits for the server's response and may
    * not return instantly; use a thread if you don't want to block your
    * application, or use a timeout to limit the time to wait. A value
    * of 0 means that the client will use the system defaut timeout
    * (which is usually pretty long).
    *
    * # Arguments
    * * request - Request to send
    * * timeout - Maximum time to wait
    */
    pub fn send_request(&self, request : &Request, timeout : &Time) -> Response {
        Response {
            response : unsafe { ffi::sfHttp_sendRequest(self.http, request.unwrap(), timeout.unwrap()) }
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
