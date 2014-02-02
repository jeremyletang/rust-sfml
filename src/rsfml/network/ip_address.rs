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
* Encapsulate an IPv4 network address.
*/

use std::str;
use std::ptr;

use traits::Wrappable;
use system::Time;

use ffi = ffi::network::ip_address;

/// Encapsulate an IPv4 network address.
pub struct IpAddress{
    #[doc(hidden)]
    priv ip : ffi::sfIpAddress
}

impl IpAddress {
    /**
    * Create an address from a string
    *
    * Here address can be either a decimal address
    * (ex: "192.168.1.56") or a network name (ex: "localhost").
    *
    * # Arguments
    * * address - IP address or network name
    *
    * Return Resulting address
    */
    pub fn new_from_string(address : &str) -> IpAddress {
        let c_address = address.to_c_str();
        IpAddress {
            ip : unsafe { ffi::sfIpAddress_fromString(c_address.unwrap()) } 
        }
    }

    /**
    * Create an address from 4 bytes
    *
    * Calling fromBytes(a, b, c, d) is equivalent
    * to calling fromString("a.b.c.d"), but safer
    * as it doesn't have to parse a string to get the address
    * components.
    *
    * # Arguments
    * * byte0 - First byte of the address
    * * byte1 - Second byte of the address
    * * byte2 - Third byte of the address
    * * byte3 - Fourth byte of the address
    *
    * Return the resulting address
    */
    pub fn mew_from_bytes(byte0 : u8, byte1 : u8, byte2 : u8, byte3 : u8) -> IpAddress {
        IpAddress {
            ip : unsafe { ffi::sfIpAddress_fromBytes(byte0, byte1, byte2, byte3) }
        }
    }

    /**
    * Construct an address from a 32-bits integer
    *
    * This function uses the internal representation of
    * the address directly. It should be used for optimization
    * purposes, and only if you got that representation from
    * sfIpAddress_ToInteger.
    * 
    * # Arguments
    * * address - 4 bytes of the address packed into a 32-bits integer
    *
    * Return the resulting address
    */
    pub fn new_from_integer(address : u32) -> IpAddress {
        IpAddress {
            ip : unsafe { ffi::sfIpAddress_fromInteger(address) } 
        }
    }

    /**
    * Get a string representation of an address
    *
    * The returned string is the decimal representation of the
    * IP address (like "192.168.1.56"), even if it was constructed
    * from a host name.
    *
    * Return a string representation of the address
    */
    pub fn to_string(&self) -> ~str {
        unsafe {
            let string : *u8 = ptr::null();
            ffi::sfIpAddress_toString(self.ip, string);
            str::raw::from_c_str(string as *i8)
        }
    }

    /**
    * Get an integer representation of the address
    *
    * The returned number is the internal representation of the
    * address, and should be used for optimization purposes only
    * (like sending the address through a socket).
    * The integer produced by this function can then be converted
    * back to a IpAddress with From_integer.
    *
    * Return a 32-bits unsigned integer representation of the address
    */
    pub fn to_integer(&self) -> u32 {
        unsafe {
            ffi::sfIpAddress_toInteger(self.ip)
        }
    }

    /**
    * Get the computer's local address
    *
    * The local address is the address of the computer from the
    * LAN point of view, i.e. something like 192.168.1.56. It is
    * meaningful only for communications over the local network.
    * Unlike sfIpAddress_getPublicAddress, this function is fast
    * and may be used safely anywhere.
    *
    * Return the local IP address of the computer
    */
    pub fn get_local_address() -> IpAddress {
        IpAddress {
            ip : unsafe { ffi::sfIpAddress_getLocalAddress() }
        }
    }

    /**
    * Get the computer's public address
    * 
    * The public address is the address of the computer from the
    * internet point of view, i.e. something like 89.54.1.169.
    * It is necessary for communications over the world wide web.
    * The only way to get a public address is to ask it to a
    * distant website; as a consequence, this function depends on
    * both your network connection and the server, and may be
    * very slow. You should use it as few as possible. Because
    * this function depends on the network connection and on a distant
    * server, you may use a time limit if you don't want your program
    * to be possibly stuck waiting in case there is a problem; use
    * 0 to deactivate this limit.
    *
    * Return the public IP address of the computer
    */
    pub fn get_public_address(timeout : &Time) -> IpAddress {
        IpAddress {
            ip : unsafe { ffi::sfIpAddress_getPublicAddress(timeout.unwrap()) } 
        }
    }
}

impl Wrappable<ffi::sfIpAddress> for IpAddress {
    fn wrap(ip : ffi::sfIpAddress) -> IpAddress {
        IpAddress {
            ip : ip
        }
    }

    fn unwrap(&self) -> ffi::sfIpAddress {
        self.ip
    }    
}
