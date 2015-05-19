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
* Socket-based communication
*
* utilities and higher-level network protocols (HTTP, FTP)
*/

pub use network::ip_address::IpAddress;
pub use network::packet::Packet;
pub use network::tcp_socket::TcpSocket;
pub use network::udp_socket::UdpSocket;
pub use network::tcp_listener::TcpListener;
pub use network::socket_status::SocketStatus;
pub use network::ftp::Ftp;
pub use network::http::Http;


#[cfg(any(target_os="macos", target_os="linux", target_os="windows"))]
mod platform {
    #[link(name = "csfml-network")]
    extern {}
}

mod ip_address;
mod packet;
mod socket_status;
mod tcp_socket;
mod udp_socket;
mod tcp_listener;
pub mod ftp;
pub mod http;
