#![feature(maybe_uninit_slice, is_sorted)]

mod sort;
pub use sort::{SortError, TrySort};
