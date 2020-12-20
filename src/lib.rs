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

#![feature(maybe_uninit_slice, is_sorted, min_specialization)]
#![no_std]
#[cfg(feature = "std")]
extern crate std;

mod binary_search;
mod min_max;
mod sort;
pub use binary_search::TryBinarySearch;
use core::fmt::{Display, Error, Formatter};
pub use min_max::TryMinMax;
pub use sort::TrySort;

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
