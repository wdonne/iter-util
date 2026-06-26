//! This adds the method `Before` to the trait `Iterator`. The value passed to it will be emitted
//! before the first element of the given iterator is emitted.
//!
//! ```rust
//! use iter_util::before::BeforeExt;
//!
//! let e: Vec<u16> = vec![].into_iter().before(0).collect();
//! let v: Vec<u16> = vec![0].into_iter().before(1).collect();
//!
//! assert_eq!(vec![0], e);
//! assert_eq!(vec![1, 0], v);
//! ```

pub trait BeforeExt
where
    Self: Sized + Iterator,
    Self::Item: Clone,
{
    fn before(self, value: Self::Item) -> Before<Self>;
}

#[derive(Debug, Clone)]
pub struct Before<I>
where
    Self: Sized,
    I: Iterator,
    I::Item: Clone,
{
    first: bool,
    iter: I,
    value: I::Item,
}

impl<I> BeforeExt for I
where
    I: Sized + Iterator,
    I::Item: Clone,
{
    fn before(self, value: I::Item) -> Before<Self> {
        Before::new(self, value)
    }
}

impl<I> Before<I>
where
    I: Iterator,
    I::Item: Clone,
{
    const fn new(iter: I, value: I::Item) -> Self {
        Self {
            first: true,
            iter,
            value,
        }
    }
}

impl<I> Iterator for Before<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            Some(self.value.clone())
        } else {
            self.iter.next()
        }
    }
}
