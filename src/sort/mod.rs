use core::cmp::Ordering;
use core::fmt::{Display, Error, Formatter};
mod from_std_quicksort;

pub trait TrySort<T> {
    fn try_sort(&mut self) -> Result<(), SortError>
    where
        T: PartialOrd<T>;
    fn try_sort_by<F>(&mut self, compare: F) -> Result<(), SortError>
    where
        F: FnMut(&T, &T) -> Option<bool>;
    fn try_sort_by_key<K, F>(&mut self, compare: F) -> Result<(), SortError>
    where
        F: FnMut(&T) -> Option<K>,
        K: PartialOrd<K>;

    fn try_sort_unstable(&mut self) -> Result<(), SortError>
    where
        T: PartialOrd<T>;

    fn try_sort_unstable_by<F>(&mut self, compare: F) -> Result<(), SortError>
    where
        F: FnMut(&T, &T) -> Option<bool>;

    fn try_sort_unstable_by_key<K, F>(&mut self, compare: F) -> Result<(), SortError>
    where
        F: FnMut(&T) -> Option<K>,
        K: PartialOrd<K>;
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default, Debug)]
pub struct SortError;

impl Display for SortError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        fmt.write_str("Sort failed because of uncompareable values")
    }
}

impl std::error::Error for SortError {}

impl<T> TrySort<T> for [T] {
    fn try_sort(&mut self) -> Result<(), SortError>
    where
        T: PartialOrd,
    {
        from_std::merge_sort(self, |a, b| match a.partial_cmp(b) {
            None => None,
            Some(Ordering::Less) => Some(true),
            _ => Some(false),
        })
        .ok_or(SortError)
    }
    fn try_sort_by<F>(&mut self, compare: F) -> Result<(), SortError>
    where
        F: FnMut(&T, &T) -> Option<bool>,
    {
        from_std::merge_sort(self, compare).ok_or(SortError)
    }
    fn try_sort_by_key<K, F>(&mut self, f: F) -> Result<(), SortError>
    where
        F: FnMut(&T) -> Option<K>,
        K: PartialOrd<K>,
    {
        let mut f2 = f;
        from_std::merge_sort(self, |a, b| match f2(a).partial_cmp(&f2(b)) {
            None => None,
            Some(Ordering::Less) => Some(true),
            _ => Some(false),
        })
        .ok_or(SortError)
    }

    fn try_sort_unstable(&mut self) -> Result<(), SortError>
    where
        T: PartialOrd<T>,
    {
        todo!()
    }

    fn try_sort_unstable_by<F>(&mut self, compare: F) -> Result<(), SortError>
    where
        F: FnMut(&T, &T) -> Option<bool>,
    {
        todo!()
    }

    fn try_sort_unstable_by_key<K, F>(&mut self, compare: F) -> Result<(), SortError>
    where
        F: FnMut(&T) -> Option<K>,
        K: PartialOrd<K>,
    {
        todo!()
    }
}

mod from_std {
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

    use std::{mem, ptr};

