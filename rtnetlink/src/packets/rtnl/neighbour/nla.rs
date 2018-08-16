use byteorder::{ByteOrder, NativeEndian};
use std::mem::size_of;

use utils::{parse_u16, parse_u32};
use {DefaultNla, NativeNla, Nla, NlaBuffer, Parseable, Result};

use constants::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NeighbourNla {
    Unspec(Vec<u8>),
    Destination(Vec<u8>),
    LinkLocalAddress(Vec<u8>),
    CacheInfo(NeighbourCacheInfo),
    Probes(Vec<u8>),
    VLAN(u16),
    Port(Vec<u8>),
    VNI(u32),
    IfIndex(u32),
    Master(Vec<u8>),
    LinkNetNSID(Vec<u8>),
    SourceVNI(u32),
    Other(DefaultNla),
}

impl Nla for NeighbourNla {
    #[cfg_attr(nightly, rustfmt::skip)]
    fn value_len(&self) -> usize {
        use self::NeighbourNla::*;
        match *self {
            Unspec(ref bytes)
            | Destination(ref bytes)
            | LinkLocalAddress(ref bytes)
            | Probes(ref bytes)
            | Port(ref bytes)
            | Master(ref bytes)
            | LinkNetNSID(ref bytes) => bytes.len(),

            CacheInfo(_) => size_of::<NeighbourCacheInfo>(),

            VLAN(_) => size_of::<u16>(),

            VNI(_)
            | IfIndex(_)
            | SourceVNI(_) => size_of::<u32>(),

            Other(ref attr) => attr.value_len(),
        }
    }

    #[cfg_attr(nightly, rustfmt::skip)]
    fn emit_value(&self, buffer: &mut [u8]) {
        use self::NeighbourNla::*;
        match *self {
            Unspec(ref bytes)
            | Destination(ref bytes)
            | LinkLocalAddress(ref bytes)
            | Probes(ref bytes)
            | Port(ref bytes)
            | Master(ref bytes)
            | LinkNetNSID(ref bytes) => buffer.copy_from_slice(bytes.as_slice()),

            CacheInfo(ref cacheinfo) => cacheinfo.to_bytes(buffer),

            VLAN(ref value) => NativeEndian::write_u16(buffer, *value),

            VNI(ref value)
            | IfIndex(ref value)
            | SourceVNI(ref value) => NativeEndian::write_u32(buffer, *value),

            Other(ref attr) => attr.emit_value(buffer),
        }
    }

    fn kind(&self) -> u16 {
        use self::NeighbourNla::*;
        match *self {
            Unspec(_) => NDA_UNSPEC,
            Destination(_) => NDA_DST,
            LinkLocalAddress(_) => NDA_LLADDR,
            CacheInfo(_) => NDA_CACHEINFO,
            Probes(_) => NDA_PROBES,
            VLAN(_) => NDA_VLAN,
            Port(_) => NDA_PORT,
            VNI(_) => NDA_VNI,
            IfIndex(_) => NDA_IFINDEX,
            Master(_) => NDA_MASTER,
            LinkNetNSID(_) => NDA_LINK_NETNSID,
            SourceVNI(_) => NDA_SRC_VNI,
            Other(ref nla) => nla.kind(),
        }
    }
}

impl<'buffer, T: AsRef<[u8]> + ?Sized> Parseable<NeighbourNla> for NlaBuffer<&'buffer T> {
    fn parse(&self) -> Result<NeighbourNla> {
        use self::NeighbourNla::*;
        let payload = self.value();
        Ok(match self.kind() {
            NDA_UNSPEC => Unspec(payload.to_vec()),
            NDA_DST => Destination(payload.to_vec()),
            NDA_LLADDR => LinkLocalAddress(payload.to_vec()),
            NDA_CACHEINFO => CacheInfo(NeighbourCacheInfo::from_bytes(payload)?),
            NDA_PROBES => Probes(payload.to_vec()),
            NDA_VLAN => VLAN(parse_u16(payload)?),
            NDA_PORT => Port(payload.to_vec()),
            NDA_VNI => VNI(parse_u32(payload)?),
            NDA_IFINDEX => IfIndex(parse_u32(payload)?),
            NDA_MASTER => Master(payload.to_vec()),
            NDA_LINK_NETNSID => LinkNetNSID(payload.to_vec()),
            NDA_SRC_VNI => SourceVNI(parse_u32(payload)?),
            _ => Other(<Self as Parseable<DefaultNla>>::parse(self)?),
        })
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct NeighbourCacheInfo {
    pub ndm_confirmed: u32,
    pub ndm_used: u32,
    pub ndm_updated: u32,
    pub ndm_refcnt: u32,
}

impl NativeNla for NeighbourCacheInfo { }
