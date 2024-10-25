#![deny(missing_docs)]
//! Rust function to permute a numeric range.

use feistel_permutation_rs::{DefaultBuildHasher, Permutation};

/// Creates a function that permutes integers in the range `0..(size-1)`.
/// When the input is in the range, the permutation function returns
/// `Ok(i)` for some `i` in the range. Otherwise, the permutation function
/// returns an error.
/// # Example
/// ```
/// use range_permutation::create;
/// let perm = create(10);
/// for i in 0..9 {
///   let p = perm(i).unwrap();
///   assert!(0 <= p && p <= 9);
/// }
/// assert!(perm(10).is_err());
/// ```
pub fn create(size: u64) -> impl Fn(u64) -> Result<u64, &'static str> {
    let perm = Permutation::new(size, 29, DefaultBuildHasher::new());
    move |n| {
        if n >= size {
            Err("out of range")
        } else {
            Ok(perm.get(n))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn is_permutation() {
        let n : u64 = 1000;
        let perm = create(n);
        let mut results = HashSet::new();
        for i in 0..n {
            let p = perm(i).unwrap();
            assert!(p < n);
            results.insert(p);
        }
        assert_eq!(results.len(), n as usize);
    }

    #[test]
    fn checks_range() {
        let perm = create(1);
        assert!(perm(0).is_ok());
        assert!(perm(1).is_err())
    }
}
