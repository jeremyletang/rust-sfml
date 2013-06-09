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
*/

use std::ptr;
use std::vec;
use std::libc::size_t;

use network::packet;
use network::ip_address;
use network::socket_status::SocketStatus;

#[doc(hidden)]
pub mod csfml {
    
    use std::libc::{size_t, c_void};
    use network::socket_status::SocketStatus;
    use network::ip_address;
    use rsfml::sfTypes::sfBool;
    use network::packet;

    pub struct sfUdpSocket {
        This : *c_void
    }

    pub extern "C" {
        fn sfUdpSocket_create() -> *sfUdpSocket;
        fn sfUdpSocket_destroy(socket : *sfUdpSocket) -> ();
        fn sfUdpSocket_setBlocking(socket : *sfUdpSocket, blocking : sfBool) -> ();
        fn sfUdpSocket_isBlocking(socket : *sfUdpSocket) -> sfBool;
        fn sfUdpSocket_getLocalPort(socket : *sfUdpSocket) -> u16;
        fn sfUdpSocket_bind(socket : *sfUdpSocket, port : u16) -> SocketStatus;
        fn sfUdpSocket_unbind(socket : *sfUdpSocket) -> ();
        fn sfUdpSocket_send(socket : *sfUdpSocket, data : *i8, size : size_t, address : ip_address::csfml::sfIpAddress, port : u16) -> SocketStatus;
        fn sfUdpSocket_receive(socket : *sfUdpSocket, data : *i8, maxSize : size_t, sizeReceived : *size_t, address : *ip_address::csfml::sfIpAddress, port : *u16) -> SocketStatus;
        fn sfUdpSocket_sendPacket(socket : *sfUdpSocket, packet : *packet::csfml::sfPacket, address : ip_address::csfml::sfIpAddress, port : u16) -> SocketStatus;
        fn sfUdpSocket_receivePacket(socket : *sfUdpSocket, packet : *packet::csfml::sfPacket, address : *ip_address::csfml::sfIpAddress, port : *u16) -> SocketStatus;
        fn sfUdpSocket_maxDatagramSize() -> u32;
    }
}

#[doc(hidden)]
pub struct UdpSocket {
    priv socket : *csfml::sfUdpSocket
}

impl UdpSocket {
    pub fn new() -> UdpSocket {
        unsafe {
            UdpSocket { socket : csfml::sfUdpSocket_create()}
        }
    }

    pub fn set_blocking(&self, blocking : bool) -> () {
        match blocking  {
            true        => unsafe {csfml::sfUdpSocket_setBlocking(self.socket, 1)},
            false       => unsafe {csfml::sfUdpSocket_setBlocking(self.socket, 0)},
        }
    }

    pub fn is_blocking(&self) -> bool {
        match unsafe {csfml::sfUdpSocket_isBlocking(self.socket)} {
            0 => false,
            _ => true
        }
    }

    pub fn get_local_port(&self) -> u16 {
        unsafe {
            csfml::sfUdpSocket_getLocalPort(self.socket)
        }
    }

    pub fn bind(&self, port : u16) -> SocketStatus {
        unsafe {
            csfml::sfUdpSocket_bind(self.socket, port)
        }

    }

    pub fn unbind(&self) -> () {
        unsafe {
            csfml::sfUdpSocket_unbind(self.socket)
        }
    }

    pub fn send(&self, data : ~[i8], address : &ip_address::IpAddress, port : u16) -> SocketStatus {
        unsafe {
            csfml::sfUdpSocket_send(self.socket, vec::raw::to_ptr(data), data.len() as size_t, address.unwrap(), port)
        }
    }

    pub fn receive(&self, maxSize : size_t) -> (~[i8], SocketStatus, size_t, ip_address::IpAddress, u16) {
        unsafe {
            let s : size_t = 0;
            let datas : *i8 = ptr::null();
            let addr : *ip_address::csfml::sfIpAddress = ptr::null();
            let port : u16 = 0;
            let stat : SocketStatus = csfml::sfUdpSocket_receive(self.socket, datas, maxSize, &s, addr, &port);
            (vec::raw::from_buf_raw(datas, s as uint), stat, s, ip_address::IpAddress::wrap(*addr), port)
        }
    }
    
    pub fn send_packet(&self, packet : &packet::Packet, address : &ip_address::IpAddress, port : u16) -> SocketStatus {
        unsafe {
            csfml::sfUdpSocket_sendPacket(self.socket, packet.unwrap(), address.unwrap(), port)
        }
    }

    pub fn receive_packet(&self) -> (packet::Packet, SocketStatus, ip_address::IpAddress, u16) {
        unsafe {
            let pack : *packet::csfml::sfPacket = ptr::null();
            let addr : *ip_address::csfml::sfIpAddress = ptr::null();
            let port : u16 = 0;
            let stat = csfml::sfUdpSocket_receivePacket(self.socket, pack, addr, &port);
            (packet::Packet::wrap(pack), stat, ip_address::IpAddress::wrap(*addr), port)
        }
    }

    pub fn max_datagram_size() -> u32 {
        unsafe {
            csfml::sfUdpSocket_maxDatagramSize()
        }
    }

    #[doc(hidden)]
    pub fn wrap(socket : *csfml::sfUdpSocket) -> UdpSocket {
        UdpSocket {socket : socket}
    }
    
    #[doc(hidden)]
    pub fn unwrap(&self) -> *csfml::sfUdpSocket {
        self.socket
    }
}

impl Drop for UdpSocket {
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfUdpSocket_destroy(self.socket)
        }
    }
}