use std::io::{self, Read};
use std::iter::FilterMap;
use std::mem::MaybeUninit;
use std::ops::Deref;

pub trait IntoCharIter {
    type IntoIter: Iterator<Item = char>;
    fn chars(self) -> Self::IntoIter;
}

impl<'a> IntoCharIter for io::StdinLock<'a> {
    type IntoIter = CharIter<
        FilterMap<
            std::io::Bytes<io::StdinLock<'a>>,
            fn(std::result::Result<u8, std::io::Error>) -> Option<u8>,
        >,
    >;
    fn chars(self) -> Self::IntoIter {
        CharIter {
            iter: self.bytes().filter_map(Result::ok),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CharIter<I: Iterator<Item = u8>> {
    iter: I,
}

impl<I: Iterator<Item = u8>> Iterator for CharIter<I> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = ArrayVec::<u8, 4>::new();
        while !buf.is_full() {
            if let Some(byte) = self.iter.next() {
                buf.push(byte);
                if let Ok(st) = std::str::from_utf8(&buf) {
                    return st.chars().next();
                }
            } else {
                break;
            }
        }
        None
    }
}

#[derive(Debug)]
struct ArrayVec<T, const N: usize> {
    inner: [MaybeUninit<T>; N],
    length: usize,
}

impl<T, const N: usize> ArrayVec<T, N> {
    fn new() -> Self {
        Self {
            inner: MaybeUninit::uninit_array(),
            length: 0,
        }
    }

    fn is_full(&self) -> bool {
        self.length >= N
    }

    #[cfg(test)]
    fn clear(&mut self) {
        self.length = 0;
    }

    fn push(&mut self, item: T) {
        assert!(self.length < N, "tried to push to a full ArrayVec");
        self.inner[self.length].write(item);
        self.length += 1;
    }
}

impl<T, const N: usize> Deref for ArrayVec<T, N> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { MaybeUninit::slice_assume_init_ref(&self.inner[..self.length]) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_vec_grows() {
        let mut av: ArrayVec<u8, 4> = ArrayVec::new();
        assert_eq!(av.len(), 0);
        assert_eq!(&av.deref(), &[]);

        av.push(0);
        assert_eq!(av.len(), 1);
        assert_eq!(&av.deref(), &[0]);

        av.push(0);
        av.push(0);
        av.push(0);
        assert_eq!(av.len(), 4);
        assert!(av.is_full());
        assert_eq!(&av.deref(), &[0, 0, 0, 0]);

        av.clear();
        assert_eq!(av.len(), 0);
        assert_eq!(&av.deref(), &[]);
    }
}
