//! This crate provides traits that match the public API of the Embassy types.
//! These traits are implemented in this crate for the Embassy types, the implementation is a
//! simple wrapper for the public API. This crate also provides mocked versions of these types
//! which also implement the traits provided so they can be used to replace the real types in unit
//! tests.

#![no_std]
#![warn(missing_docs)]

#[cfg(feature = "executor")]
pub mod executor;

#[cfg(feature = "time")]
pub mod time;
