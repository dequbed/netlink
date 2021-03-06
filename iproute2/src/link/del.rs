use futures::Future;

use rtnetlink::constants::{NLM_F_ACK, NLM_F_CREATE, NLM_F_EXCL, NLM_F_REQUEST};
use rtnetlink::{LinkMessage, NetlinkFlags, NetlinkMessage, RtnlMessage};

use connection::ConnectionHandle;
use errors::NetlinkIpError;

use Stream2Ack;

lazy_static! {
    // Flags for `ip link del`
    static ref DEL_FLAGS: NetlinkFlags =
        NetlinkFlags::from(NLM_F_REQUEST | NLM_F_ACK | NLM_F_EXCL | NLM_F_CREATE);
}

pub struct LinkDelRequest {
    handle: ConnectionHandle,
    message: LinkMessage,
}

impl LinkDelRequest {
    pub(crate) fn new(handle: ConnectionHandle, index: u32) -> Self {
        let mut message = LinkMessage::new();
        message.header_mut().set_index(index);
        LinkDelRequest { handle, message }
    }

    /// Execute the request
    pub fn execute(self) -> impl Future<Item = (), Error = NetlinkIpError> {
        let LinkDelRequest {
            mut handle,
            message,
        } = self;
        let mut req = NetlinkMessage::from(RtnlMessage::DelLink(message));
        req.header_mut().set_flags(*DEL_FLAGS);
        Stream2Ack::new(handle.request(req))
    }

    /// Return a mutable reference to the request
    pub fn message_mut(&mut self) -> &mut LinkMessage {
        &mut self.message
    }
}
