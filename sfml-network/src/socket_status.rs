// Rust-SFML - Copyright (c) 2013 Letang Jeremy.
//
// The original software, SFML library, is provided by Laurent Gomila.
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from
// the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not claim
//    that you wrote the original software. If you use this software in a product,
//    an acknowledgment in the product documentation would be appreciated but is
//    not required.
//
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

//! Status codes that may be returned by socket functions.

/// Status codes that may be returned by socket functions.
#[repr(u32)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub enum SocketStatus {
    /// The socket has sent / received the data.
    Done = 0,
    /// The socket is not ready to send / receive data yet.
    NotReady = 1,
    /// The socket sent a part of the data.
    Partial = 2,
    /// The TCP socket has been disconnected.
    Disconnected = 3,
    /// An unexpected error happened.
    Error = 4,
}
