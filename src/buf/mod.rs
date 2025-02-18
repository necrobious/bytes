//! Utilities for working with buffers.
//!
//! A buffer is any structure that contains a sequence of bytes. The bytes may
//! or may not be stored in contiguous memory. This module contains traits used
//! to abstract over buffers as well as utilities for working with buffer types.
//!
//! # `Buf`, `BufMut`
//!
//! These are the two foundational traits for abstractly working with buffers.
//! They can be thought as iterators for byte structures. They offer additional
//! performance over `Iterator` by providing an API optimized for byte slices.
//!
//! See [`Buf`] and [`BufMut`] for more details.
//!
//! [rope]: https://en.wikipedia.org/wiki/Rope_(data_structure)
//! [`Buf`]: trait.Buf.html
//! [`BufMut`]: trait.BufMut.html

mod buf_impl;
mod buf_mut;
mod chain;
mod iter;
mod take;
mod vec_deque;

// When std::io::Reader etc. traits are not available, skip these
#[cfg(feature = "std")]
mod reader;
#[cfg(feature = "std")]
mod writer;

pub use self::buf_impl::Buf;
pub use self::buf_mut::BufMut;
pub use self::chain::Chain;
pub use self::iter::IntoIter;
#[cfg(feature = "std")]
pub use self::reader::Reader;
pub use self::take::Take;
#[cfg(feature = "std")]
pub use self::writer::Writer;
