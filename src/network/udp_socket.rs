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

//! Specialized socket using the UDP protocol

use std::{ptr, slice, mem};
use libc::size_t;
use std::vec::Vec;

use traits::Wrappable;
use network::{Packet, IpAddress, SocketStatus};

use ffi::sfml_types::{SFTRUE, SFFALSE};
use ffi = ffi::network::udp_socket;

/// Specialized socket using the UDP protocol.
pub struct UdpSocket {
    #[doc(hidden)]
    socket: *mut ffi::sfUdpSocket
}

impl UdpSocket {
    /// Create a new UDP socket
    ///
    /// Return Some(UdpSocket) or None
    pub fn new() -> Option<UdpSocket> {
        let udp = unsafe { ffi::sfUdpSocket_create() };
        if udp.is_null() {
            None
        }
        else {
            Some(UdpSocket {
                socket: udp
            })
        }
    }

    /// Set the blocking state of a UDP listener
    ///
    /// In blocking mode, calls will not return until they have
    /// completed their task. For example, a call to
    /// sfUDPSocket_receive in blocking mode won't return until
    /// new data was actually received.
    /// In non-blocking mode, calls will always return immediately,
    /// using the return code to signal whether there was data
    /// available or not.
    /// By default, all sockets are blocking.
    ///
    /// #Arguments
    /// blocking - true to set the socket as blocking, false for non-blocking
    pub fn set_blocking(&self, blocking: bool) -> () {
        unsafe {
            match blocking  {
                true        => ffi::sfUdpSocket_setBlocking(self.socket, SFTRUE),
                false       => ffi::sfUdpSocket_setBlocking(self.socket, SFFALSE)
            }
        }
    }

    /// Tell whether a UDP socket is in blocking or non-blocking mode
    ///
    /// Return true if the socket is blocking, false otherwise
    pub fn is_blocking(&self) -> bool {
        match unsafe { ffi::sfUdpSocket_isBlocking(self.socket) } {
            SFFALSE  => false,
            SFTRUE   => true
        }
    }

    /// Get the port to which a UDP socket is bound locally
    ///
    /// If the socket is not bound to a port, this function
    /// returns 0.
    ///
    /// Return the port to which the socket is bound
    pub fn get_local_port(&self) -> u16 {
        unsafe {
            ffi::sfUdpSocket_getLocalPort(self.socket)
        }
    }

    /// Bind a UDP socket to a specific port
    ///
    /// Binding the socket to a port is necessary for being
    /// able to receive data on that port.
    /// You can use the special value 0 to tell the
    /// system to automatically pick an available port, and then
    /// call sfUdpSocket_getLocalPort to retrieve the chosen port.
    ///
    /// # Arguments
    /// * port - Port to bind the socket to
    ///
    /// Return the tatus code
    pub fn bind(&self, port: u16) -> SocketStatus {
        unsafe {
            mem::transmute(ffi::sfUdpSocket_bind(self.socket, port) as i8)
        }
    }

    /// Unbind a UDP socket from the local port to which it is bound
    ///
    /// The port that the socket was previously using is immediately
    /// available after this function is called. If the
    /// socket is not bound to a port, this function has no effect.
    pub fn unbind(&self) -> () {
        unsafe {
            ffi::sfUdpSocket_unbind(self.socket)
        }
    }

    /// Send raw data to a remote peer with a UDP socket
    ///
    /// Make sure that size is not greater than
    /// max_datagram_size(), otherwise this function will
    /// fail and no data will be sent.
    ///
    /// # Arguments
    /// * data - Vector to the sequence of bytes to send
    /// * remoteAddress - Address of the receiver
    /// * remotePort - Port of the receiver to send the data to
    pub fn send(&self, data: &[i8], address: &IpAddress, port: u16) -> SocketStatus {
        unsafe {
            mem::transmute(ffi::sfUdpSocket_send(self.socket, data.as_ptr() as *mut i8, data.len() as size_t, address.unwrap(), port) as i8)
        }
    }

    /// Receive raw data from a remote peer with a UDP socket
    ///
    /// In blocking mode, this function will wait until some
    /// bytes are actually received.
    /// Be careful to use a buffer which is large enough for
    /// the data that you intend to receive, if it is too small
    /// then an error will be returned and *all* the data will
    /// be lost.
    ///
    /// # Arguments
    /// * size - Maximum number of bytes that can be received
    pub fn receive(&self, max_size: size_t) -> (Vec<i8>, SocketStatus, size_t, IpAddress, u16) {
        unsafe {
            let mut s: size_t = 0;
            let datas: *mut i8 = ptr::mut_null();
            let addr: *mut ::ffi::network::ip_address::sfIpAddress = ptr::mut_null();
            let mut port: u16 = 0;
            let stat: SocketStatus = mem::transmute(ffi::sfUdpSocket_receive(self.socket, datas, max_size, &mut s, addr, &mut port) as i8);
            (slice::raw::buf_as_slice(datas as *const i8, s as uint, Vec::from_slice), stat, s, Wrappable::wrap(*addr), port)
        }
    }

    /// Send a formatted packet of data to a remote peer with a UDP socket
    ///
    /// Make sure that the packet size is not greater than
    /// max_datagram_size(), otherwise this function will
    /// fail and no data will be sent.
    ///
    /// # Arguments
    /// * packet - Packet to send
    /// * remoteAddress - Address of the receiver
    /// * remotePort - Port of the receiver to send the data to
    pub fn send_packet(&self, packet: &Packet, address: &IpAddress, port: u16) -> SocketStatus {
        unsafe {
            mem::transmute(ffi::sfUdpSocket_sendPacket(self.socket, packet.unwrap(), address.unwrap(), port) as i8)
        }
    }

    /// Receive a formatted packet of data from a remote peer with a UDP socket
    ///
    /// In blocking mode, this function will wait until the whole packet
    /// has been received.
    pub fn receive_packet(&self) -> (Packet, SocketStatus, IpAddress, u16) {
        unsafe {
            let pack: *mut ::ffi::network::packet::sfPacket = ptr::mut_null();
            let addr: *mut ::ffi::network::ip_address::sfIpAddress = ptr::mut_null();
            let mut port: u16 = 0;
            let stat: SocketStatus = mem::transmute(ffi::sfUdpSocket_receivePacket(self.socket, pack, addr, &mut port) as i8);
            (Wrappable::wrap(pack), stat, Wrappable::wrap(*addr), port)
        }
    }

    /// Return the maximum number of bytes that can be
    /// sent in a single UDP datagram
    ///
    /// Return the maximum size of a UDP datagram (message)
    pub fn max_datagram_size() -> u32 {
        unsafe {
            ffi::sfUdpSocket_maxDatagramSize()
        }
    }
}

impl Wrappable<*mut ffi::sfUdpSocket> for UdpSocket {
    fn wrap(socket: *mut ffi::sfUdpSocket) -> UdpSocket {
        UdpSocket {
            socket: socket
        }
    }

    fn unwrap(&self) -> *mut ffi::sfUdpSocket {
        self.socket
    }
}

impl Drop for UdpSocket {
    fn drop(&mut self) -> () {
        unsafe {
            ffi::sfUdpSocket_destroy(self.socket)
        }
    }
}
