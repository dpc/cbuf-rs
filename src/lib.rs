// Modified 2016 Garrett Berg <vitiral@gmail.com>
// Copyright 2015 Dawid Ciężarkiewicz
// See LICENSE-MPL
//

//! Non-thread-shareable, simple and efficient Circular Buffer
//! implementation that can store N elements when full (typical circular
//! buffer implementations store N-1) without using separate flags.
//!
//! Uses only `core` so can be used in `#[no_std]` projects by using
//! `no_std` feature.
#![no_std]
#![feature(const_fn)]
#![feature(test)]

use core::option::Option::{self, Some, None};
use core::marker::PhantomData;

extern crate test as test;

const CBUF_DATA_BIT: usize = !((usize::max_value() << 1) >> 1);

/// Circular Buffer
///
/// Turns a slice into a Circular buffer with head and tail indexes.
#[derive(Debug)]
pub struct CBuf<'a, T: 'a> {
    buf: &'a mut [T],
    ctrl: CBufControl<T>,
}

/// Circular Buffer Control
///
/// Implements the actual logic of Circular Buffer, but requires passing &[T]
/// to `get` and `put`.
#[derive(Debug)]
pub struct CBufControl<T> {
    head: usize,
    tail: usize,
    phantom: PhantomData<T>,
}

#[cfg(not(feature = "no_std"))]
impl<'a, T: Clone> CBuf<'a, T>
    where T: Clone
{
    /// Create new CBuf
    ///
    /// Length (not capacity) will be used to store elements
    /// in the circular buffer.
    ///
    /// panics if buf.len() == 0
    pub fn new(buf: &'a mut [T]) -> CBuf<T> {
        debug_assert!(buf.len() < CBUF_DATA_BIT);
        if buf.len() == 0 {
            panic!("len==0")
        }

        CBuf {
            buf: buf,
            ctrl: CBufControl::new(),
        }
    }

    /// Is buffer full?
    #[inline]
    pub fn is_full(&self) -> bool {
        self.ctrl.is_full()
    }

    /// Is buffer empty?
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.ctrl.is_empty()
    }


    /// Peek next element from the CBuf without removing it
    ///
    /// Returns `None` if buffer is empty.
    #[inline]
    pub fn peek(&mut self) -> Option<T> {
        self.ctrl.peek(self.buf)
    }

    /// Peek next element from the CBuf without removing it
    ///
    /// Makes the buffer misbehave if it's empty.
    #[inline]
    pub fn peek_unchecked(&mut self) -> T {
        self.ctrl.peek_unchecked(self.buf)
    }


    /// Remove one element from the CBuf
    ///
    /// Returns `None` if buffer is empty.
    #[inline]
    pub fn get(&mut self) -> Option<T> {
        self.ctrl.get(self.buf)
    }

    /// Remove one element from the CBuf
    ///
    /// Makes the buffer misbehave if it's empty.
    #[inline]
    pub fn get_unchecked(&mut self) -> T {
        self.ctrl.get_unchecked(self.buf)
    }

    /// Add element the buffer
    ///
    /// Ignores the write if buffer is full.
    #[inline]
    pub fn put(&mut self, val: T) {
        self.ctrl.put(self.buf, val)
    }

    /// Add element the buffer
    ///
    /// Makes the buffer misbehave if it's full.
    #[inline]
    pub fn put_unchecked(&mut self, val: T) {
        self.ctrl.put_unchecked(self.buf, val)
    }
}

impl<T: Clone> CBufControl<T> {
    pub fn new() -> CBufControl<T> {
        CBufControl {
            tail: 0,
            head: 0,
            phantom: PhantomData,
        }
    }

    /// See corresponding method of CBuf
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    /// See corresponding method of CBuf
    #[inline]
    pub fn is_full(&self) -> bool {
        (self.head ^ self.tail) == CBUF_DATA_BIT
    }

