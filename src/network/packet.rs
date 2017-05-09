use csfml_system_sys::sfBool;
use network::csfml_network_sys as ffi;
use sf_bool_ext::SfBoolExt;
use std::ffi::CString;

/// Utility type to build blocks of data to transfer over the network.
pub struct Packet {
    packet: *mut ffi::sfPacket,
}

impl Packet {
    /// Create a new packet
    pub fn new() -> Packet {
        let pck = unsafe { ffi::sfPacket_create() };
        assert!(!pck.is_null(), "Failed to create Packet");
        Packet { packet: pck }
    }

    /// Clear a packet
    ///
    /// After calling Clear, the packet is empty.
    pub fn clear(&self) {
        unsafe { ffi::sfPacket_clear(self.packet) }
    }

    /// Get the size of the data contained in a packet
    ///
    /// Return the data size, in bytes
    pub fn data_size(&self) -> u32 {
        unsafe { ffi::sfPacket_getDataSize(self.packet) as u32 }
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
        unsafe { ffi::sfPacket_readInt8(self.packet) }
    }

    /// Function to extract data from a packet
    pub fn read_u8(&self) -> u8 {
        unsafe { ffi::sfPacket_readUint8(self.packet) }
    }

    /// Function to extract data from a packet
    pub fn read_i16(&self) -> i16 {
        unsafe { ffi::sfPacket_readInt16(self.packet) }
    }

    /// Function to extract data from a packet
    pub fn read_u16(&self) -> u16 {
        unsafe { ffi::sfPacket_readUint16(self.packet) }
    }

    /// Function to extract data from a packet
    pub fn read_i32(&self) -> i32 {
        unsafe { ffi::sfPacket_readInt32(self.packet) }
    }

    /// Function to extract data from a packet
    pub fn read_u32(&self) -> u32 {
        unsafe { ffi::sfPacket_readUint32(self.packet) }
    }

    /// Function to extract data from a packet
    pub fn read_f32(&self) -> f32 {
        unsafe { ffi::sfPacket_readFloat(self.packet) as f32 }
    }

    /// Function to extract data from a packet
    pub fn read_f64(&self) -> f64 {
        unsafe { ffi::sfPacket_readDouble(self.packet) as f64 }
    }

    /// Function to extract data from a packet
    pub fn read_string(&self) -> String {
        unsafe {
            let size = ffi::sfPacket_getDataSize(self.packet);
            let mut buffer = vec![0u8; size];
            ffi::sfPacket_readString(self.packet, buffer.as_mut_ptr() as *mut _);
            String::from_utf8(buffer).unwrap()
        }
    }

    /// Function to insert data into a packet
    pub fn write_bool(&self, data: bool) {
        unsafe { ffi::sfPacket_writeBool(self.packet, sfBool::from_bool(data)) }
    }

    /// Function to insert data into a packet
    pub fn write_i8(&self, data: i8) {
        unsafe { ffi::sfPacket_writeInt8(self.packet, data) }
    }

    /// Function to insert data into a packet
    pub fn write_u8(&self, data: u8) {
        unsafe { ffi::sfPacket_writeUint8(self.packet, data) }
    }

    /// Function to insert data into a packet
    pub fn write_i16(&self, data: i16) {
        unsafe { ffi::sfPacket_writeInt16(self.packet, data) }
    }

    /// Function to insert data into a packet
    pub fn write_u16(&self, data: u16) {
        unsafe { ffi::sfPacket_writeUint16(self.packet, data) }
    }

    /// Function to insert data into a packet
    pub fn write_i32(&self, data: i32) {
        unsafe { ffi::sfPacket_writeInt32(self.packet, data) }
    }

    /// Function to insert data into a packet
    pub fn write_u32(&self, data: u32) {
        unsafe { ffi::sfPacket_writeUint32(self.packet, data) }
    }

    /// Function to insert data into a packet
    pub fn write_f32(&self, data: f32) {
        unsafe { ffi::sfPacket_writeFloat(self.packet, data) }
    }

    /// Function to insert data into a packet
    pub fn write_f64(&self, data: f64) {
        unsafe { ffi::sfPacket_writeDouble(self.packet, data) }
    }

    /// Function to insert data into a packet
    pub fn write_string(&self, string: &str) {
        let c_string = CString::new(string.as_bytes()).unwrap();
        unsafe { ffi::sfPacket_writeString(self.packet, c_string.as_ptr()) }
    }
}

impl Default for Packet {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Packet {
    fn clone(&self) -> Self {
        let pck = unsafe { ffi::sfPacket_copy(self.packet) };
        assert!(!pck.is_null(), "Failed to copy Packet");
        Packet { packet: pck }
    }
}

impl Raw for Packet {
    type Raw = *const ffi::sfPacket;
    fn raw(&self) -> Self::Raw {
        self.packet
    }
}

impl RawMut for Packet {
    type RawMut = *mut ffi::sfPacket;
    fn raw_mut(&mut self) -> Self::RawMut {
        self.packet
    }
}

impl FromRaw for Packet {
    type RawFrom = *mut ffi::sfPacket;
    unsafe fn from_raw(raw: Self::RawFrom) -> Self {
        Packet { packet: raw }
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        unsafe { ffi::sfPacket_destroy(self.packet) }
    }
}
