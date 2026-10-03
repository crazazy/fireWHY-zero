// Embedded uses no_std rust.
#![cfg_attr(target_os = "none", no_std)]
// Embedded requires allocator API to allocate Vec in PSRAM.
#![cfg_attr(target_os = "none", feature(allocator_api))]
#![allow(clippy::new_without_default)]

extern crate alloc;

mod errors;
mod shared;

mod device;

pub use device::{Addr, DeviceImpl, DirImpl};
pub use errors::*;
pub use shared::*;
