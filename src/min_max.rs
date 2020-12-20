use crate::{InvalidOrderError, OrderResult};
use core::cmp::Ordering;

pub trait MinMax<T> {
    fn try_min(&mut self) -> OrderResult<T>
    where
        T: PartialOrd<T>;
    fn try_min_by<F>(&mut self, compare: F) -> OrderResult<T>
    where
        F: FnMut(&T, &T) -> Option<Ordering>;
    fn try_min_by_key<K, F>(&mut self, f: F) -> OrderResult<T>
    where
        F: FnMut(&T) -> Option<K>,
        K: PartialOrd<K>;
    fn try_max(&mut self) -> OrderResult<T>
    where
        T: PartialOrd<T>;
    fn try_max_by<F>(&mut self, compare: F) -> OrderResult<T>
    where
        F: FnMut(&T, &T) -> Option<Ordering>;
    fn try_max_by_key<K, F>(&mut self, f: F) -> OrderResult<T>
    where
        F: FnMut(&T) -> Option<K>,
        K: PartialOrd<K>;
}

impl<T, Iter> MinMax<T> for Iter
where
    Iter: Iterator<Item = T>,
{
    #[inline]
    fn try_min(&mut self) -> OrderResult<T>
    where
        T: PartialOrd<T>,
    {
        try_select_by(self, |a, b| a.partial_cmp(b), Ordering::Greater)
    }

    #[inline]
    fn try_max(&mut self) -> OrderResult<T>
    where
        T: PartialOrd<T>,
    {
        try_select_by(self, |a, b| a.partial_cmp(b), Ordering::Less)
    }

    #[inline]
    fn try_min_by<F>(&mut self, compare: F) -> OrderResult<T>
    where
        F: FnMut(&T, &T) -> Option<Ordering>,
    {
        try_select_by(self, compare, Ordering::Greater)
    }

    #[inline]
    fn try_max_by<F>(&mut self, compare: F) -> OrderResult<T>
    where
        F: FnMut(&T, &T) -> Option<Ordering>,
    {
        try_select_by(self, compare, Ordering::Less)
    }

    #[inline]
    fn try_min_by_key<K, F>(&mut self, f: F) -> OrderResult<T>
    where
        F: FnMut(&T) -> Option<K>,
        K: PartialOrd<K>,
    {
        let mut fk = f;
        try_select_by(self, |a, b| fk(a).partial_cmp(&fk(b)), Ordering::Greater)
    }

    #[inline]
    fn try_max_by_key<K, F>(&mut self, f: F) -> OrderResult<T>
    where
        F: FnMut(&T) -> Option<K>,
        K: PartialOrd<K>,
    {
        let mut fk = f;
        try_select_by(self, |a, b| fk(a).partial_cmp(&fk(b)), Ordering::Less)
    }
}

fn try_select_by<T, F>(
    mut iter: impl Iterator<Item = T>,
    compare: F,
    target: Ordering,
) -> OrderResult<T>
where
    F: FnMut(&T, &T) -> Option<Ordering>,
{
    let mut compare = compare;
    iter.try_fold(None, |a: Option<T>, b| match (a, b) {
        (None, _) => None,
        (Some(m), n) if compare(&m, &n)? == target => Some(Some(n)),
        (m, _) => Some(m),
    })
    .flatten()
    .ok_or(InvalidOrderError)
}
