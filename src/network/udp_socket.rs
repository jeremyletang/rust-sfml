use std::{ptr, mem};

use system::raw_conv::{Raw, RawMut, FromRaw};
use network::{Packet, IpAddress, SocketStatus};

use csfml_system_sys::sfBool;
use csfml_network_sys as ffi;
use ext::sf_bool_ext::SfBoolExt;

/// Specialized socket using the UDP protocol.
pub struct UdpSocket {
    socket: *mut ffi::sfUdpSocket,
}

impl UdpSocket {
    /// Create a new UDP socket
    pub fn new() -> UdpSocket {
        let udp = unsafe { ffi::sfUdpSocket_create() };
        if udp.is_null() {
            panic!("sfUdpSocket_create returned null.")
        } else {
            UdpSocket { socket: udp }
        }
    }

    /// Set the blocking state of a UDP listener
    ///
    /// In blocking mode, calls will not return until they have
    /// completed their task. For example, a call to
    /// `UdpSocket::receive` in blocking mode won't return until
    /// new data was actually received.
    /// In non-blocking mode, calls will always return immediately,
    /// using the return code to signal whether there was data
    /// available or not.
    /// By default, all sockets are blocking.
    ///
    /// #Arguments
    /// blocking - true to set the socket as blocking, false for non-blocking
    pub fn set_blocking(&self, blocking: bool) {
        unsafe { ffi::sfUdpSocket_setBlocking(self.socket, sfBool::from_bool(blocking)) }
    }

    /// Tell whether a UDP socket is in blocking or non-blocking mode
    ///
    /// Return true if the socket is blocking, false otherwise
    pub fn is_blocking(&self) -> bool {
        unsafe { ffi::sfUdpSocket_isBlocking(self.socket) }.to_bool()
    }

    /// Get the port to which a UDP socket is bound locally
    ///
    /// If the socket is not bound to a port, this function
    /// returns 0.
    ///
    /// Return the port to which the socket is bound
    pub fn local_port(&self) -> u16 {
        unsafe { ffi::sfUdpSocket_getLocalPort(self.socket) }
    }

    /// Bind a UDP socket to a specific port
    ///
    /// Binding the socket to a port is necessary for being
    /// able to receive data on that port.
    /// You can use the special value 0 to tell the
    /// system to automatically pick an available port, and then
    /// call `UdpSocket::local_port` to retrieve the chosen port.
    ///
    /// # Arguments
    /// * port - Port to bind the socket to
    ///
    /// Return the tatus code
    pub fn bind(&self, port: u16, address: IpAddress) -> SocketStatus {
        unsafe { mem::transmute(ffi::sfUdpSocket_bind(self.socket, port, address.raw()) as i32) }
    }

    /// Unbind a UDP socket from the local port to which it is bound
    ///
    /// The port that the socket was previously using is immediately
    /// available after this function is called. If the
    /// socket is not bound to a port, this function has no effect.
    pub fn unbind(&self) {
        unsafe { ffi::sfUdpSocket_unbind(self.socket) }
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
            mem::transmute(ffi::sfUdpSocket_send(self.socket,
                                                 data.as_ptr() as *const _,
                                                 data.len(),
                                                 address.raw(),
                                                 port) as i32)
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
    ///
    /// Returns the socket status, the actual number of bytes received, the ip address of the
    /// sender, and the port of the sender.
    pub fn receive(&self, destination: &mut [u8]) -> (SocketStatus, usize, IpAddress, u16) {
        unsafe {
            let mut actual_read_len = 0;
            let mut addr = ::std::mem::zeroed();
            let mut port = 0;
            let status = ffi::sfUdpSocket_receive(self.socket,
                                                  destination.as_mut_ptr() as *mut _,
                                                  destination.len(),
                                                  &mut actual_read_len,
                                                  &mut addr,
                                                  &mut port);
            let status = mem::transmute(status);
            (status, actual_read_len, IpAddress::from_raw(addr), port)
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
    pub fn send_packet(&self, packet: &mut Packet, address: &IpAddress, port: u16) -> SocketStatus {
        unsafe {
            mem::transmute(ffi::sfUdpSocket_sendPacket(self.socket,
                                                       packet.raw_mut(),
                                                       address.raw(),
                                                       port) as i32)
        }
    }

    /// Receive a formatted packet of data from a remote peer with a UDP socket
    ///
    /// In blocking mode, this function will wait until the whole packet
    /// has been received.
    pub fn receive_packet(&self) -> (Packet, SocketStatus, IpAddress, u16) {
        unsafe {
            let pack: *mut ffi::sfPacket = ptr::null_mut();
            let addr: *mut ffi::sfIpAddress = ptr::null_mut();
            let mut port: u16 = 0;
            let status = ffi::sfUdpSocket_receivePacket(self.socket, pack, addr, &mut port);
            (Packet::from_raw(pack), mem::transmute(status), IpAddress::from_raw(*addr), port)
        }
    }

    /// Return the maximum number of bytes that can be
    /// sent in a single UDP datagram
    ///
    /// Return the maximum size of a UDP datagram (message)
    pub fn max_datagram_size() -> u32 {
        unsafe { ffi::sfUdpSocket_maxDatagramSize() }
    }
}

impl Default for UdpSocket {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for UdpSocket {
    fn drop(&mut self) {
        unsafe { ffi::sfUdpSocket_destroy(self.socket) }
    }
}
