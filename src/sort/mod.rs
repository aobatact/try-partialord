use crate::{InvalidOrderError, OrderResult};
use core::cmp::Ordering;
mod std_quicksort;

/// Sort methods for PratialOrd
pub trait TrySort<T> {
    #[cfg(feature = "std")]
    fn try_sort(&mut self) -> OrderResult<()>
    where
        T: PartialOrd<T>;

    #[cfg(feature = "std")]
    fn try_sort_by<F>(&mut self, compare: F) -> OrderResult<()>
    where
        F: FnMut(&T, &T) -> Option<bool>;

    #[cfg(feature = "std")]
    fn try_sort_by_key<K, F>(&mut self, f: F) -> OrderResult<()>
    where
        F: FnMut(&T) -> Option<K>,
        K: PartialOrd<K>;

    fn try_sort_unstable(&mut self) -> OrderResult<()>
    where
        T: PartialOrd<T>;

    fn try_sort_unstable_by<F>(&mut self, compare: F) -> OrderResult<()>
    where
        F: FnMut(&T, &T) -> Option<bool>;

    fn try_sort_unstable_by_key<K, F>(&mut self, f: F) -> OrderResult<()>
    where
        F: FnMut(&T) -> Option<K>,
        K: PartialOrd<K>;
}

impl<T> TrySort<T> for [T] {
    #[inline]
    #[cfg(feature = "std")]
    fn try_sort(&mut self) -> OrderResult<()>
    where
        T: PartialOrd,
    {
        std_mergesort::merge_sort(self, |a, b| a.partial_cmp(b).map(|a| a == Ordering::Less))
            .ok_or(InvalidOrderError)
    }

    #[inline]
    #[cfg(feature = "std")]
    fn try_sort_by<F>(&mut self, compare: F) -> OrderResult<()>
    where
        F: FnMut(&T, &T) -> Option<bool>,
    {
        std_mergesort::merge_sort(self, compare).ok_or(InvalidOrderError)
    }

    #[inline]
    #[cfg(feature = "std")]
    fn try_sort_by_key<K, F>(&mut self, f: F) -> OrderResult<()>
    where
        F: FnMut(&T) -> Option<K>,
        K: PartialOrd<K>,
    {
        let mut f2 = f;
        std_mergesort::merge_sort(self, |a, b| {
            f2(a).partial_cmp(&f2(b)).map(|a| a == Ordering::Less)
        })
        .ok_or(InvalidOrderError)
    }

    #[inline]
    fn try_sort_unstable(&mut self) -> OrderResult<()>
    where
        T: PartialOrd<T>,
    {
        std_quicksort::quicksort(self, |a, b| a.partial_cmp(b).map(|a| a == Ordering::Less))
            .ok_or(InvalidOrderError)
    }

    #[inline]
    fn try_sort_unstable_by<F>(&mut self, compare: F) -> OrderResult<()>
    where
        F: FnMut(&T, &T) -> Option<bool>,
    {
        std_quicksort::quicksort(self, compare).ok_or(InvalidOrderError)
    }

    #[inline]
    fn try_sort_unstable_by_key<K, F>(&mut self, f: F) -> OrderResult<()>
    where
        F: FnMut(&T) -> Option<K>,
        K: PartialOrd<K>,
    {
        let mut f2 = f;
        std_quicksort::quicksort(self, |a, b| {
            f2(a).partial_cmp(&f2(b)).map(|a| a == Ordering::Less)
        })
        .ok_or(InvalidOrderError)
    }
}

#[cfg(feature = "std")]
mod std_mergesort;

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use crate::sort::*;
    use rand::distributions::Standard;
    use rand::prelude::*;
    use std::vec::Vec;

    #[test]
    fn try_sort_ok() {
        let rng = thread_rng();
        let mut v: Vec<f32> = Standard.sample_iter(rng).take(100).collect();
        let res = v.try_sort();
        assert!(res.is_ok());
        assert!(v.is_sorted())
    }

    #[test]
    fn try_sort_error() {
        let rng = thread_rng();
        let mut v: Vec<f32> = Standard.sample_iter(rng).take(100).collect();
        v.push(f32::NAN);
        let res = v.try_sort();
        assert!(res.is_err());
        assert!(!v.is_sorted())
    }
}
