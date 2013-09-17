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
* Socket that listens to new TCP connections
*
*
*
*
*
*/

use std::ptr;

use traits::wrappable::Wrappable;
use network::tcp_socket::TcpSocket;
use network::socket_status::SocketStatus;

#[doc(hidden)]
pub mod ffi {

    use std::libc::{c_void};
 
    use network::tcp_socket;
    use sfml_types::SfBool;
    use network::socket_status::SocketStatus;

    pub struct sfTcpListener {
        This : *c_void
    }
    
    extern "C" {
        pub fn sfTcpListener_create() -> *sfTcpListener;
        pub fn sfTcpListener_destroy(listener : *sfTcpListener) -> ();
        pub fn sfTcpListener_setBlocking(listener : *sfTcpListener, blocking : SfBool) -> ();
        pub fn sfTcpListener_isBlocking(listener : *sfTcpListener) -> SfBool;
        pub fn sfTcpListener_getLocalPort(listener : *sfTcpListener) -> u16;
        pub fn sfTcpListener_listen(listener : *sfTcpListener, port : u16) -> SocketStatus;
        pub fn sfTcpListener_accept(listener : *sfTcpListener, connected : **tcp_socket::ffi::sfTcpSocket) -> SocketStatus;
    }
}

#[doc(hidden)]
pub struct TcpListener {
    priv listener : *ffi::sfTcpListener
}

impl TcpListener {
    /**
    * Create a new TCP listener
    *
    * Return a new option to TcpListener object or None
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn new() -> Option<TcpListener> {
        let list = unsafe { ffi::sfTcpListener_create() };
        if ptr::is_null(list) {
            None
        }
        else {
            Some(TcpListener { 
                listener : list
            })
        }
    }

    /**
    * Set the blocking state of a TCP listener
    *
    * In blocking mode, calls will not return until they have
    * completed their task. For example, a call to
    * sfTcpListener_accept in blocking mode won't return until
    * a new connection was actually received.
    * In non-blocking mode, calls will always return immediately,
    * using the return code to signal whether there was data
    * available or not.
    * By default, all sockets are blocking.
    *
    * # Arguments
    * * blocking - true to set the socket as blocking, false for non-blocking
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_blocking(&mut self, blocking : bool) -> () {
        unsafe {
            match blocking  {
                true        => ffi::sfTcpListener_setBlocking(self.listener, 1),
                false       => ffi::sfTcpListener_setBlocking(self.listener, 0)
            }
        }
    }

    /**
    * Tell whether a TCP listener is in blocking or non-blocking mode
    *
    * Return true if the socket is blocking, false otherwise
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_blocking(&self) -> bool {
        match unsafe { ffi::sfTcpListener_isBlocking(self.listener) } {
            0 => false,
            _ => true
        }
    }

    /**
    * Get the port to which a TCP listener is bound locally
    *
    * If the socket is not listening to a port, this function
    * returns 0.
    *
    * Return the port to which the TCP listener is bound
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_local_port(&self) -> u16 {
        unsafe {
            ffi::sfTcpListener_getLocalPort(self.listener)
        }
    }
    
    /**
    * Start listening for connections
    *
    * This functions makes the socket listen to the specified
    * port, waiting for new connections.
    * If the socket was previously listening to another port,
    * it will be stopped first and bound to the new port.
    * 
    * # Arguments
    * * port - Port to listen for new connections
    *
    * Return status code
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn listen(&self, port : u16) -> SocketStatus {
        unsafe {
            ffi::sfTcpListener_listen(self.listener, port)
        }
    }
    
    /**
    * Accept a new connection
    *
    * If the socket is in blocking mode, this function will
    * not return until a connection is actually received.
    * 
    * # Arguments
    * * connected - Socket that will hold the new connection
    *
    * Return status code
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn accept(&self, connected : @TcpSocket) -> SocketStatus {
        unsafe {
            ffi::sfTcpListener_accept(self.listener, &connected.unwrap())
        }
    }
}

impl Drop for TcpListener {
    #[fixed_stack_segment] #[inline(never)]
    fn drop(&mut self) -> () {
        unsafe {
            ffi::sfTcpListener_destroy(self.listener)
        }
    }
}
