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
* Specialized socket using the UDP protocol.
*
*
*
*
*/

use std::{ptr, vec, cast};
use std::libc::size_t;

use traits::wrappable::Wrappable;
use network::{packet, ip_address};
use network::socket_status::SocketStatus;
use sfml_types::*;

#[doc(hidden)]
pub mod ffi {
    
    use std::libc::{size_t, c_void};

    use network::socket_status;
    use network::ip_address;
    use sfml_types::SfBool;
    use network::packet;

    pub struct sfUdpSocket {
        This : *c_void
    }

    extern "C" {
        pub fn sfUdpSocket_create() -> *sfUdpSocket;
        pub fn sfUdpSocket_destroy(socket : *sfUdpSocket) -> ();
        pub fn sfUdpSocket_setBlocking(socket : *sfUdpSocket, blocking : SfBool) -> ();
        pub fn sfUdpSocket_isBlocking(socket : *sfUdpSocket) -> SfBool;
        pub fn sfUdpSocket_getLocalPort(socket : *sfUdpSocket) -> u16;
        pub fn sfUdpSocket_bind(socket : *sfUdpSocket, port : u16) -> socket_status::ffi::SocketStatus;
        pub fn sfUdpSocket_unbind(socket : *sfUdpSocket) -> ();
        pub fn sfUdpSocket_send(socket : *sfUdpSocket, data : *i8, size : size_t, address : ip_address::ffi::sfIpAddress, port : u16) -> socket_status::ffi::SocketStatus;
        pub fn sfUdpSocket_receive(socket : *sfUdpSocket, data : *i8, maxSize : size_t, sizeReceived : *size_t, address : *ip_address::ffi::sfIpAddress, port : *u16) -> socket_status::ffi::SocketStatus;
        pub fn sfUdpSocket_sendPacket(socket : *sfUdpSocket, packet : *packet::ffi::sfPacket, address : ip_address::ffi::sfIpAddress, port : u16) -> socket_status::ffi::SocketStatus;
        pub fn sfUdpSocket_receivePacket(socket : *sfUdpSocket, packet : *packet::ffi::sfPacket, address : *ip_address::ffi::sfIpAddress, port : *u16) -> socket_status::ffi::SocketStatus;
        pub fn sfUdpSocket_maxDatagramSize() -> u32;
    }
}

pub struct UdpSocket {
    #[doc(hidden)]
    priv socket : *ffi::sfUdpSocket
}

