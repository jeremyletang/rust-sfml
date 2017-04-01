//! Socket-based communication
//!
//! Utilities and higher-level network protocols (HTTP, FTP)
//!
//! # Note from maintainer
//!
//! The `network` module is currently considered a second class citizen, and is not receiving
//! much maintentance. Use it at your own risk!
//! Consider using native Rust solutions for networking.

pub use network::ftp::Ftp;
pub use network::http::Http;
pub use network::ip_address::IpAddress;
pub use network::packet::Packet;
pub use network::socket_status::SocketStatus;
pub use network::tcp_listener::TcpListener;
pub use network::tcp_socket::TcpSocket;
pub use network::udp_socket::UdpSocket;

mod ip_address;
mod packet;
mod socket_status;
mod tcp_socket;
mod udp_socket;
mod tcp_listener;
pub mod ftp;
pub mod http;
