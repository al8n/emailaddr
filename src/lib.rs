#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(missing_docs)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc as std;

#[cfg(feature = "std")]
extern crate std;

pub use addr::*;
pub use buffer::*;
pub use domain::*;
pub use local::*;

mod addr;
#[cfg(all(feature = "arbitrary", any(feature = "alloc", feature = "std")))]
mod arbitrary_impl;
mod buffer;
mod domain;
mod local;
#[cfg(all(feature = "quickcheck", any(feature = "alloc", feature = "std")))]
mod quickcheck_impl;
#[cfg(feature = "serde")]
pub use serde_impl::EmailAddrSerdeStorage;
#[cfg(feature = "serde")]
mod serde_impl;
