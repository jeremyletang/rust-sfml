//! Socket-based communication
//!
//! Utilities and higher-level network protocols (HTTP, FTP)
//!
//! # Note from maintainer
//!
//! The `network` module is currently considered a second class citizen, and is not receiving
//! much maintentance. Use it at your own risk!
//! Consider using native Rust solutions for networking.

extern crate csfml_network_sys;

pub use self::{
    ftp::Ftp, http::Http, ip_address::IpAddress, packet::Packet, socket_status::SocketStatus,
    tcp_listener::TcpListener, tcp_socket::TcpSocket, udp_socket::UdpSocket,
};

pub mod ftp;
pub mod http;
mod ip_address;
mod packet;
mod socket_status;
mod tcp_listener;
mod tcp_socket;
mod udp_socket;
