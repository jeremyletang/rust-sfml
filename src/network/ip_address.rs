use csfml_network_sys as ffi;
use std::ffi::{CStr, CString};
use std::str;
use system::Time;
use system::raw_conv::{FromRaw, Raw};

/// Encapsulate an IPv4 network address.
#[derive(Clone, Copy)]
pub struct IpAddress {
    ip: ffi::sfIpAddress,
}

impl IpAddress {
    /// Create an address from a string
    ///
    /// Here address can be either a decimal address
    /// (ex: "192.168.1.56") or a network name (ex: "localhost").
    ///
    /// # Arguments
    /// * address - IP address or network name
    ///
    /// Return Resulting address
    pub fn from_string(address: &str) -> IpAddress {
        let c_address = CString::new(address.as_bytes()).unwrap();
        IpAddress { ip: unsafe { ffi::sfIpAddress_fromString(c_address.as_ptr()) } }
    }

    /// Create an address from 4 bytes
    ///
    /// Calling fromBytes(a, b, c, d) is equivalent
    /// to calling fromString("a.b.c.d"), but safer
    /// as it doesn't have to parse a string to get the address
    /// components.
    ///
    /// # Arguments
    /// * byte0 - First byte of the address
    /// * byte1 - Second byte of the address
    /// * byte2 - Third byte of the address
    /// * byte3 - Fourth byte of the address
    ///
    /// Return the resulting address
    pub fn from_bytes(byte0: u8, byte1: u8, byte2: u8, byte3: u8) -> IpAddress {
        IpAddress { ip: unsafe { ffi::sfIpAddress_fromBytes(byte0, byte1, byte2, byte3) } }
    }

    /// Construct an address from a 32-bits integer
    ///
    /// This function uses the internal representation of
    /// the address directly. It should be used for optimization
    /// purposes, and only if you got that representation from
    /// `IpAddress::to_integer`.
    ///
    /// # Arguments
    /// * address - 4 bytes of the address packed into a 32-bits integer
    ///
    /// Return the resulting address
    pub fn from_integer(address: u32) -> IpAddress {
        IpAddress { ip: unsafe { ffi::sfIpAddress_fromInteger(address) } }
    }

    /// Get a string representation of an address
    ///
    /// The returned string is the decimal representation of the
    /// IP address (like "192.168.1.56"), even if it was constructed
    /// from a host name.
    ///
    /// Return a string representation of the address
    pub fn to_string(&self) -> String {
        unsafe {
            let ptr = &self.ip as *const _ as *const _;
            str::from_utf8(CStr::from_ptr(ptr).to_bytes()).unwrap().into()
        }
    }

    /// Get an integer representation of the address
    ///
    /// The returned number is the internal representation of the
    /// address, and should be used for optimization purposes only
    /// (like sending the address through a socket).
    /// The integer produced by this function can then be converted
    /// back to a IpAddress with From_integer.
    ///
    /// Return a 32-bits unsigned integer representation of the address
    pub fn to_integer(&self) -> u32 {
        unsafe { ffi::sfIpAddress_toInteger(self.ip) }
    }

    /// Get the computer's local address
    ///
    /// The local address is the address of the computer from the
    /// LAN point of view, i.e. something like 192.168.1.56. It is
    /// meaningful only for communications over the local network.
    /// Unlike `IpAddress::public_address`, this function is fast
    /// and may be used safely anywhere.
    ///
    /// Return the local IP address of the computer
    pub fn local_address() -> IpAddress {
        IpAddress { ip: unsafe { ffi::sfIpAddress_getLocalAddress() } }
    }

    /// Get the computer's public address
    ///
    /// The public address is the address of the computer from the
    /// internet point of view, i.e. something like 89.54.1.169.
    /// It is necessary for communications over the world wide web.
    /// The only way to get a public address is to ask it to a
    /// distant website; as a consequence, this function depends on
    /// both your network connection and the server, and may be
    /// very slow. You should use it as few as possible. Because
    /// this function depends on the network connection and on a distant
    /// server, you may use a time limit if you don't want your program
    /// to be possibly stuck waiting in case there is a problem; use
    /// 0 to deactivate this limit.
    ///
    /// Return the public IP address of the computer
    pub fn public_address(timeout: &Time) -> IpAddress {
        IpAddress { ip: unsafe { ffi::sfIpAddress_getPublicAddress(timeout.raw()) } }
    }
}

impl Raw for IpAddress {
    type Raw = ffi::sfIpAddress;
    fn raw(&self) -> Self::Raw {
        self.ip
    }
}

impl FromRaw for IpAddress {
    type RawFrom = ffi::sfIpAddress;
    unsafe fn from_raw(raw: Self::RawFrom) -> Self {
        IpAddress { ip: raw }
    }
}

#[test]
fn ip_to_string() {
    let ip = IpAddress::from_integer(101010);
    assert_eq!(ip.to_string(), "0.1.138.146");
}
