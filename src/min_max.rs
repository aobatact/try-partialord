use crate::{InvalidOrderError, OrderResult};
use core::cmp::Ordering;

/// Min and max methods for [`PartialOrd`]
/// ```
/// use try_partialord::*;
/// use rand::distributions::Standard;
/// use rand::prelude::*;
///
/// let rng = thread_rng();
/// let mut v: Vec<f32> = Standard.sample_iter(rng).take(100).collect();
/// let min = v.iter().try_min().unwrap();
/// assert_eq!(min, v.iter().min_by(|a, b| a.partial_cmp(b).unwrap()));
///
/// // min is error because of uncompareabale value `NAN`
/// v.push(f32::NAN);
/// let min = v.iter().try_min();
/// assert!(min.is_err());
/// ```
pub trait TryMinMax<T> {
    /// `PartialOrd` version for [`Iterator::min`].
    #[inline]
    fn try_min(self) -> OrderResult<Option<T>>
    where
        T: PartialOrd<T>,
        Self: Sized,
    {
        self.try_select_by(|a, b| a.partial_cmp(b), Ordering::Greater)
    }
    /// `PartialOrd` version for [`Iterator::min_by`].
    #[inline]
    fn try_min_by<F>(self, compare: F) -> OrderResult<Option<T>>
    where
        F: FnMut(&T, &T) -> Option<Ordering>,
        Self: Sized,
    {
        self.try_select_by(compare, Ordering::Greater)
    }
    /// `PartialOrd` version for [`Iterator::min_by_key`].
    #[inline]
    fn try_min_by_key<K, F>(self, f: F) -> OrderResult<Option<T>>
    where
        F: FnMut(&T) -> Option<K>,
        K: PartialOrd<K>,
        Self: Sized,
    {
        let mut fk = f;
        self.try_select_by(|a, b| fk(a).partial_cmp(&fk(b)), Ordering::Greater)
    }
    /// `PartialOrd` version for [`Iterator::max`].
    #[inline]
    fn try_max(self) -> OrderResult<Option<T>>
    where
        T: PartialOrd<T>,
        Self: Sized,
    {
        self.try_select_by(|a, b| a.partial_cmp(b), Ordering::Less)
    }
    /// `PartialOrd` version for [`Iterator::max_by`].
    #[inline]
    fn try_max_by<F>(self, compare: F) -> OrderResult<Option<T>>
    where
        F: FnMut(&T, &T) -> Option<Ordering>,
        Self: Sized,
    {
        self.try_select_by(compare, Ordering::Less)
    }
    /// `PartialOrd` version for [`Iterator::max_by_key`].
    #[inline]
    fn try_max_by_key<K, F>(self, f: F) -> OrderResult<Option<T>>
    where
        F: FnMut(&T) -> Option<K>,
        K: PartialOrd<K>,
        Self: Sized,
    {
        let mut fk = f;
        self.try_select_by(|a, b| fk(a).partial_cmp(&fk(b)), Ordering::Less)
    }
    /// Base method for getting min or max. `target` is to tell what you want to get is min or max.
    /// - min -> `Ordering::Greater`
    /// - max -> `Ordering::Less`
    fn try_select_by<F>(self, compare: F, target: Ordering) -> OrderResult<Option<T>>
    where
        F: FnMut(&T, &T) -> Option<Ordering>;
}

impl<T, Iter> TryMinMax<T> for Iter
where
    Iter: Iterator<Item = T>,
{
    #[inline]
    fn try_select_by<F>(self, compare: F, target: Ordering) -> OrderResult<Option<T>>
    where
        F: FnMut(&T, &T) -> Option<Ordering>,
    {
        try_select_by(self, compare, target)
    }
}

fn try_select_by<T, F>(
    mut iter: impl Iterator<Item = T>,
    compare: F,
    target: Ordering,
) -> OrderResult<Option<T>>
where
    F: FnMut(&T, &T) -> Option<Ordering>,
{
    let mut compare = compare;
    iter.try_fold(None, |a: Option<T>, b| match (a, b) {
        (None, n) => Some(Some(n)),
        (Some(m), n) if compare(&m, &n)? == target => Some(Some(n)),
        (m, _) => Some(m),
    })
    .ok_or(InvalidOrderError)
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use crate::*;
    use rand::distributions::Standard;
    use rand::prelude::*;
    use std::vec::Vec;

    #[test]
    fn try_min_ok() {
        let rng = thread_rng();
        let v: Vec<f32> = Standard.sample_iter(rng).take(100).collect();
        let min = v.iter().try_min().unwrap();
        assert_eq!(min, v.iter().min_by(|a, b| a.partial_cmp(b).unwrap()));

        let iter = &mut v.iter();
        let min = iter.try_min().unwrap();
        assert_eq!(min, v.iter().min_by(|a, b| a.partial_cmp(b).unwrap()));
    }

    #[test]
    fn try_min_error() {
        let rng = thread_rng();
        let mut v: Vec<f32> = Standard.sample_iter(rng).take(100).collect();
        v.push(f32::NAN);
        let min = v.iter().try_min();
        assert!(min.is_err());
    }
}
