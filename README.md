# try-partialOrd

No need to wrap `f32`, `f64` to sort any more.

This crate provides helper traits for type with only `PartialOrd` but not `Ord`( like `f32`, `f64`), to use methods where `Ord` is needed, like sort, min, max and binary_search.
These methods are almost same as the methods for Ord, exept that it returns `InvalidOrderError` when the `partial_cmp`
returns `None`.
These traits have `try_` methods like `try_sort` for `slice::sort`
This is safer than using something like `sort_by` with ignoreing None case of `partial_cmp` because it handle error instead of panic.
Sort is using the same logic as std.
```
# #![feature(is_sorted)]
use try_partialord::*;
# use rand::distributions::Standard;
# use rand::prelude::*;
let mut vec: Vec<f32> = Standard.sample_iter(thread_rng()).take(100).collect();
//no NAN in vec so sort should succed
let sort_result = vec.try_sort();
assert!(sort_result.is_ok());
assert!(vec.try_is_sorted().unwrap_or(false));
vec.push(f32::NAN);
//NAN in vec so sort should fail
let sort_result = vec.try_sort();
assert!(sort_result.is_err());
assert!(vec.try_is_sorted().is_err());
```

