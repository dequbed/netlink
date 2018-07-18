//! This package contains types that represent netlink messages. See the [`libnl` library
//! documentation][libnl] for an introduction to the Netlink protocols.
//!
//! Currently, only a subset of the `NETLINK_ROUTE` family has been implemented. It is available in
//! [`packet::rtnl`]. On the long term, I intend to support a few additional netlink families
//! (see `man 7 netlink` for a complete list of protocol families).
//!
//! [`packet::rtnl`]: rtnl/index.html
//! [libnl]: https://www.infradead.org/~tgr/libnl/doc/core.html#core_netlink_fundamentals

use core::ops::{Range, RangeFrom};

/// Represent a multi-bytes field with a fixed size in a packet
pub(crate) type Field = Range<usize>;
/// Represent a field that starts at a given index in a packet
pub(crate) type Rest = RangeFrom<usize>;
/// Represent a field of exactly one byte in a packet
pub(crate) type Index = usize;

#[cfg(feature = "rtnetlink")]
/// rtnetlink types (see `man 7 rtnetlink`)
mod rtnl;
#[cfg(feature = "rtnetlink")]
pub use self::rtnl::*;

#[cfg(feature = "audit")]
mod audit;
#[cfg(feature = "audit")]
pub use self::audit::*;

mod netlink;
pub use self::netlink::*;

// FIXME: should we expose these traits or only keep them for internal use?
mod traits;
pub use self::traits::*;
