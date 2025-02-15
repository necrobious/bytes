use crate::{Buf, BufMut};
use crate::buf::IntoIter;

#[cfg(feature = "std")]
use std::io::{IoSlice, IoSliceMut};

/// A `Chain` sequences two buffers.
///
/// `Chain` is an adapter that links two underlying buffers and provides a
/// continuous view across both buffers. It is able to sequence either immutable
/// buffers ([`Buf`] values) or mutable buffers ([`BufMut`] values).
///
/// This struct is generally created by calling [`Buf::chain`]. Please see that
/// function's documentation for more detail.
///
/// # Examples
///
/// ```
/// use bytes::{Bytes, Buf};
/// use bytes::buf::Chain;
///
/// let mut buf = Bytes::from(&b"hello "[..])
///             .chain(Bytes::from(&b"world"[..]));
///
/// let full: Bytes = buf.to_bytes();
/// assert_eq!(full[..], b"hello world"[..]);
/// ```
///
/// [`Buf::chain`]: trait.Buf.html#method.chain
/// [`Buf`]: trait.Buf.html
/// [`BufMut`]: trait.BufMut.html
#[derive(Debug)]
pub struct Chain<T, U> {
    a: T,
    b: U,
}

impl<T, U> Chain<T, U> {
    /// Creates a new `Chain` sequencing the provided values.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BytesMut;
    /// use bytes::buf::Chain;
    ///
    /// let buf = Chain::new(
    ///     BytesMut::with_capacity(1024),
    ///     BytesMut::with_capacity(1024));
    ///
    /// // Use the chained buffer
    /// ```
    pub fn new(a: T, b: U) -> Chain<T, U> {
        Chain {
            a,
            b,
        }
    }

    /// Gets a reference to the first underlying `Buf`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::{Bytes, Buf};
    ///
    /// let buf = Bytes::from(&b"hello"[..])
    ///             .chain(Bytes::from(&b"world"[..]));
    ///
    /// assert_eq!(buf.first_ref()[..], b"hello"[..]);
    /// ```
    pub fn first_ref(&self) -> &T {
        &self.a
    }

    /// Gets a mutable reference to the first underlying `Buf`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::{Bytes, Buf};
    ///
    /// let mut buf = Bytes::from(&b"hello "[..])
    ///                 .chain(Bytes::from(&b"world"[..]));
    ///
    /// buf.first_mut().advance(1);
    ///
    /// let full: Bytes = buf.to_bytes();
    /// assert_eq!(full[..], b"ello world"[..]);
    /// ```
    pub fn first_mut(&mut self) -> &mut T {
        &mut self.a
    }

    /// Gets a reference to the last underlying `Buf`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::{Bytes, Buf};
    ///
    /// let buf = Bytes::from(&b"hello"[..])
    ///             .chain(Bytes::from(&b"world"[..]));
    ///
    /// assert_eq!(buf.last_ref()[..], b"world"[..]);
    /// ```
    pub fn last_ref(&self) -> &U {
        &self.b
    }

    /// Gets a mutable reference to the last underlying `Buf`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::{Bytes, Buf};
    ///
    /// let mut buf = Bytes::from(&b"hello "[..])
    ///                 .chain(Bytes::from(&b"world"[..]));
    ///
    /// buf.last_mut().advance(1);
    ///
    /// let full: Bytes = buf.to_bytes();
    /// assert_eq!(full[..], b"hello orld"[..]);
    /// ```
    pub fn last_mut(&mut self) -> &mut U {
        &mut self.b
    }

    /// Consumes this `Chain`, returning the underlying values.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::{Bytes, Buf};
    ///
    /// let chain = Bytes::from(&b"hello"[..])
    ///             .chain(Bytes::from(&b"world"[..]));
    ///
    /// let (first, last) = chain.into_inner();
    /// assert_eq!(first[..], b"hello"[..]);
    /// assert_eq!(last[..], b"world"[..]);
    /// ```
    pub fn into_inner(self) -> (T, U) {
        (self.a, self.b)
    }
}

impl<T, U> Buf for Chain<T, U>
    where T: Buf,
          U: Buf,
{
    fn remaining(&self) -> usize {
        self.a.remaining() + self.b.remaining()
    }

    fn bytes(&self) -> &[u8] {
        if self.a.has_remaining() {
            self.a.bytes()
        } else {
            self.b.bytes()
        }
    }

    fn advance(&mut self, mut cnt: usize) {
        let a_rem = self.a.remaining();

        if a_rem != 0 {
            if a_rem >= cnt {
                self.a.advance(cnt);
                return;
            }

            // Consume what is left of a
            self.a.advance(a_rem);

            cnt -= a_rem;
        }

        self.b.advance(cnt);
    }

    #[cfg(feature = "std")]
    fn bytes_vectored<'a>(&'a self, dst: &mut [IoSlice<'a>]) -> usize {
        let mut n = self.a.bytes_vectored(dst);
        n += self.b.bytes_vectored(&mut dst[n..]);
        n
    }

    fn to_bytes(&mut self) -> crate::Bytes {
        let mut bytes: crate::BytesMut = self.a.to_bytes().try_mut()
            .unwrap_or_else(|bytes| bytes.into());

        bytes.put(&mut self.b);
        bytes.freeze()
    }
}

impl<T, U> BufMut for Chain<T, U>
    where T: BufMut,
          U: BufMut,
{
    fn remaining_mut(&self) -> usize {
        self.a.remaining_mut() + self.b.remaining_mut()
    }

    unsafe fn bytes_mut(&mut self) -> &mut [u8] {
        if self.a.has_remaining_mut() {
            self.a.bytes_mut()
        } else {
            self.b.bytes_mut()
        }
    }

    unsafe fn advance_mut(&mut self, mut cnt: usize) {
        let a_rem = self.a.remaining_mut();

        if a_rem != 0 {
            if a_rem >= cnt {
                self.a.advance_mut(cnt);
                return;
            }

            // Consume what is left of a
            self.a.advance_mut(a_rem);

            cnt -= a_rem;
        }

        self.b.advance_mut(cnt);
    }

    #[cfg(feature = "std")]
    unsafe fn bytes_vectored_mut<'a>(&'a mut self, dst: &mut [IoSliceMut<'a>]) -> usize {
        let mut n = self.a.bytes_vectored_mut(dst);
        n += self.b.bytes_vectored_mut(&mut dst[n..]);
        n
    }
}

impl<T, U> IntoIterator for Chain<T, U>
where
    T: Buf,
    U: Buf,
{
    type Item = u8;
    type IntoIter = IntoIter<Chain<T, U>>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}
