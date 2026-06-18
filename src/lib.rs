#![no_std]
#![cfg_attr(all(doc, docsrs), feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod color;

#[cfg(feature = "colorize")]
mod colorize;

#[cfg(feature = "split")]
pub mod bytes;
#[cfg(feature = "split")]
pub mod str;

pub use crate::color::Color;

#[cfg(feature = "colorize")]
pub use crate::colorize::{Colored, Colorize};
