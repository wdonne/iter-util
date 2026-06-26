//! This adds the method `fold_result` to the trait `Iterator`. An iterator with `Result` values is
//! folded into another `Result` value. It stops when an error value is encountered, which is then
//! reported.
//!
//! ```rust
//! use iter_util::fold_result::FoldResultExt;
//!
//! let data_error: Vec<Result<i32, &str>> = vec![Ok(0), Err("error"), Ok(2)];
//! let data_ok: Vec<Result<i32, String>> = vec![Ok(0), Ok(1), Ok(2)];
//!
//! assert_eq!(data_ok.into_iter().fold_result(0, |a, v| a + v), Ok(3));
//! assert_eq!(data_error.into_iter().fold_result(0, |a, v| a + v), Err("error"));
//! ```

pub trait FoldResultExt<B, F, T, E>
where
    F: FnMut(B, T) -> B,
{
    fn fold_result(&mut self, init: B, f: F) -> Result<B, E>;
}

impl<I, B, F, T, E> FoldResultExt<B, F, T, E> for I
where
    F: FnMut(B, T) -> B,
    I: Iterator<Item = Result<T, E>>,
{
    fn fold_result(&mut self, init: B, mut f: F) -> Result<B, E>
    where
        F: FnMut(B, T) -> B,
    {
        let mut accum = Ok(init);

        while accum.is_ok()
            && let Some(x) = self.next()
        {
            match x {
                Ok(v) => accum = Ok(f(accum.ok().unwrap(), v)),
                Err(e) => accum = Err(e),
            }
        }

        accum
    }
}