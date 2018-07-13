#![cfg_attr(feature = "nightly", feature(tool_attributes))]
#![cfg_attr(feature = "nightly", feature(custom_attribute))]
#![cfg_attr(feature = "nightly", allow(unused_attributes))]
#![feature(const_fn)]
extern crate byteorder;
extern crate core;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;
extern crate bit_field;

extern crate sc;

use core::ops::{Range, RangeFrom};

/// Represent a multi-bytes field with a fixed size in a packet
pub(crate) type Field = Range<usize>;
/// Represent a field that starts at a given index in a packet
pub(crate) type Rest = RangeFrom<usize>;
/// Represent a field of exactly one byte in a packet
pub(crate) type Index = usize;

mod constants;
pub use self::constants::*;

mod errors;
pub use self::errors::*;

mod packets;
pub use self::packets::*;

pub use sc::platform::nr as syscalls;
