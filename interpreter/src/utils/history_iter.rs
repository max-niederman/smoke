use std::collections::linked_list::{Cursor, LinkedList};
use std::sync::RwLock;

/// A structure which allows for the use of one iterator as though it were multiple
///
/// This is achieved by remembering each item produced by the iterator
#[derive(Debug, Clone)]
pub struct HistoryIter<I: Iterator> {
    /// The underlying [`Iterator`]
    inner: I,

    /// History of consumed items
    history: LinkedList<I::Item>,
    // TODO: Use forbidden magic to intelligently pop items from the back of the history
}

impl<I> HistoryIter<I>
where
    I: Iterator,
    I::Item: Clone,
{
    pub fn new(iter: I) -> Self {
        Self {
            inner: iter,
            history: LinkedList::new(),
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
            self.history.push_front(item);
        }

        next
    }
}

/// A view into a [`HistoryIter`]
#[derive(Debug, Clone)]
pub struct HistoryIterView<'s, I: Iterator> {
    /// The underlying [`HistoryIter`]
    source: &'s RwLock<HistoryIter<I>>,

    /// Cursor into the source's history
    ///
    /// Its use as unsafe, since it should only be used when we have a read lock on `source`
    cursor: Cursor<'s, I::Item>,
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
            // This is fine because the cursor is only used when `self.source` is read-locked
            cursor: unsafe { std::mem::transmute(source.read().unwrap().history.cursor_back()) },
        }
    }

    pub fn is_caught_up(&self) -> bool {
        self.cursor.peek_next().is_none()
    }
}

impl<'s, I> Iterator for HistoryIterView<'s, I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        if self.is_caught_up() {
            self.source.write().unwrap().next()
        } else {
            let lock = self.source.read();
            self.cursor.move_next();
            let item = self.cursor.current().cloned();
            drop(lock);

            item
        }
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