    /// See corresponding method of CBuf
    pub fn get(&mut self, buf: &[T]) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        Some(self.get_unchecked(buf))
    }

    /// See corresponding method of CBuf
    pub fn get_unchecked(&mut self, buf: &[T]) -> T {
        let val = buf[self.tail & !CBUF_DATA_BIT].clone();

        self.tail += 1;

        if (self.tail & !CBUF_DATA_BIT) >= buf.len() {
            self.tail = (self.tail - buf.len()) ^ CBUF_DATA_BIT;
        }

        val
    }

    /// See corresponding method of CBuf
    pub fn peek(&mut self, buf: &[T]) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        Some(self.peek_unchecked(buf))
    }

    /// See corresponding method of CBuf
    pub fn peek_unchecked(&mut self, buf: &[T]) -> T {
        buf[self.tail & !CBUF_DATA_BIT].clone()
    }

    /// See corresponding method of CBuf
    pub fn put(&mut self, buf: &mut [T], val: T) {
        if self.is_full() {
            return;
        }
        self.put_unchecked(buf, val)
    }

    /// See corresponding method of CBuf
    pub fn put_unchecked(&mut self, buf: &mut [T], val: T) {
        buf[self.head & !CBUF_DATA_BIT] = val;

        self.head += 1;

        if (self.head & !CBUF_DATA_BIT) >= buf.len() {
            self.head = (self.head - buf.len()) ^ CBUF_DATA_BIT;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use test;

    #[test]
    fn basic_ctl() {
        let mut buf = &mut [0u8, 2];
        let mut cbuf = CBufControl::<u8>::new();

        assert!(cbuf.is_empty());
        assert!(!cbuf.is_full());

        cbuf.put(buf, 3);
        cbuf.put(buf, 4);
        cbuf.put(buf, 42); // will have no effect
        cbuf.put(buf, 42); // will have no effect
        assert!(!cbuf.is_empty());
        assert!(cbuf.is_full());

        assert_eq!(cbuf.peek(buf).unwrap(), 3);
        cbuf.peek(buf).unwrap();
        assert!(!cbuf.is_empty());
        assert!(cbuf.is_full());

        assert_eq!(cbuf.get(buf).unwrap(), 3);
        assert_eq!(cbuf.get(buf).unwrap(), 4);
        assert!(cbuf.is_empty());
        assert!(!cbuf.is_full());

        assert!(cbuf.get(buf).is_none());
        assert!(cbuf.get(buf).is_none());
        cbuf.put(buf, 42);
        assert_eq!(cbuf.get(buf).unwrap(), 42);
    }

    #[test]
    fn basic_cbuf() {
        let mut buf = &mut [0u8, 0u8];
        let mut cbuf = CBuf::new(buf);

        assert!(cbuf.is_empty());
        assert!(!cbuf.is_full());

        cbuf.put(3);
        cbuf.put(4);
        cbuf.put(42); // will have no effect
        cbuf.put(42); // will have no effect
        assert!(!cbuf.is_empty());
        assert!(cbuf.is_full());

        assert_eq!(cbuf.peek().unwrap(), 3);
        cbuf.peek().unwrap();
        assert!(!cbuf.is_empty());
        assert!(cbuf.is_full());

        assert_eq!(cbuf.get().unwrap(), 3);
        assert_eq!(cbuf.get().unwrap(), 4);
        assert!(cbuf.is_empty());
        assert!(!cbuf.is_full());

        assert!(cbuf.get().is_none());
        assert!(cbuf.get().is_none());
        cbuf.put(42);
        assert_eq!(cbuf.get().unwrap(), 42);
    }

    #[test]
    fn patterns() {
        let mut buf = [0u8, 7];
        let mut cbuf = CBufControl::<u8>::new();

        let mut cur_len = 0;
        let mut put_val = 0;
        let mut get_val = 0;

        for pattern in 0..256 {
            if cur_len == 0 {
                assert!(cbuf.is_empty());
            }
            if cur_len == buf.len() {
                assert!(cbuf.is_full());
            }

            for bit_i in 0..8 {
                match pattern & (1 << bit_i) == 0 {
                    true => {
                        if cbuf.is_empty() {
                            assert!(cbuf.peek(&buf).is_none());
                            assert!(cbuf.get(&buf).is_none());
                        } else {
                            assert!(cbuf.peek(&buf).unwrap() == get_val);
                            let val = cbuf.get(&buf).unwrap();
                            assert!(val == get_val);
                            get_val = get_val.wrapping_add(1);
                            cur_len -= 1;
                        }
                    }
                    false => {
                        if cbuf.is_full() {
                            cbuf.put(&mut buf, put_val);
                            assert!(cbuf.is_full());
                        } else {
                            cbuf.put(&mut buf, put_val);
                            put_val = put_val.wrapping_add(1);
                            cur_len += 1;
                        }
                    }
                }
            }
        }
    }

    #[bench]
    pub fn put_and_get(b: &mut Bencher) {
        let buf = &mut [0u8; 256];
        let mut cbuf = CBuf::new(buf);

        b.iter(|| {
            cbuf.put(0u8);
            cbuf.get();
        });

        test::black_box(cbuf.get());
    }

    #[bench]
    pub fn put_unchecked_and_get(b: &mut Bencher) {
        let buf = &mut [0u8; 256];
        let mut cbuf = CBuf::new(buf);

        b.iter(|| {
            cbuf.put_unchecked(0u8);
            cbuf.get_unchecked();
        });

        test::black_box(cbuf.get());
    }
}
