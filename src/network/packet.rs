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

//! Utility class to build blocks of data to transfer over the network.

use std::ptr;
use std::ffi::{CString, CStr};
use std::str;

use traits::Wrappable;

use sfml_types::sfBool;
use csfml_network_sys as ffi;

/// Utility class to build blocks of data to transfer over the network.
pub struct Packet {
    packet: *mut ffi::sfPacket
}

impl Packet {
    /// Create a new packet
    ///
    /// Return Some(Packet) or None
    pub fn new() -> Option<Packet> {
        let pck = unsafe { ffi::sfPacket_create() };
        if pck.is_null() {
            None
        }
        else {
            Some(Packet {
                packet: pck
            })
        }
    }

    /// Create a new packet by copying an existing one
    ///
    /// Return Some(Packet) or None
    pub fn clone(&self) -> Option<Packet> {
        let pck = unsafe { ffi::sfPacket_copy(self.packet) };
        if pck.is_null() {
            None
        }
        else{
            Some(Packet {
                packet: pck
            })
        }
    }

    /// Clear a packet
    ///
    /// After calling Clear, the packet is empty.
    pub fn clear(&self) {
        unsafe {
            ffi::sfPacket_clear(self.packet)
        }
    }

    /// Get the size of the data contained in a packet
    ///
    /// This function returns the number of bytes pointed to by
    /// what sfPacket_getData returns.
    ///
    /// Return the data size, in bytes
    pub fn get_data_size(&self) -> u32 {
        unsafe {
            ffi::sfPacket_getDataSize(self.packet) as u32
        }
    }

    /// Tell if the reading position has reached the
    /// end of a packet
    ///
    /// This function is useful to know if there is some data
    /// left to be read, without actually reading it.
    ///
    /// Return true if all data was read, false otherwise
    pub fn end_of_packet(&self) -> bool {
        unsafe { ffi::sfPacket_endOfPacket(self.packet) }.to_bool()
    }

    /// Test the validity of a packet, for reading
    ///
    /// This function allows to test the packet, to check if
    /// a reading operation was successful.
    ///
    /// A packet will be in an invalid state if it has no more
    /// data to read.
    ///
    /// Return true if last data extraction from packet was successful
    pub fn can_read(&self) -> bool {
        unsafe { ffi::sfPacket_canRead(self.packet) }.to_bool()
    }

    /// Function to extract data from a packet
    pub fn read_bool(&self) -> bool {
        unsafe { ffi::sfPacket_readBool(self.packet) }.to_bool()
    }

    /// Function to extract data from a packet
    pub fn read_i8(&self) -> i8 {
        unsafe {
            ffi::sfPacket_readInt8(self.packet)
        }
    }

    /// Function to extract data from a packet
    pub fn read_u8(&self) -> u8 {
        unsafe {
            ffi::sfPacket_readUint8(self.packet)
        }
    }

    /// Function to extract data from a packet
    pub fn read_i16(&self) -> i16 {
        unsafe {
            ffi::sfPacket_readInt16(self.packet)
        }
    }

    /// Function to extract data from a packet
    pub fn read_u16(&self) -> u16 {
        unsafe {
            ffi::sfPacket_readUint16(self.packet)
        }
    }

    /// Function to extract data from a packet
    pub fn read_i32(&self) -> i32 {
        unsafe {
            ffi::sfPacket_readInt32(self.packet)
        }
    }

    /// Function to extract data from a packet
    pub fn read_u32(&self) -> u32 {
        unsafe {
            ffi::sfPacket_readUint32(self.packet)
        }
    }

    /// Function to extract data from a packet
    pub fn read_f32(&self) -> f32 {
        unsafe {
            ffi::sfPacket_readFloat(self.packet) as f32
        }
    }

    /// Function to extract data from a packet
    pub fn read_f64(&self) -> f64 {
        unsafe {
            ffi::sfPacket_readDouble(self.packet) as f64
        }
    }

    /// Function to extract data from a packet
    pub fn read_string(&self) -> String {
        unsafe {
            let string: *mut u8 = ptr::null_mut();
            ffi::sfPacket_readString(self.packet, string);
            let string = string as *const i8;
            str::from_utf8(CStr::from_ptr(string).to_bytes_with_nul()).unwrap().into()
        }
}

    /// Function to insert data into a packet
    pub fn write_bool(&self, data: bool) {
        unsafe {
            ffi::sfPacket_writeBool(self.packet, sfBool::from_bool(data))
        }
    }

    /// Function to insert data into a packet
    pub fn write_i8(&self, data: i8) {
        unsafe {
            ffi::sfPacket_writeInt8(self.packet, data)
        }
    }

    /// Function to insert data into a packet
    pub fn write_u8(&self, data: u8) {
        unsafe {
            ffi::sfPacket_writeUint8(self.packet, data)
        }
    }

    /// Function to insert data into a packet
    pub fn write_i16(&self, data: i16) {
        unsafe {
            ffi::sfPacket_writeInt16(self.packet, data)
        }
    }

    /// Function to insert data into a packet
    pub fn write_u16(&self, data: u16) {
        unsafe {
            ffi::sfPacket_writeUint16(self.packet, data)
        }
    }

    /// Function to insert data into a packet
    pub fn write_i32(&self, data: i32) {
        unsafe {
            ffi::sfPacket_writeInt32(self.packet, data)
        }
    }

    /// Function to insert data into a packet
    pub fn write_u32(&self, data: u32) {
        unsafe {
            ffi::sfPacket_writeUint32(self.packet, data)
        }
    }

    /// Function to insert data into a packet
    pub fn write_f32(&self, data: f32) {
        unsafe {
            ffi::sfPacket_writeFloat(self.packet, data)
        }
    }

    /// Function to insert data into a packet
    pub fn write_f64(&self, data: f64) {
        unsafe {
            ffi::sfPacket_writeDouble(self.packet, data)
        }
    }

    /// Function to insert data into a packet
    pub fn write_string(&self, string: &str) {
        let c_string = CString::new(string.as_bytes()).unwrap().as_ptr();
        unsafe {
            ffi::sfPacket_writeString(self.packet, c_string)
        }
    }
}

impl Wrappable<*mut ffi::sfPacket> for Packet {
    fn unwrap(&self) -> *mut ffi::sfPacket {
        self.packet
    }

    fn wrap(packet: *mut ffi::sfPacket) -> Packet {
        Packet {
            packet: packet
        }
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        unsafe {
            ffi::sfPacket_destroy(self.packet)
        }
    }
}
