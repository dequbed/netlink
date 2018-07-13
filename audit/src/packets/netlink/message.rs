use {
    AckMessage, Emitable, Error, ErrorBuffer, ErrorMessage, NetlinkBuffer, NetlinkHeader,
    Parseable, Result, NLMSG_DONE, NLMSG_ERROR, NLMSG_NOOP, NLMSG_OVERRUN,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AuditMessage {
    Done,
    Error(ErrorMessage),
    Ack(AckMessage),
    Noop,
    Overrun(Vec<u8>),
    Other(Vec<u8>),
}

impl AuditMessage {
    pub fn is_done(&self) -> bool {
        *self == AuditMessage::Done
    }

    pub fn is_noop(&self) -> bool {
        *self == AuditMessage::Noop
    }

    pub fn is_overrun(&self) -> bool {
        if let AuditMessage::Overrun(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_error(&self) -> bool {
        if let AuditMessage::Error(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_ack(&self) -> bool {
        if let AuditMessage::Ack(_) = *self {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NetlinkMessage {
    header: NetlinkHeader,
    message: AuditMessage,
    finalized: bool,
}

impl From<AuditMessage> for NetlinkMessage {
    fn from(message: AuditMessage) -> Self {
        NetlinkMessage {
            header: NetlinkHeader::default(),
            message,
            finalized: false,
        }
    }
}

impl NetlinkMessage {
    pub fn into_parts(self) -> (NetlinkHeader, AuditMessage) {
        (self.header, self.message)
    }

    pub fn message(&self) -> &AuditMessage {
        &self.message
    }

    pub fn message_mut(&mut self) -> &mut AuditMessage {
        &mut self.message
    }

    pub fn header(&self) -> &NetlinkHeader {
        &self.header
    }

    pub fn header_mut(&mut self) -> &mut NetlinkHeader {
        &mut self.header
    }

    /// Safely serialize the message. Under the hood, this calls
    /// [`Emitable::emit()`](trait.Emitable.html#tymethod.emit), but unlike `emit()`, this method
    /// does not panic if the message is malformed or if the destination buffer is too small.
    /// Instead, an error is returned. Note that you must call [`finalize()`](#method.finalize)
    /// before calling this method, otherwise, `Error::Malformed` is returned.
    pub fn to_bytes(&self, buffer: &mut [u8]) -> Result<usize> {
        if !self.finalized {
            Err(Error::Malformed)
        } else if self.header().length() as usize > buffer.len() {
            Err(Error::Exhausted)
        } else {
            self.emit(buffer);
            Ok(self.header().length() as usize)
        }
    }

    /// Try to parse a message from a buffer
    pub fn from_bytes(buffer: &[u8]) -> Result<Self> {
        NetlinkBuffer::new_checked(&buffer)?.parse()
    }

    /// Check if the payload is a `NLMSG_DONE` message
    /// ([`AuditMessage::Done`](enum.AuditMessage.html#variant.Done))
    pub fn is_done(&self) -> bool {
        self.message().is_done()
    }

    /// Check if the payload is a `NLMSG_NOOP` message
    /// ([`AuditMessage::Noop`](enum.AuditMessage.html#variant.Noop))
    pub fn is_noop(&self) -> bool {
        self.message().is_noop()
    }

    /// Check if the payload is a `NLMSG_OVERRUN` message
    /// ([`AuditMessage::Overrun`](enum.AuditMessage.html#variant.Overrun))
    pub fn is_overrun(&self) -> bool {
        self.message().is_overrun()
    }

    /// Check if the payload is a `NLMSG_ERROR` message with a negative error code
    /// ([`AuditMessage::Error`](enum.AuditMessage.html#variant.Error))
    pub fn is_error(&self) -> bool {
        self.message().is_error()
    }

    /// Check if the payload is a `NLMSG_ERROR` message with a non-negative error code
    /// ([`AuditMessage::Ack`](enum.AuditMessage.html#variant.Ack))
    pub fn is_ack(&self) -> bool {
        self.message().is_ack()
    }

    /// Ensure the header (`NetlinkHeader`) is consistent with the payload (`AuditMessage`):
    ///
    /// - compute the payload length and set the header's length field
    /// - check the payload type and set the header's message type field accordingly
    ///
    /// If you are not 100% sure the header is correct, this method should be called before calling
    /// [`Emitable::emit()`](trait.Emitable.html#tymethod.emit) or
    /// [`to_bytes()`](#method.to_bytes). `emit()` could panic if the header is inconsistent with
    /// the rest of the message, and `to_bytes()` would return an error.
    pub fn finalize(&mut self) {
        use self::AuditMessage::*;
        *self.header.length_mut() = self.buffer_len() as u32;
        *self.header.message_type_mut() = match self.message {
            Noop => NLMSG_NOOP,
            Done => NLMSG_DONE,
            Error(_) | Ack(_) => NLMSG_ERROR,
            Overrun(_) => NLMSG_OVERRUN,
            Other(_) => unimplemented!(),
        };
        self.finalized = true;
    }
}

impl<'buffer, T: AsRef<[u8]> + 'buffer> Parseable<NetlinkMessage> for NetlinkBuffer<&'buffer T> {
    fn parse(&self) -> Result<NetlinkMessage> {
        use self::AuditMessage::*;
        let header = <Self as Parseable<NetlinkHeader>>::parse(self)?;

        let message = match header.message_type() {
            NLMSG_ERROR => {
                let msg: ErrorMessage = ErrorBuffer::new(&self.payload()).parse()?;
                if msg.code >= 0 {
                    Ack(msg as AckMessage)
                } else {
                    Error(msg)
                }
            }
            NLMSG_NOOP => Noop,
            NLMSG_DONE => Done,
            _ => Other(self.payload().to_vec()),
        };
        Ok(NetlinkMessage {
            header,
            message,
            finalized: true,
        })
    }
}

impl Emitable for NetlinkMessage {
    #[cfg_attr(nightly, rustfmt::skip)]
    fn buffer_len(&self) -> usize {
        use self::AuditMessage::*;
        let payload_len = match self.message {
            Noop | Done => 0,

            | Overrun(ref bytes)
            | Other(ref bytes)
            => bytes.len(),

            Error(ref msg) => msg.buffer_len(),
            Ack(ref msg) => msg.buffer_len(),
        };
        self.header.buffer_len() + payload_len
    }

    #[cfg_attr(nightly, rustfmt::skip)]
    fn emit(&self, buffer: &mut [u8]) {
        use self::AuditMessage::*;
        self.header.emit(buffer);
        let buffer = &mut buffer[self.header.buffer_len()..];
        match self.message {
            Noop | Done => {},

            Overrun(ref bytes)
            | Other(ref bytes)
            => buffer.copy_from_slice(bytes),

            Error(ref msg) => msg.emit(buffer),
            Ack(ref msg) => msg.emit(buffer),
        }
    }
}
