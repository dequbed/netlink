use super::{NetlinkBuffer, NetlinkFlags, NETLINK_HEADER_LEN};
use {Emitable, Parseable, Result};

/// A Netlink header representation.
///
/// A netlink header has the following structure:
///
/// ```no_rust
/// 0                8                16              24               32
/// +----------------+----------------+----------------+----------------+
/// |                 packet length (including header)                  |
/// +----------------+----------------+----------------+----------------+
/// |          message type           |              flags              |
/// +----------------+----------------+----------------+----------------+
/// |                           sequence number                         |
/// +----------------+----------------+----------------+----------------+
/// |                   port number (formerly known as PID)             |
/// +----------------+----------------+----------------+----------------+
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct NetlinkHeader {
    /// Length of the netlink packet, including the header and the payload
    length: u32,

    /// NetlinkMessage type. The meaning of this field depends on the netlink protocol family in use.
    message_type: u16,

    /// Flags
    flags: NetlinkFlags,

    /// Sequence number of the packet
    sequence_number: u32,

    /// Port number (usually set to the the process ID)
    port_number: u32,
}

impl NetlinkHeader {
    /// Create a new header, initialized with the given values
    pub fn new(
        length: u32,
        message_type: u16,
        flags: NetlinkFlags,
        sequence_number: u32,
        port_number: u32,
    ) -> Self {
        NetlinkHeader {
            length,
            message_type,
            flags,
            sequence_number,
            port_number,
        }
    }

    /// Get the length field
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Get a mutable reference to the length field
    pub fn length_mut(&mut self) -> &mut u32 {
        &mut self.length
    }

    /// Setter for the length field
    pub fn set_length(&mut self, value: u32) -> &mut Self {
        self.length = value;
        self
    }

    /// Get the message type field
    pub fn message_type(&self) -> u16 {
        self.message_type
    }

    /// Get a mutable reference to the message type field
    pub fn message_type_mut(&mut self) -> &mut u16 {
        &mut self.message_type
    }

    /// Setter for the message_type field
    pub fn set_message_type(&mut self, value: u16) -> &mut Self {
        self.message_type = value;
        self
    }

    /// Get the flags field
    pub fn flags(&self) -> NetlinkFlags {
        self.flags
    }

    /// Get a mutable reference to the flags field
    pub fn flags_mut(&mut self) -> &mut NetlinkFlags {
        &mut self.flags
    }

    /// Setter for the flags field
    pub fn set_flags(&mut self, value: NetlinkFlags) -> &mut Self {
        self.flags = value;
        self
    }

    /// Get the sequence number field
    pub fn sequence_number(&self) -> u32 {
        self.sequence_number
    }

    /// Get a mutable reference to the sequence number field
    pub fn sequence_number_mut(&mut self) -> &mut u32 {
        &mut self.sequence_number
    }

    /// Setter for the sequence number field
    pub fn set_sequence_number(&mut self, value: u32) -> &mut Self {
        self.sequence_number = value;
        self
    }

    /// Get the port number field
    pub fn port_number(&self) -> u32 {
        self.port_number
    }

    /// Get a mutable reference to the port number field
    pub fn port_number_mut(&mut self) -> &mut u32 {
        &mut self.port_number
    }

    /// Setter for the port number field
    pub fn set_port_number(&mut self, value: u32) -> &mut Self {
        self.port_number = value;
        self
    }
}

impl Emitable for NetlinkHeader {
    fn buffer_len(&self) -> usize {
        NETLINK_HEADER_LEN
    }

    fn emit(&self, buffer: &mut [u8]) {
        let mut buffer = NetlinkBuffer::new(buffer);
        buffer.set_message_type(self.message_type);
        buffer.set_length(self.length);
        buffer.set_flags(self.flags);
        buffer.set_sequence_number(self.sequence_number);
        buffer.set_port_number(self.port_number);
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NetlinkHeader> for NetlinkBuffer<&'a T> {
    fn parse(&self) -> Result<NetlinkHeader> {
        Ok(NetlinkHeader {
            length: self.length(),
            message_type: self.message_type(),
            flags: self.flags(),
            sequence_number: self.sequence_number(),
            port_number: self.port_number(),
        })
    }
}
