/*!
* Socket-based communication
*
* utilities and higher-level network protocols (HTTP, FTP)
*
*/

#[cfg(mac_dylib)]
#[cfg(target_os="linux")]
#[cfg(target_os="win32")]
mod platform {
    #[link_args="-lcsfml-network"]
    extern {}
}
