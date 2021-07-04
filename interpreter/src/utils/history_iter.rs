use std::sync::RwLock;

/// A structure which allows for the use of one iterator as though it were multiple
///
/// This is achieved by remembering each item produced by the iterator
#[derive(Debug, Clone)]
pub struct HistoryIter<I: Iterator> {
    /// The underlying [`Iterator`]
    inner: I,

    /// History of consumed items
    history: Vec<I::Item>,
}

impl<I> HistoryIter<I>
where
    I: Iterator,
    I::Item: Clone,
{
    pub fn new(iter: I) -> Self {
        Self {
            inner: iter,
            history: Vec::new(),
        }
    }
}

impl<I> Iterator for HistoryIter<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next();

        if let Some(item) = next.clone() {
            self.history.push(item);
        }

        next
    }
}

/// A view into a [`HistoryIter`]
pub struct HistoryIterView<'s, I: Iterator> {
    /// The underlying [`HistoryIter`]
    source: &'s RwLock<HistoryIter<I>>,

    /// Index into the source's history
    current: usize,
}

impl<'s, I> HistoryIterView<'s, I>
where
    I: Iterator,
    I::Item: Clone,
{
    /// Create a new view into a [`HistoryIter`]
    pub fn new(source: &'s RwLock<HistoryIter<I>>) -> Self {
        Self {
            source,
            current: 0,
        }
    }

    pub fn is_caught_up(&self) -> bool {
        self.source.read().unwrap().history.get(self.current).is_none()
    }
}

impl<'s, I> Clone for HistoryIterView<'s, I>
where
    I: Iterator,
    I::Item: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.source)
    }
}

impl<'s, I> Iterator for HistoryIterView<'s, I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        let ret = if self.is_caught_up() {
            self.source.write().unwrap().next()
        } else {
            self.source.read().unwrap().history.get(self.current).cloned()
        };
        self.current += 1;

        ret
    }
}

pub trait IntoHistoryIter {
    type Inner: Iterator;
    fn into_history(self) -> RwLock<HistoryIter<Self::Inner>>;
}
impl<I> IntoHistoryIter for I
where
    I: Iterator,
    I::Item: Clone,
{
    type Inner = Self;
    fn into_history(self) -> RwLock<HistoryIter<Self::Inner>> {
        RwLock::new(HistoryIter::new(self))
    }
}

pub trait HistoryIterContainer<I: Iterator> {
    fn view(&self) -> HistoryIterView<I>;
}
impl<I> HistoryIterContainer<I> for RwLock<HistoryIter<I>>
where
    I: Iterator,
    I::Item: Clone,
{
    fn view(&self) -> HistoryIterView<I> {
        HistoryIterView::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &[usize] = &[1, 2, 3];

    #[test]
    fn master_iterates() {
        let iter = SOURCE.iter().into_history();
        let mut lock = iter.write().unwrap();

        for item in SOURCE {
            assert_eq!(
                lock.next(),
                Some(item)
            );
        }
        assert_eq!(
            lock.next(),
            None
        );
    }

    #[test]
    fn view_iterates() {
        let iter = SOURCE.iter().into_history();
        let mut view = iter.view();

        eprintln!("{:#?}", iter);
        for item in SOURCE {
            assert_eq!(
                view.next(),
                Some(item),
            );
        }
        assert_eq!(
            view.next(),
            None
        );
    }
}
