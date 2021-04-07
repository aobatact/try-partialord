// Mit License from https://github.com/rust-lang/rust
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

// ignore-tidy-undocumented-unsafe

//! Helper traits for [`PartialOrd`](`core::cmp::PartialOrd`) like [`f32`], [`f64`] to use methods where [`Ord`](`core::cmp::Ord`) is needed, like sort, min, max and binarySearch.
//! These methods are almost same as the methods for Ord, exept that it returns [`InvalidOrderError`] when the [`partial_cmp`](`std::cmp::PartialOrd::partial_cmp`)
//! returns [`None`](`core::option::Option::None`).
//! These traits have `try_` methods like [`try_sort`](`TrySort::try_sort`) for `sort`
//!
//! This is safer than using something like `sort_by` with ignoreing None case of [`partial_cmp`](`std::cmp::PartialOrd::partial_cmp`) because it handle error instead of panic.
//! ```
//! # #![feature(is_sorted)]
//! use try_partialord::*;
//! # use rand::distributions::Standard;
//! # use rand::prelude::*;
//!
//! let mut vec: Vec<f32> = Standard.sample_iter(thread_rng()).take(100).collect();
//! //no NAN in vec so sort should succed
//! let sort_result = vec.try_sort();
//! assert!(sort_result.is_ok());
//! assert!(vec.is_sorted());
//! vec.push(f32::NAN);
//! //NAN in vec so sort should fail
//! let sort_result = vec.try_sort();
//! assert!(sort_result.is_err());
//! assert!(!vec.is_sorted());
//! ```

//#![feature(maybe_uninit_slice, is_sorted, min_specialization)]
#![cfg_attr(not(feature = "std"), no_std)]

mod binary_search;
mod min_max;
mod sort;
pub use binary_search::TryBinarySearch;
use core::fmt::{Display, Error, Formatter};
pub use min_max::TryMinMax;
pub use sort::TrySort;

/// Error when [`partial_cmp`](`std::cmp::PartialOrd::partial_cmp`) returns [`None`] during the operation.
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default, Debug)]
pub struct InvalidOrderError;

impl Display for InvalidOrderError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        fmt.write_str("Failed because of uncompareable value")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidOrderError {}

type OrderResult<T> = Result<T, InvalidOrderError>;

/*
pub trait HasOnlyInvalidOrderValue {
    fn is_invalid(&self) -> bool;
    fn as_ordered(self) -> Option<Ordered<Self>>
    where
        Self: Sized,
    {
        if self.is_invalid() {
            Some(Ordered(self))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Ordered<T>(T);

impl<T: core::cmp::PartialEq> Eq for Ordered<T> {}
impl<T: core::cmp::PartialOrd> Ord for Ordered<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
*/
