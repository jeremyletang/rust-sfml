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
*
*/

pub use self::ip_address::IpAddress;
pub use self::packet::Packet;
pub use self::socket_status::{SocketStatus, SocketNotReady, SocketDisconnected, SocketError};
// pub use self::tcp_socket::TcpSocket;
// pub use self::udp_socket::*;
// pub use self::tcp_listener::*;
pub use self::ftp::*;
pub use self::http::*;

#[doc(hidden)]
#[cfg(target_os="macos")]
#[cfg(target_os="linux")]
#[cfg(target_os="win32")]
mod platform {
    #[link_args="-lcsfml-network"]
    extern {}
}

pub mod ip_address;
pub mod packet;
pub mod socket_status;
pub mod tcp_socket;
pub mod udp_socket;
pub mod tcp_listener;
pub mod ftp;
pub mod http;
