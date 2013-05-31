/*!
* Socket-based communication
*
* utilities and higher-level network protocols (HTTP, FTP)
*
*/

#[cfg(target_os="linux")]
#[cfg(target_os="win32")]
mod others {
    #[link_args="-lcsfml-network"]
    extern {}
}
