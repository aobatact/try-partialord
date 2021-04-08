use crate::{ord_as_cmp, InvalidOrderError, OrderResult};
use core::cmp::Ordering;
#[cfg(feature = "std")]
mod std_mergesort;
mod std_quicksort;

/// Sort methods for [`PartialOrd`].
pub trait TrySort<T> {
    #[cfg(feature = "std")]
    #[inline]
    /// [`PartialOrd`] version for [`slice::sort`]
    fn try_sort(&mut self) -> OrderResult<()>
    where
        T: PartialOrd<T>,
    {
        self.try_sort_by(ord_as_cmp)
    }
    #[cfg(feature = "std")]
    /// [`PartialOrd`] version for [`slice::sort_by`]
    fn try_sort_by<F>(&mut self, compare: F) -> OrderResult<()>
    where
        F: FnMut(&T, &T) -> Option<bool>;
    #[cfg(feature = "std")]
    #[inline]
    /// [`PartialOrd`] version for [`slice::sort_by_key`]
    fn try_sort_by_key<K, F>(&mut self, f: F) -> OrderResult<()>
    where
        F: FnMut(&T) -> Option<K>,
        K: PartialOrd<K>,
    {
        let mut f2 = f;
        self.try_sort_by(|a, b| f2(a).partial_cmp(&f2(b)).map(|a| a == Ordering::Less))
    }
    #[cfg(feature = "std")]
    /// [`PartialOrd`] version for [`slice::sort_by_cached_key`]
    fn try_sort_by_cached_key<K, F>(&mut self, f: F) -> OrderResult<()>
    where
        F: FnMut(&T) -> Option<K>,
        K: PartialOrd<K>;

    #[inline]
    /// [`PartialOrd`] version for [`slice::sort_unstable`]
    fn try_sort_unstable(&mut self) -> OrderResult<()>
    where
        T: PartialOrd<T>,
    {
        self.try_sort_unstable_by(ord_as_cmp)
    }
    /// [`PartialOrd`] version for [`slice::sort_unstable_by`]
    fn try_sort_unstable_by<F>(&mut self, compare: F) -> OrderResult<()>
    where
        F: FnMut(&T, &T) -> Option<bool>;
    #[inline]
    /// [`PartialOrd`] version for [`slice::sort_unstable_by_key`]
    fn try_sort_unstable_by_key<K, F>(&mut self, f: F) -> OrderResult<()>
    where
        F: FnMut(&T) -> Option<K>,
        K: PartialOrd<K>,
    {
        let mut f2 = f;
        self.try_sort_unstable_by(|a, b| f2(a).partial_cmp(&f2(b)).map(|a| a == Ordering::Less))
    }

    #[inline]
    /// [`PartialOrd`] version for [`slice::is_sorted`]
    fn try_is_sorted(&self) -> OrderResult<bool>
    where
        T: PartialOrd<T>,
    {
        self.try_is_sorted_by(ord_as_cmp)
    }
    /// [`PartialOrd`] version for [`slice::is_sorted_by`]
    fn try_is_sorted_by<F>(&self, compare: F) -> OrderResult<bool>
    where
        F: FnMut(&T, &T) -> Option<bool>;
    #[inline]
    /// [`PartialOrd`] version for [`slice::is_sorted_by_key`]
    fn try_is_sorted_by_key<K, F>(&mut self, f: F) -> OrderResult<bool>
    where
        F: FnMut(&T) -> Option<K>,
        K: PartialOrd<K>,
    {
        let mut f2 = f;
        self.try_is_sorted_by(|a, b| f2(a).partial_cmp(&f2(b)).map(|a| a == Ordering::Less))
    }
}

impl<T> TrySort<T> for [T] {
    #[inline]
    #[cfg(feature = "std")]
    fn try_sort_by<F>(&mut self, compare: F) -> OrderResult<()>
    where
        F: FnMut(&T, &T) -> Option<bool>,
    {
        std_mergesort::merge_sort(self, compare).ok_or(InvalidOrderError)
    }

    #[inline]
    fn try_sort_unstable_by<F>(&mut self, compare: F) -> OrderResult<()>
    where
        F: FnMut(&T, &T) -> Option<bool>,
    {
        std_quicksort::quicksort(self, compare).ok_or(InvalidOrderError)
    }

    #[inline]
    fn try_is_sorted_by<F>(&self, compare: F) -> OrderResult<bool>
    where
        F: FnMut(&T, &T) -> Option<bool>,
    {
        try_is_sorted_by(self, compare)
    }

    #[cfg(feature = "std")]
    #[inline]
    fn try_sort_by_cached_key<K, F>(&mut self, f: F) -> OrderResult<()>
    where
        F: FnMut(&T) -> Option<K>,
        K: PartialOrd<K>,
    {
        // Helper macro for indexing our vector by the smallest possible type, to reduce allocation.
        macro_rules! sort_by_key {
            ($t:ty, $slice:ident, $f:ident) => {{
                let mut indices: Vec<_> = $slice
                    .iter()
                    .map($f)
                    .enumerate()
                    .map(|(i, k)| (k, i as $t))
                    .collect();
                // The elements of `indices` are unique, as they are indexed, so any sort will be
                // stable with respect to the original slice. We use `sort_unstable` here because
                // it requires less memory allocation.
                indices.try_sort_unstable()?;
                for i in 0..$slice.len() {
                    let mut index = indices[i].1;
                    while (index as usize) < i {
                        index = indices[index as usize].1;
                    }
                    indices[i].1 = index;
                    $slice.swap(i, index as usize);
                }
                Ok(())
            }};
        }

        let sz_u8 = core::mem::size_of::<(K, u8)>();
        let sz_u16 = core::mem::size_of::<(K, u16)>();
        let sz_u32 = core::mem::size_of::<(K, u32)>();
        let sz_usize = core::mem::size_of::<(K, usize)>();

        let len = self.len();
        if len < 2 {
            return Ok(());
        }
        if sz_u8 < sz_u16 && len <= (u8::MAX as usize) {
            return sort_by_key!(u8, self, f);
        }
        if sz_u16 < sz_u32 && len <= (u16::MAX as usize) {
            return sort_by_key!(u16, self, f);
        }
        if sz_u32 < sz_usize && len <= (u32::MAX as usize) {
            return sort_by_key!(u32, self, f);
        }
        sort_by_key!(usize, self, f)
    }
}

fn try_is_sorted_by<T, F>(slice: &[T], compare: F) -> OrderResult<bool>
where
    F: FnMut(&T, &T) -> Option<bool>,
{
    let mut cmp = compare;
    if slice.len() > 1 {
        unsafe {
            let mut prev = slice.get_unchecked(0);
            for i in 1..slice.len() {
                let next = slice.get_unchecked(i);
                if let Some(x) = cmp(&prev, &next) {
                    if !x {
                        return Ok(false);
                    }
                    prev = next;
                } else {
                    return Err(InvalidOrderError);
                }
            }
        }
    }
    Ok(true)
}

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
        assert!(v.try_is_sorted().unwrap_or(false))
    }

    #[test]
    fn try_sort_error() {
        let rng = thread_rng();
        let mut v: Vec<f32> = Standard.sample_iter(rng).take(100).collect();
        v.push(f32::NAN);
        let res = v.try_sort();
        assert!(res.is_err());
        assert!(!v.try_is_sorted().is_err())
    }
}
