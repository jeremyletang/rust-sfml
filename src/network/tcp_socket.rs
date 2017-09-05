use csfml_system_sys::sfBool;
use network::{IpAddress, Packet, SocketStatus};
use network::csfml_network_sys as ffi;
use sf_bool_ext::SfBoolExt;
use std::{mem, ptr};
use system::Time;

/// Specialized socket using the TCP protocol
pub struct TcpSocket {
    socket: *mut ffi::sfTcpSocket,
}

impl TcpSocket {
    /// Create a new TCP socket
    pub fn new() -> TcpSocket {
        let tcp = unsafe { ffi::sfTcpSocket_create() };
        assert!(!tcp.is_null(), "Failed to create TcpSocket");
        TcpSocket { socket: tcp }
    }

    /// Set the blocking state of a TCP listener
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
    /// # Arguments
    /// * blocking - true to set the socket as blocking, false for non-blocking
    pub fn set_blocking(&mut self, blocking: bool) {
        unsafe { ffi::sfTcpSocket_setBlocking(self.socket, sfBool::from_bool(blocking)) }
    }

    /// Tell whether a TCP socket is in blocking or non-blocking mode
    ///
    /// Return true if the socket is blocking, false otherwise
    pub fn is_blocking(&self) -> bool {
        unsafe { ffi::sfTcpSocket_isBlocking(self.socket) }.to_bool()
    }

    /// Get the port to which a TCP socket is bound locally
    ///
    /// If the socket is not bound to a port, this function
    /// returns 0.
    ///
    /// Return the port to which the socket is bound
    pub fn local_port(&self) -> u16 {
        unsafe { ffi::sfTcpSocket_getLocalPort(self.socket) }
    }

    /// Get the address of the connected peer of a TCP socket
    ///
    /// It the socket is not connected, this function returns a "none" (invalid) ip address.
    ///
    /// Return the address of the remote peer
    pub fn remote_address(&self) -> IpAddress {
        unsafe { IpAddress::from_raw(ffi::sfTcpSocket_getRemoteAddress(self.socket)) }
    }

    /// Get the port of the connected peer to which
    /// a TCP socket is connected
    ///
    /// If the socket is not connected, this function returns 0.
    ///
    /// Return the remote port to which the socket is connected
    pub fn remote_port(&self) -> u16 {
        unsafe { ffi::sfTcpSocket_getRemotePort(self.socket) }
    }

    /// Connect a TCP socket to a remote peer
    ///
    /// In blocking mode, this function may take a while, especially
    /// if the remote peer is not reachable. The last parameter allows
    /// you to stop trying to connect after a given timeout.
    /// If the socket was previously connected, it is first disconnected.
    ///
    /// # Arguments
    /// * remoteAddress - Address of the remote peer
    /// * remotePort - Port of the remote peer
    /// * timeout - Maximum time to wait
    pub fn connect(&self, host: &IpAddress, port: u16, timeout: Time) -> SocketStatus {
        unsafe {
            mem::transmute(ffi::sfTcpSocket_connect(
                self.socket,
                host.raw(),
                port,
                timeout.raw(),
            ))
        }
    }

    /// Disconnect a TCP socket from its remote peer
    ///
    /// This function gracefully closes the connection. If the
    /// socket is not connected, this function has no effect.
    pub fn disconnect(&mut self) {
        unsafe { ffi::sfTcpSocket_disconnect(self.socket) }
    }

    /// Send raw data to the remote peer of a TCP socket
    ///
    /// # Arguments
    /// * data - Vector of the sequence of bytes to send
    ///
    /// Return the status code
    pub fn send(&self, data: &[i8]) -> SocketStatus {
        unsafe {
            let status = ffi::sfTcpSocket_send(self.socket, data.as_ptr() as *const _, data.len());
            mem::transmute(status)
        }
    }

    /// Receive raw data from the remote peer of a TCP socket
    ///
    /// In blocking mode, this function will wait until some
    /// bytes are actually received.
    /// This function will fail if the socket is not connected.
    ///
    /// # Arguments
    /// * destination - The slice to write the bytes into
    ///
    /// Returns a tuple containing the socket status, and the actual number of bytes received.
    pub fn receive(&self, destination: &mut [u8]) -> (SocketStatus, usize) {
        unsafe {
            let mut actual_read_len = 0;
            let status = ffi::sfTcpSocket_receive(
                self.socket,
                destination.as_mut_ptr() as *mut _,
                destination.len(),
                &mut actual_read_len,
            );
            let status: SocketStatus = mem::transmute(status);
            (status, actual_read_len)
        }
    }

    /// Send a formatted packet of data to the remote peer of a TCP socket
    ///
    /// # Arguments
    /// * packet - Packet to send
    ///
    /// Return the socket status
    pub fn send_packet(&self, packet: &mut Packet) -> SocketStatus {
        unsafe {
            mem::transmute(
                ffi::sfTcpSocket_sendPacket(self.socket, packet.raw_mut()) as i32,
            )
        }
    }

    /// Receive a formatted packet of data from the remote peer
    ///
    /// In blocking mode, this function will wait until the whole packet
    /// has been received.
    /// This function will fail if the socket is not connected.
    ///
    /// Return a packet and a socket status
    pub fn receive_packet(&self) -> (Packet, SocketStatus) {
        unsafe {
            let pack: *mut ffi::sfPacket = ptr::null_mut();
            let stat: SocketStatus =
                mem::transmute(ffi::sfTcpSocket_receivePacket(self.socket, pack) as i32);
            (Packet::from_raw(pack), stat)
        }
    }
}

impl Default for TcpSocket {
    fn default() -> Self {
        Self::new()
    }
}

impl Raw for TcpSocket {
    type Raw = *const ffi::sfTcpSocket;
    fn raw(&self) -> Self::Raw {
        self.socket
    }
}

impl RawMut for TcpSocket {
    type RawMut = *mut ffi::sfTcpSocket;
    fn raw_mut(&mut self) -> Self::RawMut {
        self.socket
    }
}

impl Drop for TcpSocket {
    fn drop(&mut self) {
        unsafe { ffi::sfTcpSocket_destroy(self.socket) }
    }
}
