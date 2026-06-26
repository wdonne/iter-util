//! This adds the method `after` to the trait `Iterator`. The value passed to it will be emitted
//! when the given iterator is depleted.
//!
//! ```rust
//! use iter_util::after::AfterExt;
//!
//! let e: Vec<u16> = vec![].into_iter().after(0).collect();
//! let v: Vec<u16> = vec![0].into_iter().after(1).collect();
//! 
//! assert_eq!(vec![0], e);
//! assert_eq!(vec![0, 1], v);
//! ```


pub trait AfterExt
where
    Self: Sized + Iterator,
    Self::Item: Clone,
{
    fn after(self, value: Self::Item) -> After<Self>;
}

#[derive(Debug, Clone)]
pub struct After<I>
where
    Self: Sized,
    I: Iterator,
    I::Item: Clone,
{
    iter: I,
    value: I::Item,
    sent_after: bool,
}

impl<I> AfterExt for I
where
    I: Sized + Iterator,
    I::Item: Clone,
{
    fn after(self, value: I::Item) -> After<Self> {
        After::new(self, value)
    }
}

impl<I> After<I>
where
    I: Iterator,
    I::Item: Clone,
{
    const fn new(iter: I, value: I::Item) -> Self {
        Self {
            iter,
            sent_after: false,
            value,
        }
    }
}

impl<I> Iterator for After<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(v) => Some(v),
            None => {
                if !self.sent_after {
                    self.sent_after = true;
                    Some(self.value.clone())
                } else {
                    None
                }
            }
        }
    }
}
