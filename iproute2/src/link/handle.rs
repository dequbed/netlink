use connection::ConnectionHandle;

use super::{LinkAddRequest, LinkDelRequest, LinkGetRequest, LinkSetRequest};

pub struct LinkHandle(ConnectionHandle);

impl LinkHandle {
    pub fn new(handle: ConnectionHandle) -> Self {
        LinkHandle(handle)
    }

    pub fn set(&self, index: u32) -> LinkSetRequest {
        LinkSetRequest::new(self.0.clone(), index)
    }

    pub fn add(&self) -> LinkAddRequest {
        LinkAddRequest::new(self.0.clone())
    }

    pub fn del(&mut self, index: u32) -> LinkDelRequest {
        LinkDelRequest::new(self.0.clone(), index)
    }

    /// Retrieve the list of links (equivalent to `ip link show`)
    pub fn get(&mut self) -> LinkGetRequest {
        LinkGetRequest::new(self.0.clone())
    }
}
