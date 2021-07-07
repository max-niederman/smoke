use std::mem::MaybeUninit;
use std::ops::Deref;

/// An array-backed, stack-allocated growable vector
#[derive(Debug)]
pub struct ArrayVec<T, const N: usize> {
    inner: [MaybeUninit<T>; N],
    length: usize,
}

impl<T, const N: usize> ArrayVec<T, N> {
    pub const fn new() -> Self {
        Self {
            inner: MaybeUninit::uninit_array(),
            length: 0,
        }
    }

    pub const fn is_full(&self) -> bool {
        self.length >= N
    }

    #[cfg(test)]
    pub fn clear(&mut self) {
        self.length = 0;
    }

    pub fn push(&mut self, item: T) {
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