    /// Inserts `v[0]` into pre-sorted sequence `v[1..]` so that whole `v[..]` becomes sorted.
    ///
    /// This is the integral subroutine of insertion sort.
    fn insert_head<T, F>(v: &mut [T], is_less: &mut F) -> Option<()>
    where
        F: FnMut(&T, &T) -> Option<bool>,
    {
        if v.len() >= 2 && is_less(&v[1], &v[0])? {
            unsafe {
                // There are three ways to implement insertion here:
                //
                // 1. Swap adjacent elements until the first one gets to its final destination.
                //    However, this way we copy data around more than is necessary. If elements are big
                //    structures (costly to copy), this method will be slow.
                //
                // 2. Iterate until the right place for the first element is found. Then shift the
                //    elements succeeding it to make room for it and finally place it into the
                //    remaining hole. This is a good method.
                //
                // 3. Copy the first element into a temporary variable. Iterate until the right place
                //    for it is found. As we go along, copy every traversed element into the slot
                //    preceding it. Finally, copy data from the temporary variable into the remaining
                //    hole. This method is very good. Benchmarks demonstrated slightly better
                //    performance than with the 2nd method.
                //
                // All methods were benchmarked, and the 3rd showed best results. So we chose that one.
                let mut tmp = mem::ManuallyDrop::new(ptr::read(&v[0]));

                // Intermediate state of the insertion process is always tracked by `hole`, which
                // serves two purposes:
                // 1. Protects integrity of `v` from panics in `is_less`.
                // 2. Fills the remaining hole in `v` in the end.
                //
                // Panic safety:
                //
                // If `is_less` panics at any point during the process, `hole` will get dropped and
                // fill the hole in `v` with `tmp`, thus ensuring that `v` still holds every object it
                // initially held exactly once.
                let mut hole = InsertionHole {
                    src: &mut *tmp,
                    dest: &mut v[1],
                };
                ptr::copy_nonoverlapping(&v[1], &mut v[0], 1);

                for i in 2..v.len() {
                    if !is_less(&v[i], &*tmp)? {
                        break;
                    }
                    ptr::copy_nonoverlapping(&v[i], &mut v[i - 1], 1);
                    hole.dest = &mut v[i];
                }
                // `hole` gets dropped and thus copies `tmp` into the remaining hole in `v`.
            }
        }
        // When dropped, copies from `src` into `dest`.
        struct InsertionHole<T> {
            src: *mut T,
            dest: *mut T,
        }

        impl<T> Drop for InsertionHole<T> {
            fn drop(&mut self) {
                unsafe {
                    ptr::copy_nonoverlapping(self.src, self.dest, 1);
                }
            }
        }
        Some(())
    }

    /// Merges non-decreasing runs `v[..mid]` and `v[mid..]` using `buf` as temporary storage, and
    /// stores the result into `v[..]`.
    ///
    /// # Safety
    ///
    /// The two slices must be non-empty and `mid` must be in bounds. Buffer `buf` must be long enough
    /// to hold a copy of the shorter slice. Also, `T` must not be a zero-sized type.
    unsafe fn merge<T, F>(v: &mut [T], mid: usize, buf: *mut T, is_less: &mut F) -> Option<()>
    where
        F: FnMut(&T, &T) -> Option<bool>,
    {
        let len = v.len();
        let v = v.as_mut_ptr();
        let (v_mid, v_end) = unsafe { (v.add(mid), v.add(len)) };

        // The merge process first copies the shorter run into `buf`. Then it traces the newly copied
        // run and the longer run forwards (or backwards), comparing their next unconsumed elements and
        // copying the lesser (or greater) one into `v`.
        //
        // As soon as the shorter run is fully consumed, the process is done. If the longer run gets
        // consumed first, then we must copy whatever is left of the shorter run into the remaining
        // hole in `v`.
        //
        // Intermediate state of the process is always tracked by `hole`, which serves two purposes:
        // 1. Protects integrity of `v` from panics in `is_less`.
        // 2. Fills the remaining hole in `v` if the longer run gets consumed first.
        //
        // Panic safety:
        //
        // If `is_less` panics at any point during the process, `hole` will get dropped and fill the
        // hole in `v` with the unconsumed range in `buf`, thus ensuring that `v` still holds every
        // object it initially held exactly once.
        let mut hole;

        if mid <= len - mid {
            // The left run is shorter.
            unsafe {
                ptr::copy_nonoverlapping(v, buf, mid);
                hole = MergeHole {
                    start: buf,
                    end: buf.add(mid),
                    dest: v,
                };
            }

            // Initially, these pointers point to the beginnings of their arrays.
            let left = &mut hole.start;
            let mut right = v_mid;
            let out = &mut hole.dest;

            while *left < hole.end && right < v_end {
                // Consume the lesser side.
                // If equal, prefer the left run to maintain stability.
                unsafe {
                    let to_copy = if is_less(&*right, &**left)? {
                        get_and_increment(&mut right)
                    } else {
                        get_and_increment(left)
                    };
                    ptr::copy_nonoverlapping(to_copy, get_and_increment(out), 1);
                }
            }
        } else {
            // The right run is shorter.
            unsafe {
                ptr::copy_nonoverlapping(v_mid, buf, len - mid);
                hole = MergeHole {
                    start: buf,
                    end: buf.add(len - mid),
                    dest: v_mid,
                };
            }

            // Initially, these pointers point past the ends of their arrays.
            let left = &mut hole.dest;
            let right = &mut hole.end;
            let mut out = v_end;

            while v < *left && buf < *right {
                // Consume the greater side.
                // If equal, prefer the right run to maintain stability.
                unsafe {
                    let to_copy = if is_less(&*right.offset(-1), &*left.offset(-1))? {
                        decrement_and_get(left)
                    } else {
                        decrement_and_get(right)
                    };
                    ptr::copy_nonoverlapping(to_copy, decrement_and_get(&mut out), 1);
                }
            }
        }
        // Finally, `hole` gets dropped. If the shorter run was not fully consumed, whatever remains of
        // it will now be copied into the hole in `v`.

        unsafe fn get_and_increment<T>(ptr: &mut *mut T) -> *mut T {
            let old = *ptr;
            *ptr = unsafe { ptr.offset(1) };
            old
        }

        unsafe fn decrement_and_get<T>(ptr: &mut *mut T) -> *mut T {
            *ptr = unsafe { ptr.offset(-1) };
            *ptr
        }

        // When dropped, copies the range `start..end` into `dest..`.
        struct MergeHole<T> {
            start: *mut T,
            end: *mut T,
            dest: *mut T,
        }

        impl<T> Drop for MergeHole<T> {
            fn drop(&mut self) {
                // `T` is not a zero-sized type, so it's okay to divide by its size.
                let len = (self.end as usize - self.start as usize) / mem::size_of::<T>();
                unsafe {
                    ptr::copy_nonoverlapping(self.start, self.dest, len);
                }
            }
        }

        Some(())
    }

