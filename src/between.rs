//! This adds the method `between` to the trait `Iterator`. The value passed to it will be emitted
//! between all the elements of the given iterator.
//!
//! ```rust
//! use iter_util::between::BetweenExt;
//!
//! let v1: Vec<u16> = vec![].into_iter().between(0).collect();
//! let v2: Vec<u16> = vec![0].into_iter().between(0).collect();
//! let v3: Vec<u16> = vec![1, 2].into_iter().between(0).collect();
//! let v4: Vec<u16> = vec![1, 2, 3].into_iter().between(0).collect();
//!
//! assert_eq!(true, v1.is_empty());
//! assert_eq!(vec![0], v2);
//! assert_eq!(vec![1, 0, 2], v3);
//! assert_eq!(vec![1, 0, 2, 0, 3], v4);
//! ```

pub trait BetweenExt
where
    Self: Sized + Iterator,
    Self::Item: Clone,
{
    fn between(self, value: Self::Item) -> Between<Self>;
}

#[derive(Debug, Clone)]
pub struct Between<I>
where
    Self: Sized,
    I: Iterator,
    I::Item: Clone,
{
    first: bool,
    iter: I,
    pending: Option<I::Item>,
    value: I::Item,
}

impl<I> BetweenExt for I
where
    I: Sized + Iterator,
    I::Item: Clone,
{
    fn between(self, value: I::Item) -> Between<Self> {
        Between::new(self, value)
    }
}

impl<I> Between<I>
where
    I: Iterator,
    I::Item: Clone,
{
    const fn new(iter: I, value: I::Item) -> Self {
        Self {
            first: true,
            iter,
            pending: None,
            value,
        }
    }
}

impl<I> Iterator for Between<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            self.iter.next()
        } else {
            match self.pending.clone() {
                None => {
                    self.pending = self.iter.next();

                    if self.pending.is_none() {
                        None
                    } else {
                        Some(self.value.clone())
                    }
                }
                Some(p) => {
                    self.pending = None;
                    Some(p)
                }
            }
        }
    }
}
