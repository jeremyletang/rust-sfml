use csfml_network_sys as ffi;
use csfml_system_sys::sfBool;
use ext::sf_bool_ext::SfBoolExt;
use network::{IpAddress, SocketStatus, TcpSocket};
use std::mem;
use system::raw_conv::{Raw, RawMut};

/// Socket that listens to new TCP connections
pub struct TcpListener {
    listener: *mut ffi::sfTcpListener,
}

impl TcpListener {
    /// Create a new TCP listener
    pub fn new() -> TcpListener {
        let list = unsafe { ffi::sfTcpListener_create() };
        assert!(!list.is_null(), "Failed to create TcpListener");
        TcpListener { listener: list }
    }

    /// Set the blocking state of a TCP listener
    ///
    /// In blocking mode, calls will not return until they have
    /// completed their task. For example, a call to
    /// `TcpListener::accept` in blocking mode won't return until
    /// a new connection was actually received.
    /// In non-blocking mode, calls will always return immediately,
    /// using the return code to signal whether there was data
    /// available or not.
    /// By default, all sockets are blocking.
    ///
    /// # Arguments
    /// * blocking - true to set the socket as blocking, false for non-blocking
    pub fn set_blocking(&mut self, blocking: bool) {
        unsafe { ffi::sfTcpListener_setBlocking(self.listener, sfBool::from_bool(blocking)) }
    }

    /// Tell whether a TCP listener is in blocking or non-blocking mode
    ///
    /// Return true if the socket is blocking, false otherwise
    pub fn is_blocking(&self) -> bool {
        unsafe { ffi::sfTcpListener_isBlocking(self.listener) }.to_bool()
    }

    /// Get the port to which a TCP listener is bound locally
    ///
    /// If the socket is not listening to a port, this function
    /// returns 0.
    ///
    /// Return the port to which the TCP listener is bound
    pub fn local_port(&self) -> u16 {
        unsafe { ffi::sfTcpListener_getLocalPort(self.listener) }
    }

    /// Start listening for connections
    ///
    /// This functions makes the socket listen to the specified
    /// port, waiting for new connections.
    /// If the socket was previously listening to another port,
    /// it will be stopped first and bound to the new port.
    ///
    /// # Arguments
    /// * port - Port to listen for new connections
    ///
    /// Return status code
    pub fn listen(&self, port: u16, address: IpAddress) -> SocketStatus {
        unsafe {
            mem::transmute(ffi::sfTcpListener_listen(self.listener, port, address.raw()) as i32)
        }
    }

    /// Accept a new connection
    ///
    /// If the socket is in blocking mode, this function will
    /// not return until a connection is actually received.
    ///
    /// # Arguments
    /// * connected - Socket that will hold the new connection
    ///
    /// Return status code
    pub fn accept(&self, connected: &mut TcpSocket) -> SocketStatus {
        unsafe {
            mem::transmute(ffi::sfTcpListener_accept(self.listener, &mut connected.raw_mut()) as
                           i32)
        }
    }
}

impl Default for TcpListener {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for TcpListener {
    fn drop(&mut self) {
        unsafe { ffi::sfTcpListener_destroy(self.listener) }
    }
}
