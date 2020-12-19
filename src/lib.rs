#![feature(maybe_uninit_slice, is_sorted, min_specialization)]

mod sort;
pub use sort::{SortError, TrySort};
