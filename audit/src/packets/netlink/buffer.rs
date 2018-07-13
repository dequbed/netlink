use byteorder::{ByteOrder, NativeEndian};

use {Error, Field, NetlinkFlags, Rest, Result};

const LENGTH: Field = 0..4;
const MESSAGE_TYPE: Field = 4..6;
const FLAGS: Field = 6..8;
const SEQUENCE_NUMBER: Field = 8..12;
const PORT_NUMBER: Field = 12..16;
const PAYLOAD: Rest = 16..;

/// Length of a Netlink packet header
pub const NETLINK_HEADER_LEN: usize = PAYLOAD.start;

#[derive(Debug, PartialEq, Eq, Clone)]
/// A raw Netlink buffer that provides getters and setter for the various header fields, and to
/// retrieve the payloads.
pub struct NetlinkBuffer<T> {
    buffer: T,
}

impl<T: AsRef<[u8]>> NetlinkBuffer<T> {
    /// Create a new `NetlinkBuffer` that uses the given buffer as storage. Note that when calling
    /// this method no check is performed, so trying to access fields may panic. If you're not sure
    /// the given buffer contains a valid netlink packet, use
    /// [`new_checked()`](struct.NetlinkBuffer.html#method.new_checked) instead.
    ///
    pub fn new(buffer: T) -> NetlinkBuffer<T> {
        NetlinkBuffer { buffer }
    }

    /// Check the length of the given buffer and make sure it's big enough so that trying to access
    /// packet fields won't panic. If the buffer is big enough, create a new `NewlinkBuffer` that
    /// uses this buffer as storage.
    pub fn new_checked(buffer: T) -> Result<NetlinkBuffer<T>> {
        let packet = Self::new(buffer);
        packet.check_buffer_length()?;
        Ok(packet)
    }

    fn check_buffer_length(&self) -> Result<()> {
        let len = self.buffer.as_ref().len();
        if len < PORT_NUMBER.end || len < self.length() as usize {
            Err(Error::Truncated)
        } else {
            Ok(())
        }
    }

    /// Return the payload length.
    ///
    /// # Panic
    ///
    /// This panic is the underlying storage is too small or if the `length` field in the header is
    /// set to a value that exceeds the storage length (see
    /// [`new_checked()`](struct.NetlinkBuffer.html#method.new_checked))
    pub fn payload_length(&self) -> usize {
        let total_length = self.length() as usize;
        let payload_offset = PAYLOAD.start;
        // This may panic!
        total_length - payload_offset
    }

    /// Consume the packet, returning the underlying buffer.
    pub fn into_inner(self) -> T {
        self.buffer
    }

    /// Return the `length` field
    ///
    /// # Panic
    ///
    /// This panic is the underlying storage is too small (see [`new_checked()`](struct.NetlinkBuffer.html#method.new_checked))
    pub fn length(&self) -> u32 {
        let data = self.buffer.as_ref();
        NativeEndian::read_u32(&data[LENGTH])
    }

    /// Return the `type` field
    ///
    /// # Panic
    ///
    /// This panic is the underlying storage is too small (see [`new_checked()`](struct.NetlinkBuffer.html#method.new_checked))
    pub fn message_type(&self) -> u16 {
        let data = self.buffer.as_ref();
        NativeEndian::read_u16(&data[MESSAGE_TYPE])
    }

    /// Return the `flags` field
    ///
    /// # Panic
    ///
    /// This panic is the underlying storage is too small (see [`new_checked()`](struct.NetlinkBuffer.html#method.new_checked))
    pub fn flags(&self) -> NetlinkFlags {
        let data = self.buffer.as_ref();
        NetlinkFlags::from(NativeEndian::read_u16(&data[FLAGS]))
    }

    /// Return the `sequence_number` field
    ///
    /// # Panic
    ///
    /// This panic is the underlying storage is too small (see [`new_checked()`](struct.NetlinkBuffer.html#method.new_checked))
    pub fn sequence_number(&self) -> u32 {
        let data = self.buffer.as_ref();
        NativeEndian::read_u32(&data[SEQUENCE_NUMBER])
    }

    /// Return the `port_number` field
    ///
    /// # Panic
    ///
    /// This panic is the underlying storage is too small (see [`new_checked()`](struct.NetlinkBuffer.html#method.new_checked))
    pub fn port_number(&self) -> u32 {
        let data = self.buffer.as_ref();
        NativeEndian::read_u32(&data[PORT_NUMBER])
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> NetlinkBuffer<T> {
    /// Set the packet header `length` field
    ///
    /// # Panic
    ///
    /// This panic is the underlying storage is too small (see [`new_checked()`](struct.NetlinkBuffer.html#method.new_checked))
    pub fn set_length(&mut self, value: u32) {
        let data = self.buffer.as_mut();
        NativeEndian::write_u32(&mut data[LENGTH], value)
    }

    /// Set the packet header `message_type` field
    ///
    /// # Panic
    ///
    /// This panic is the underlying storage is too small (see [`new_checked()`](struct.NetlinkBuffer.html#method.new_checked))
    pub fn set_message_type(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NativeEndian::write_u16(&mut data[MESSAGE_TYPE], value)
    }

    /// Set the packet header `flags` field
    ///
    /// # Panic
    ///
    /// This panic is the underlying storage is too small (see [`new_checked()`](struct.NetlinkBuffer.html#method.new_checked))
    pub fn set_flags(&mut self, value: NetlinkFlags) {
        let data = self.buffer.as_mut();
        NativeEndian::write_u16(&mut data[FLAGS], value.into())
    }

    /// Set the packet header `sequence_number` field
    ///
    /// # Panic
    ///
    /// This panic is the underlying storage is too small (see [`new_checked()`](struct.NetlinkBuffer.html#method.new_checked))
    pub fn set_sequence_number(&mut self, value: u32) {
        let data = self.buffer.as_mut();
        NativeEndian::write_u32(&mut data[SEQUENCE_NUMBER], value)
    }

    /// Set the packet header `port_number` field
    ///
    /// # Panic
    ///
    /// This panic is the underlying storage is too small (see [`new_checked()`](struct.NetlinkBuffer.html#method.new_checked))
    pub fn set_port_number(&mut self, value: u32) {
        let data = self.buffer.as_mut();
        NativeEndian::write_u32(&mut data[PORT_NUMBER], value)
    }

    /// Return a mutable pointer to the payload.
    ///
    /// # Panic
    ///
    /// This panic is the underlying storage is too small or if the `length` field in the header is
    /// set to a value that exceeds the storage length (see
    /// [`new_checked()`](struct.NetlinkBuffer.html#method.new_checked))
    pub fn payload_mut(&mut self) -> &mut [u8] {
        let range = PAYLOAD.start..self.length() as usize;
        let data = self.buffer.as_mut();
        &mut data[range]
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> NetlinkBuffer<&'a T> {
    /// Return a pointer to the packet payload.
    ///
    /// # Panic
    ///
    /// This panic is the underlying storage is too small or if the `length` field in the header is
    /// set to a value that exceeds the storage length (see
    /// [`new_checked()`](struct.NetlinkBuffer.html#method.new_checked))
    pub fn payload(&self) -> &'a [u8] {
        let range = PAYLOAD.start..self.length() as usize;
        let data = self.buffer.as_ref();
        &data[range]
    }
}