impl UdpSocket {
    /**
    * Create a new UDP socket
    *
    * Return a new option to UdpSocket object or None
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn new() -> Option<UdpSocket> {
        let udp = unsafe { ffi::sfUdpSocket_create() };
        if ptr::is_null(udp) {
            None
        }
        else {
            Some(UdpSocket {
                socket : udp
            })
        }
    }

    /**
    * Set the blocking state of a UDP listener
    *
    * In blocking mode, calls will not return until they have
    * completed their task. For example, a call to
    * sfUDPSocket_receive in blocking mode won't return until
    * new data was actually received.
    * In non-blocking mode, calls will always return immediately,
    * using the return code to signal whether there was data
    * available or not.
    * By default, all sockets are blocking.
    *
    * #Arguments
    * blocking - true to set the socket as blocking, false for non-blocking
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_blocking(&self, blocking : bool) -> () {
        unsafe {
            match blocking  {
                true        => ffi::sfUdpSocket_setBlocking(self.socket, SFTRUE),
                false       => ffi::sfUdpSocket_setBlocking(self.socket, SFFALSE)
            }
        }
    }

    /**
    * Tell whether a UDP socket is in blocking or non-blocking mode
    *
    * Return true if the socket is blocking, false otherwise
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_blocking(&self) -> bool {
        match unsafe { ffi::sfUdpSocket_isBlocking(self.socket) } {
            SFFALSE  => false,
            SFTRUE   => true,
            _        => unreachable!()
        }
    }

    /**
    * Get the port to which a UDP socket is bound locally
    *
    * If the socket is not bound to a port, this function
    * returns 0.
    * 
    * Return the port to which the socket is bound
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_local_port(&self) -> u16 {
        unsafe {
            ffi::sfUdpSocket_getLocalPort(self.socket)
        }
    }

    /**
    * Bind a UDP socket to a specific port
    *
    * Binding the socket to a port is necessary for being
    * able to receive data on that port.
    * You can use the special value 0 to tell the
    * system to automatically pick an available port, and then
    * call sfUdpSocket_getLocalPort to retrieve the chosen port.
    * 
    * # Arguments
    * * port - Port to bind the socket to
    *
    * Return the tatus code
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn bind(&self, port : u16) -> SocketStatus {
        unsafe {
            cast::transmute(ffi::sfUdpSocket_bind(self.socket, port) as i8)
        }

    }

    /**
    * Unbind a UDP socket from the local port to which it is bound
    *
    * The port that the socket was previously using is immediately
    * available after this function is called. If the
    * socket is not bound to a port, this function has no effect.
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn unbind(&self) -> () {
        unsafe {
            ffi::sfUdpSocket_unbind(self.socket)
        }
    }

    /**
    * Send raw data to a remote peer with a UDP socket
    *
    * Make sure that size is not greater than
    * max_datagram_size(), otherwise this function will
    * fail and no data will be sent.
    *
    * # Arguments
    * * data - Vector to the sequence of bytes to send
    * * remoteAddress - Address of the receiver
    * * remotePort - Port of the receiver to send the data to
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn send(&self, data : ~[i8], address : &ip_address::IpAddress, port : u16) -> SocketStatus {
        unsafe {
            cast::transmute(ffi::sfUdpSocket_send(self.socket, vec::raw::to_ptr(data), data.len() as size_t, address.unwrap(), port) as i8)
        }
    }

    /**
    * Receive raw data from a remote peer with a UDP socket
    *
    * In blocking mode, this function will wait until some
    * bytes are actually received.
    * Be careful to use a buffer which is large enough for
    * the data that you intend to receive, if it is too small
    * then an error will be returned and *all* the data will
    * be lost.
    *
    * # Arguments
    * * size - Maximum number of bytes that can be received
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn receive(&self, max_size : size_t) -> (~[i8], SocketStatus, size_t, ip_address::IpAddress, u16) {
        unsafe {
            let s : size_t = 0;
            let datas : *i8 = ptr::null();
            let addr : *ip_address::ffi::sfIpAddress = ptr::null();
            let port : u16 = 0;
            let stat : SocketStatus = cast::transmute(ffi::sfUdpSocket_receive(self.socket, datas, max_size, &s, addr, &port) as i8);
            (vec::raw::from_buf_raw(datas, s as uint), stat, s, Wrappable::wrap(*addr), port)
        }
    }
    
    /**
    * Send a formatted packet of data to a remote peer with a UDP socket
    *
    * Make sure that the packet size is not greater than
    * max_datagram_size(), otherwise this function will
    * fail and no data will be sent.
    *
    * # Arguments
    * * packet - Packet to send
    * * remoteAddress - Address of the receiver
    * * remotePort - Port of the receiver to send the data to
    *
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn send_packet(&self, packet : &packet::Packet, address : &ip_address::IpAddress, port : u16) -> SocketStatus {
        unsafe {
            cast::transmute(ffi::sfUdpSocket_sendPacket(self.socket, packet.unwrap(), address.unwrap(), port) as i8)
        }
    }

    /**
    * Receive a formatted packet of data from a remote peer with a UDP socket
    *
    * In blocking mode, this function will wait until the whole packet
    * has been received.
    *
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn receive_packet(&self) -> (packet::Packet, SocketStatus, ip_address::IpAddress, u16) {
        unsafe {
            let pack : *packet::ffi::sfPacket = ptr::null();
            let addr : *ip_address::ffi::sfIpAddress = ptr::null();
            let port : u16 = 0;
            let stat : SocketStatus = cast::transmute(ffi::sfUdpSocket_receivePacket(self.socket, pack, addr, &port) as i8);
            (Wrappable::wrap(pack), stat, Wrappable::wrap(*addr), port)
        }
    }

    /**
    * Return the maximum number of bytes that can be
    * sent in a single UDP datagram
    *
    * Return the maximum size of a UDP datagram (message)
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn max_datagram_size() -> u32 {
        unsafe {
            ffi::sfUdpSocket_maxDatagramSize()
        }
    }
}

impl Wrappable<*ffi::sfUdpSocket> for UdpSocket {
    fn wrap(socket : *ffi::sfUdpSocket) -> UdpSocket {
        UdpSocket {
            socket : socket
        }
    }
    
    fn unwrap(&self) -> *ffi::sfUdpSocket {
        self.socket
    }
}

impl Drop for UdpSocket {
    #[fixed_stack_segment] #[inline(never)]
    fn drop(&mut self) -> () {
        unsafe {
            ffi::sfUdpSocket_destroy(self.socket)
        }
    }
}
