/*
* Rust-SFML - Copyright (c) Letang Jeremy.
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
*
*/

use network::tcp_socket;
use network::socket_status::SocketStatus;

#[doc(hidden)]
pub mod csfml {

    use std::libc::{c_void};
 
    use network::tcp_socket;
    use rsfml::sfTypes::sfBool;
    use network::socket_status::SocketStatus;

    pub struct sfTcpListener {
        This : *c_void
    }
    
    pub extern "C" {
        fn sfTcpListener_create() -> *sfTcpListener;
        fn sfTcpListener_destroy(listener : *sfTcpListener) -> ();
        fn sfTcpListener_setBlocking(listener : *sfTcpListener, blocking : sfBool) -> ();
        fn sfTcpListener_isBlocking(listener : *sfTcpListener) -> sfBool;
        fn sfTcpListener_getLocalPort(listener : *sfTcpListener) -> u16;
        fn sfTcpListener_listen(listener : *sfTcpListener, port : u16) -> SocketStatus;
        fn sfTcpListener_accept(listener : *sfTcpListener, connected : **tcp_socket::csfml::sfTcpSocket) -> SocketStatus;
    }
}

#[doc(hidden)]
pub struct TcpListener {
    priv listener : *csfml::sfTcpListener
}

impl TcpListener {
    pub fn new() -> TcpListener {
        TcpListener { listener : unsafe {csfml::sfTcpListener_create()} }
    }

    pub fn set_blocking(&self, blocking : bool) -> () {
        match blocking  {
            true        => unsafe {csfml::sfTcpListener_setBlocking(self.listener, 1)},
            false       => unsafe {csfml::sfTcpListener_setBlocking(self.listener, 0)},
        }
    }

    pub fn is_blocking(&self) -> bool {
        match unsafe {csfml::sfTcpListener_isBlocking(self.listener)} {
            0 => false,
            _ => true
        }
    }

    pub fn get_local_port(&self) -> u16 {
        unsafe {
            csfml::sfTcpListener_getLocalPort(self.listener)
        }
    }
    
    pub fn listen(&self, port : u16) -> SocketStatus {
        unsafe {
            csfml::sfTcpListener_listen(self.listener, port)
        }
    }
    
    pub fn accept(&self, connected : @tcp_socket::TcpSocket) -> SocketStatus {
        unsafe {
            csfml::sfTcpListener_accept(self.listener, &connected.unwrap())
        }
    }
}

impl Drop for TcpListener {
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfTcpListener_destroy(self.listener)
        }
    }
}