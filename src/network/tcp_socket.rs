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
*/

use std::libc::size_t;
use std::vec;
use std::ptr;

use network::ip_address;
use system::time;
use network::socket_status::SocketStatus;
use network::packet;

#[doc(hidden)]
pub mod csfml {

    use std::libc::{c_void, size_t};
    use rsfml::sfTypes::sfBool;
    use network::ip_address;
    use system::time;
    use network::socket_status::SocketStatus;
    use network::packet;

    pub struct sfTcpSocket {
        This : *c_void
    }

    pub extern "C" {
        fn sfTcpSocket_create() -> *sfTcpSocket;
        fn sfTcpSocket_destroy(socket : *sfTcpSocket) -> ();
        fn sfTcpSocket_setBlocking(socket : *sfTcpSocket, blocking : sfBool) -> ();
        fn sfTcpSocket_isBlocking(socket : *sfTcpSocket) -> sfBool;
        fn sfTcpSocket_getLocalPort(socket : *sfTcpSocket) -> u16;
        fn sfTcpSocket_getRemoteAddress(socket : *sfTcpSocket) -> ip_address::csfml::sfIpAddress;
        fn sfTcpSocket_getRemotePort(socket : *sfTcpSocket) -> u16;
        fn sfTcpSocket_connect(socket : *sfTcpSocket, host : ip_address::csfml::sfIpAddress, port : u16,  timeout : time::csfml::sfTime) -> SocketStatus;
        fn sfTcpSocket_disconnect(socket : *sfTcpSocket) -> ();
        fn sfTcpSocket_send(socket : *sfTcpSocket, data : *i8, size : size_t) -> SocketStatus;
        fn sfTcpSocket_receive(socket : *sfTcpSocket, data : *i8, maxSize : size_t, sizeReceived : *size_t) -> SocketStatus;
        fn sfTcpSocket_sendPacket(socket : *sfTcpSocket, packet : *packet::csfml::sfPacket) -> SocketStatus;
        fn sfTcpSocket_receivePacket(socket : *sfTcpSocket, packet : *packet::csfml::sfPacket) -> SocketStatus;
    }

}

#[doc(hidden)]
pub struct TcpSocket {
    priv socket : *csfml::sfTcpSocket
}

impl TcpSocket {
    pub fn new() -> TcpSocket {
        unsafe {
            TcpSocket { socket : csfml::sfTcpSocket_create()}
        }
    }

    pub fn set_blocking(&self, blocking : bool) -> () {
        match blocking  {
            true        => unsafe {csfml::sfTcpSocket_setBlocking(self.socket, 1)},
            false       => unsafe {csfml::sfTcpSocket_setBlocking(self.socket, 0)},
        }
    }

    pub fn is_blocking(&self) -> bool {
        match unsafe {csfml::sfTcpSocket_isBlocking(self.socket)} {
            0 => false,
            _ => true
        }
    }

    pub fn get_local_port(&self) -> u16 {
        unsafe {
            csfml::sfTcpSocket_getLocalPort(self.socket)
        }
    }

    pub fn get_remote_address(&self) -> ip_address::IpAddress {
        unsafe {
            ip_address::IpAddress::wrap(csfml::sfTcpSocket_getRemoteAddress(self.socket))
        }
    }

    pub fn get_remote_port(&self) -> u16 {
        unsafe {
            csfml::sfTcpSocket_getRemotePort(self.socket)
        }
    }

    pub fn connect(&self, host : &ip_address::IpAddress, port : u16, timeout : time::Time) -> SocketStatus {
        unsafe {
            csfml::sfTcpSocket_connect(self.socket, host.unwrap(), port, timeout.unwrap())
        }
    }

    pub fn disconnect(&self) -> () {
        unsafe {
            csfml::sfTcpSocket_disconnect(self.socket)
        }
    }

    pub fn send(&self, data : ~[i8]) -> SocketStatus {
        unsafe {
            csfml::sfTcpSocket_send(self.socket, vec::raw::to_ptr(data), data.len() as size_t)
        }
    }

    pub fn receive(&self, maxSize : size_t) -> (~[i8], SocketStatus, size_t) {
        unsafe {
            let s : size_t = 0;
            let datas : *i8 = ptr::null();
            let stat : SocketStatus = csfml::sfTcpSocket_receive(self.socket, datas, maxSize, &s);
            (vec::raw::from_buf_raw(datas, s as uint), stat, s)
        }
    }
    
    pub fn send_packet(&self, packet : &packet::Packet) -> SocketStatus {
        unsafe {
            csfml::sfTcpSocket_sendPacket(self.socket, packet.unwrap())
        }
    }

    pub fn receive_packet(&self) -> (packet::Packet, SocketStatus) {
        unsafe {
            let pack : *packet::csfml::sfPacket = ptr::null();
            let stat = csfml::sfTcpSocket_receivePacket(self.socket, pack);
            (packet::Packet::wrap(pack), stat)
        }
    }

    #[doc(hidden)]
    pub fn wrap(socket : *csfml::sfTcpSocket) -> TcpSocket {
        TcpSocket {socket : socket}
    }
    
    #[doc(hidden)]
    pub fn unwrap(&self) -> *csfml::sfTcpSocket {
        self.socket
    }
}

impl Drop for TcpSocket {
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfTcpSocket_destroy(self.socket)
        }
    }
}