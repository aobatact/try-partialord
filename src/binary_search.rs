use crate::{InvalidOrderError, OrderResult};
use core::cmp::Ordering;
#[cfg(feature = "try_v2")]
use core::ops::{Residual, Try};

/// Binary Search methods for [`PartialOrd`].
///
/// Caution! This might not return error even if there is invalid order value (like [`f32::NAN`]), because including these value means that it is not sorted correctly and we cannot ensure the return value of binary_search for unsorted slice.
pub trait TryBinarySearch<T> {
    ///[`PartialOrd`] version for [`slice::binary_search`]
    #[inline]
    fn try_binary_search(&self, x: &T) -> OrderResult<Result<usize, usize>>
    where
        T: PartialOrd<T>,
    {
        self.try_binary_search_by(|a| a.partial_cmp(x))
    }

    ///[`Try`] version for [`slice::binary_search_by`]
    #[cfg(feature = "try_v2")]
    fn try_binary_search_by_r<F, R>(
        &self,
        compare: F,
    ) -> <<R as Try>::Residual as Residual<Result<usize, usize>>>::TryType
    where
        F: FnMut(&T) -> R,
        R: core::ops::Try<Output = Ordering>,
        <R as Try>::Residual: Residual<Result<usize, usize>>;

    ///[`PartialOrd`] version for [`slice::binary_search_by`]
    #[cfg(feature = "try_v2")]
    fn try_binary_search_by<F>(&self, compare: F) -> OrderResult<Result<usize, usize>>
    where
        F: FnMut(&T) -> Option<Ordering>,
    {
        self.try_binary_search_by_r(compare)
            .ok_or(InvalidOrderError)
    }

    ///[`PartialOrd`] version for [`slice::binary_search_by`]
    #[cfg(not(feature = "try_v2"))]
    fn try_binary_search_by<F>(&self, compare: F) -> OrderResult<Result<usize, usize>>
    where
        F: FnMut(&T) -> Option<Ordering>;

    #[inline]
    ///[`PartialOrd`] version for [`slice::binary_search_by_key`]
    fn try_binary_search_by_key<K, F>(&self, b: &K, f: F) -> OrderResult<Result<usize, usize>>
    where
        F: FnMut(&T) -> Option<K>,
        K: PartialOrd<K>,
    {
        let mut fk = f;
        self.try_binary_search_by(|a| fk(a)?.partial_cmp(b))
    }
}

impl<T> TryBinarySearch<T> for [T] {
    fn try_binary_search_by<F>(&self, compare: F) -> OrderResult<Result<usize, usize>>
    where
        F: FnMut(&T) -> Option<Ordering>,
    {
        try_binary_search_by_inner(self, compare).ok_or(InvalidOrderError)
    }

    #[cfg(feature = "try_v2")]
    #[inline]
    fn try_binary_search_by_r<F, R>(
        &self,
        compare: F,
    ) -> <<R as Try>::Residual as Residual<Result<usize, usize>>>::TryType
    where
        F: FnMut(&T) -> R,
        R: core::ops::Try<Output = Ordering>,
        <R as Try>::Residual: Residual<Result<usize, usize>>,
    {
        try_binary_search_by_inner(self, compare)
    }
}

#[cfg(feature = "try_v2")]
fn try_binary_search_by_inner<T, F, R>(
    slice: &[T],
    mut compare: F,
) -> <<R as Try>::Residual as Residual<Result<usize, usize>>>::TryType
where
    F: FnMut(&T) -> R,
    R: core::ops::Try<Output = Ordering>,
    <R as Try>::Residual: Residual<Result<usize, usize>>,
{
    let mut size = slice.len();
    let mut left = 0;
    let mut right = size;
    while size > 0 {
        let mid = left + size / 2;

        // SAFETY: the call is made safe by the following invariants:
        // - `mid >= 0`
        // - `mid < size`: `mid` is limited by `[left; right)` bound.
        let cmp = compare(unsafe { slice.get_unchecked(mid) })?;

        // The reason why we use if/else control flow rather than match
        // is because match reorders comparison operations, which is perf sensitive.
        // This is x86 asm for u8: https://rust.godbolt.org/z/8Y8Pra.

        if cmp == Ordering::Less {
            left = mid + 1;
        } else if cmp == Ordering::Greater {
            right = mid;
        } else {
            // SAFETY: same as the `get_unchecked` above
            //unsafe { core::intrinsics::assume(mid < slice.len()) };
            return Try::from_output(Ok(mid));
        }

        size = right - left;
    }
    Try::from_output(Err(left))
}

#[cfg(not(feature = "try_v2"))]
fn try_binary_search_by_inner<T, F>(slice: &[T], mut compare: F) -> Option<Result<usize, usize>>
where
    F: FnMut(&T) -> Option<Ordering>,
{
    let mut size = slice.len();
    let mut left = 0;
    let mut right = size;
    while size > 0 {
        let mid = left + size / 2;

        // SAFETY: the call is made safe by the following invariants:
        // - `mid >= 0`
        // - `mid < size`: `mid` is limited by `[left; right)` bound.
        let cmp = compare(unsafe { slice.get_unchecked(mid) })?;

        // The reason why we use if/else control flow rather than match
        // is because match reorders comparison operations, which is perf sensitive.
        // This is x86 asm for u8: https://rust.godbolt.org/z/8Y8Pra.

        if cmp == Ordering::Less {
            left = mid + 1;
        } else if cmp == Ordering::Greater {
            right = mid;
        } else {
            // SAFETY: same as the `get_unchecked` above
            //unsafe { core::intrinsics::assume(mid < slice.len()) };
            return Some(Ok(mid));
        }

        size = right - left;
    }
    Some(Err(left))
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use crate::*;
    use rand::distributions::Standard;
    use rand::prelude::*;
    use std::vec::Vec;

    #[test]
    fn try_binary_search_ok() {
        let rng = thread_rng();
        let mut v: Vec<f32> = Standard.sample_iter(rng).take(100).collect();
        assert!(v.try_sort().is_ok());
        let b = random();
        let i = v.try_binary_search(&b);
        assert!(i.is_ok());
        let ik = i.unwrap().unwrap_or_else(|e| e);
        for sm in v[..ik].iter() {
            //print!("sm {}",sm);
            assert!(sm < &b);
        }
        for sm in v[ik..].iter() {
            assert!(sm >= &b);
        }
    }
}
