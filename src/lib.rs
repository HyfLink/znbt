//! *ZNBT* is a more memory efficient minecraft NBT library.
//!
//! TODO: crate-level documentation

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod kind;
