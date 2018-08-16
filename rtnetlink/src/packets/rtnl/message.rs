use super::*;

use {AckMessage, ErrorMessage};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RtnlMessage {
    Done,
    Error(ErrorMessage),
    Ack(AckMessage),
    Noop,
    Overrun(Vec<u8>),
    NewLink(LinkMessage),
    DelLink(LinkMessage),
    GetLink(LinkMessage),
    SetLink(LinkMessage),
    NewAddress(AddressMessage),
    DelAddress(AddressMessage),
    GetAddress(AddressMessage),
    GetNeighbour(NeighbourMessage),
    NewNeighbour(NeighbourMessage),
    //SetNeighbourTable(NeighbourTableMessage),
    Other(Vec<u8>),
}

impl RtnlMessage {
    pub fn is_done(&self) -> bool {
        *self == RtnlMessage::Done
    }

    pub fn is_noop(&self) -> bool {
        *self == RtnlMessage::Noop
    }

    pub fn is_overrun(&self) -> bool {
        if let RtnlMessage::Overrun(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_error(&self) -> bool {
        if let RtnlMessage::Error(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_ack(&self) -> bool {
        if let RtnlMessage::Ack(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_new_link(&self) -> bool {
        if let RtnlMessage::NewLink(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_del_link(&self) -> bool {
        if let RtnlMessage::DelLink(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_get_link(&self) -> bool {
        if let RtnlMessage::GetLink(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_set_link(&self) -> bool {
        if let RtnlMessage::SetLink(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_new_address(&self) -> bool {
        if let RtnlMessage::NewAddress(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_del_address(&self) -> bool {
        if let RtnlMessage::DelAddress(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_get_address(&self) -> bool {
        if let RtnlMessage::GetAddress(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_get_neighbour(&self) -> bool {
        if let RtnlMessage::GetNeighbour(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_new_neighbour(&self) -> bool {
        if let RtnlMessage::NewNeighbour(_) = *self {
            true
        } else {
            false
        }
    }

    /*
     *pub fn is_set_neighbourtable(&self) -> bool {
     *    if let RtnlMessage::SetNeighbourTable(_) = *self {
     *        true
     *    } else {
     *        false
     *    }
     *}
     */
}
