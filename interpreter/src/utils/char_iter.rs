use super::array_vec::ArrayVec;
use std::io::{self, Read};
use std::iter::FilterMap;

#[derive(Debug, Clone)]
pub struct CharIter<I: Iterator<Item = u8>> {
    iter: I,
}

impl<I: Iterator<Item = u8>> CharIter<I> {
    pub fn new(iter: I) -> Self {
        Self { iter }
    }
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
        CharIter::new(self.bytes().filter_map(Result::ok))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_iter_iterates() {
        let source = "Hello world!";

        let correct = source.chars();
        let from_bytes = CharIter::new(source.bytes());

        assert!(from_bytes.eq(correct));
    }
}
