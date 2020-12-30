use crate::{InvalidOrderError, OrderResult};
use core::cmp::Ordering;

/// Binary Search methods for PratialOrd
pub trait TryBinarySearch<T> {
    fn try_binary_search(&self, x: &T) -> OrderResult<Result<usize, usize>>
    where
        T: PartialOrd<T>;
    fn try_binary_search_by<F>(&self, compare: F) -> OrderResult<Result<usize, usize>>
    where
        F: FnMut(&T) -> Option<Ordering>;
    fn try_binary_search_by_key<K, F>(&self, b: &K, f: F) -> OrderResult<Result<usize, usize>>
    where
        F: FnMut(&T) -> Option<K>,
        K: PartialOrd<K>;
}

impl<T> TryBinarySearch<T> for [T] {
    #[inline]
    fn try_binary_search(&self, x: &T) -> OrderResult<Result<usize, usize>>
    where
        T: PartialOrd<T>,
    {
        self.try_binary_search_by(|a| a.partial_cmp(x))
    }

    #[inline]
    fn try_binary_search_by<F>(&self, compare: F) -> OrderResult<Result<usize, usize>>
    where
        F: FnMut(&T) -> Option<Ordering>,
    {
        try_binary_search_by_inner(self, compare).ok_or(InvalidOrderError)
    }

    #[inline]
    fn try_binary_search_by_key<K, F>(&self, b: &K, f: F) -> OrderResult<Result<usize, usize>>
    where
        F: FnMut(&T) -> Option<K>,
        K: PartialOrd<K>,
    {
        let mut fk = f;
        self.try_binary_search_by(|a| fk(a)?.partial_cmp(b))
    }
}

fn try_binary_search_by_inner<T, F>(slice: &[T], compare: F) -> Option<Result<usize, usize>>
where
    F: FnMut(&T) -> Option<Ordering>,
{
    let s = slice;
    let mut size = s.len();
    if size == 0 {
        return Some(Err(0));
    }
    let mut compare = compare;
    let mut base = 0usize;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        // SAFETY: the call is made safe by the following inconstants:
        // - `mid >= 0`: by definition
        // - `mid < size`: `mid = size / 2 + size / 4 + size / 8 ...`
        let cmp = compare(unsafe { s.get_unchecked(mid) });
        if let Some(cmp_1) = cmp {
            base = if cmp_1 == Ordering::Greater {
                base
            } else {
                mid
            };
            size -= half;
        } else {
            return None;
        }
    }
    // SAFETY: base is always in [0, size) because base <= mid.
    if let Some(cmp) = compare(unsafe { s.get_unchecked(base) }) {
        Some(if cmp == Ordering::Equal {
            Ok(base)
        } else {
            Err(base + (cmp == Ordering::Less) as usize)
        })
    } else {
        return None;
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use crate::*;
    use rand::distributions::Standard;
    use rand::prelude::*;
    use std::print;
    use std::vec::Vec;

    #[test]
    fn try_binary_search_ok() {
        let rng = thread_rng();
        let mut v: Vec<f32> = Standard.sample_iter(rng).take(100).collect();
        assert!(v.try_sort().is_ok());
        let b = random();
        print!("t {}", b);
        let i = v.try_binary_search(&b);
        assert!(i.is_ok());
        let ik = match i.unwrap() {
            Ok(o) => o,
            Err(e) => e,
        };
        for sm in v[..ik].iter() {
            //print!("sm {}",sm);
            assert!(sm < &b);
        }
        for sm in v[ik..].iter() {
            assert!(sm >= &b);
        }
    }
}
