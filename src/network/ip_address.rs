/*
* Rust-SFML - Copyright (c) Letang Jeremy.
*
* The Original software, SFML library, is provided by Laurent Gomila.
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

use std::libc::c_char;
use system::time::Time;
use std::str;
use std::ptr;

#[doc(hidden)]
pub mod csfml{
    
    use std::libc::c_char;
    use system::time;

    pub struct sfIpAddress {
        c1 : c_char,
        c2 : c_char,
        c3 : c_char,
        c4 : c_char,
        c5 : c_char,
        c6 : c_char,
        c7 : c_char,
        c8 : c_char,
        c9 : c_char,
        c10 : c_char,
        c11 : c_char,
        c12 : c_char,
        c13 : c_char,
        c14 : c_char,
        c15 : c_char,
        c16 : c_char
    }

    pub extern "C" {
        fn sfIpAddress_fromString(address : *c_char) -> sfIpAddress;
        fn sfIpAddress_fromBytes(byte0 : u8, byte1 : u8, byte2 : u8, byte3 : u8) -> sfIpAddress;
        fn sfIpAddress_fromInteger(address : u32) -> sfIpAddress;
        fn sfIpAddress_toString(address : sfIpAddress, string : *c_char) -> ();
        fn sfIpAddress_toInteger(address : sfIpAddress) -> u32;
        fn sfIpAddress_getLocalAddress() -> sfIpAddress;
        fn sfIpAddress_getPublicAddress(timeout : time::csfml::sfTime) -> sfIpAddress;
    }
}

pub struct IpAddress{
    priv ip : csfml::sfIpAddress
}

impl IpAddress {
    pub fn new_from_string(address : ~str) -> IpAddress {
        do str::as_c_str(address) |addr_buf| {
            IpAddress { ip : unsafe {csfml::sfIpAddress_fromString(addr_buf)}}
        }
    }

    pub fn mew_from_bytes(byte0 : u8, byte1 : u8, byte2 : u8, byte3 : u8) -> IpAddress {
        IpAddress { ip : unsafe {csfml::sfIpAddress_fromBytes(byte0, byte1, byte2, byte3)}}
    }

    pub fn new_from_integer(address : u32) -> IpAddress {
        IpAddress { ip : unsafe {csfml::sfIpAddress_fromInteger(address)}}
    }

    pub fn to_string(&self) -> ~str {
        unsafe {
            let string : *c_char = ptr::null();
            csfml::sfIpAddress_toString(self.ip, string);
            str::raw::from_c_str(string)
        }
    }

    pub fn to_integer(&self) -> u32 {
        unsafe {
            csfml::sfIpAddress_toInteger(self.ip)
        }
    }

    pub fn get_local_address() -> IpAddress {
        unsafe {
            IpAddress { ip : csfml::sfIpAddress_getLocalAddress()}
        }
    }

    pub fn get_public_address(timeout : &Time) -> IpAddress {
        unsafe {
            IpAddress { ip : csfml::sfIpAddress_getPublicAddress(timeout.unwrap())}
        }
    }
    
    pub fn wrap(ip : csfml::sfIpAddress) -> IpAddress {
        IpAddress {ip : ip}
    }

    pub fn unwrap(&self) -> csfml::sfIpAddress {
        self.ip
    }
}