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
