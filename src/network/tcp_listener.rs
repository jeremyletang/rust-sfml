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

//! Socket that listens to new TCP connections

use std::mem;

use traits::Wrappable;
use network::{TcpSocket, SocketStatus};

use ffi::sfml_types::{SFTRUE, SFFALSE};
use ffi = ffi::network::tcp_listener;

/// Socket that listens to new TCP connections
pub struct TcpListener {
    #[doc(hidden)]
    listener : *ffi::sfTcpListener
}

impl TcpListener {
    /**
    * Create a new TCP listener
    *
    * Return Some(TcpListener) or None
    */
    pub fn new() -> Option<TcpListener> {
        let list = unsafe { ffi::sfTcpListener_create() };
        if list.is_null() {
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
    pub fn set_blocking(&mut self, blocking : bool) -> () {
        unsafe {
            match blocking  {
                true        => ffi::sfTcpListener_setBlocking(self.listener, SFTRUE),
                false       => ffi::sfTcpListener_setBlocking(self.listener, SFFALSE)
            }
        }
    }

    /**
    * Tell whether a TCP listener is in blocking or non-blocking mode
    *
    * Return true if the socket is blocking, false otherwise
    */
    pub fn is_blocking(&self) -> bool {
        match unsafe { ffi::sfTcpListener_isBlocking(self.listener) } {
            SFFALSE => false,
            SFTRUE  => true
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
    pub fn listen(&self, port : u16) -> SocketStatus {
        unsafe {
            mem::transmute(ffi::sfTcpListener_listen(self.listener, port) as i8)
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
    pub fn accept(&self, connected : &TcpSocket) -> SocketStatus {
        unsafe {
            mem::transmute(ffi::sfTcpListener_accept(self.listener, &connected.unwrap()) as i8)
        }
    }
}

impl Drop for TcpListener {
    fn drop(&mut self) -> () {
        unsafe {
            ffi::sfTcpListener_destroy(self.listener)
        }
    }
}