    /// This merge sort borrows some (but not all) ideas from TimSort, which is described in detail
    /// [here](http://svn.python.org/projects/python/trunk/Objects/listsort.txt).
    ///
    /// The algorithm identifies strictly descending and non-descending subsequences, which are called
    /// natural runs. There is a stack of pending runs yet to be merged. Each newly found run is pushed
    /// onto the stack, and then some pairs of adjacent runs are merged until these two invariants are
    /// satisfied:
    ///
    /// 1. for every `i` in `1..runs.len()`: `runs[i - 1].len > runs[i].len`
    /// 2. for every `i` in `2..runs.len()`: `runs[i - 2].len > runs[i - 1].len + runs[i].len`
    ///
    /// The invariants ensure that the total running time is `O(n * log(n))` worst-case.
    pub fn merge_sort<T, F>(v: &mut [T], mut is_less: F) -> Option<()>
    where
        F: FnMut(&T, &T) -> Option<bool>,
    {
        // Slices of up to this length get sorted using insertion sort.
        const MAX_INSERTION: usize = 20;
        // Very short runs are extended using insertion sort to span at least this many elements.
        const MIN_RUN: usize = 10;

        // Sorting has no meaningful behavior on zero-sized types.
        if mem::size_of::<T>() == 0 {
            return Some(());
        }

        let len = v.len();

        // Short arrays get sorted in-place via insertion sort to avoid allocations.
        if len <= MAX_INSERTION {
            if len >= 2 {
                for i in (0..len - 1).rev() {
                    insert_head(&mut v[i..], &mut is_less);
                }
            }
            return Some(());
        }

        // Allocate a buffer to use as scratch memory. We keep the length 0 so we can keep in it
        // shallow copies of the contents of `v` without risking the dtors running on copies if
        // `is_less` panics. When merging two sorted runs, this buffer holds a copy of the shorter run,
        // which will always have length at most `len / 2`.
        let mut buf = Vec::with_capacity(len / 2);

        // In order to identify natural runs in `v`, we traverse it backwards. That might seem like a
        // strange decision, but consider the fact that merges more often go in the opposite direction
        // (forwards). According to benchmarks, merging forwards is slightly faster than merging
        // backwards. To conclude, identifying runs by traversing backwards improves performance.
        let mut runs = vec![];
        let mut end = len;
        while end > 0 {
            // Find the next natural run, and reverse it if it's strictly descending.
            let mut start = end - 1;
            if start > 0 {
                start -= 1;
                unsafe {
                    if is_less(v.get_unchecked(start + 1), v.get_unchecked(start))? {
                        while start > 0
                            && is_less(v.get_unchecked(start), v.get_unchecked(start - 1))?
                        {
                            start -= 1;
                        }
                        v[start..end].reverse();
                    } else {
                        while start > 0
                            && !is_less(v.get_unchecked(start), v.get_unchecked(start - 1))?
                        {
                            start -= 1;
                        }
                    }
                }
            }

            // Insert some more elements into the run if it's too short. Insertion sort is faster than
            // merge sort on short sequences, so this significantly improves performance.
            while start > 0 && end - start < MIN_RUN {
                start -= 1;
                insert_head(&mut v[start..end], &mut is_less);
            }

            // Push this run onto the stack.
            runs.push(Run {
                start,
                len: end - start,
            });
            end = start;

            // Merge some pairs of adjacent runs to satisfy the invariants.
            while let Some(r) = collapse(&runs) {
                let left = runs[r + 1];
                let right = runs[r];
                unsafe {
                    merge(
                        &mut v[left.start..right.start + right.len],
                        left.len,
                        buf.as_mut_ptr(),
                        &mut is_less,
                    );
                }
                runs[r] = Run {
                    start: left.start,
                    len: left.len + right.len,
                };
                runs.remove(r + 1);
            }
        }

        // Finally, exactly one run must remain in the stack.
        debug_assert!(runs.len() == 1 && runs[0].start == 0 && runs[0].len == len);

        // Examines the stack of runs and identifies the next pair of runs to merge. More specifically,
        // if `Some(r)` is returned, that means `runs[r]` and `runs[r + 1]` must be merged next. If the
        // algorithm should continue building a new run instead, `None` is returned.
        //
        // TimSort is infamous for its buggy implementations, as described here:
        // http://envisage-project.eu/timsort-specification-and-verification/
        //
        // The gist of the story is: we must enforce the invariants on the top four runs on the stack.
        // Enforcing them on just top three is not sufficient to ensure that the invariants will still
        // hold for *all* runs in the stack.
        //
        // This function correctly checks invariants for the top four runs. Additionally, if the top
        // run starts at index 0, it will always demand a merge operation until the stack is fully
        // collapsed, in order to complete the sort.
        #[inline]
        fn collapse(runs: &[Run]) -> Option<usize> {
            let n = runs.len();
            if n >= 2
                && (runs[n - 1].start == 0
                    || runs[n - 2].len <= runs[n - 1].len
                    || (n >= 3 && runs[n - 3].len <= runs[n - 2].len + runs[n - 1].len)
                    || (n >= 4 && runs[n - 4].len <= runs[n - 3].len + runs[n - 2].len))
            {
                if n >= 3 && runs[n - 3].len < runs[n - 1].len {
                    Some(n - 3)
                } else {
                    Some(n - 2)
                }
            } else {
                None
            }
        }

        #[derive(Clone, Copy)]
        struct Run {
            start: usize,
            len: usize,
        }

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::*;
    use rand::distributions::Standard;
    use rand::prelude::*;

    #[test]
    fn try_sort_ok() {
        let rng = thread_rng();
        let mut v: Vec<f32> = Standard.sample_iter(rng).take(100).collect();
        let res = (&mut v).try_sort();
        assert!(res.is_ok());
        assert!(v.is_sorted())
    }

    #[test]
    fn try_sort_error() {
        let rng = thread_rng();
        let mut v: Vec<f32> = Standard.sample_iter(rng).take(100).collect();
        v.push(f32::NAN);
        let res = (&mut v).try_sort();
        assert!(res.is_err());
        assert!(!v.is_sorted())
    }
}
