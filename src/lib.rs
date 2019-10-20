#![no_std]
//! This crate provides a user implementation of the `unreachable_unchecked()` function. See [unreachable_unchecked](fn.unreachable_unchecked.html).

/// A workaround to be able to name the `!` type on the stable compiler.
/// See https://github.com/rust-lang/rust/issues/58733.
trait Bang {
    type Output: ?Sized;
}

impl <T: ?Sized> Bang for fn() -> T {
    type Output = T;
}

/// Alias `!` as `Never`.
type Never = <fn() -> ! as Bang>::Output;

/// Tell the compiler there is an external static of the `!` type.
extern {
    static NEVER: Never;
}

/// Informs the compiler that this point in the code is not reachable, enabling
/// further optimizations.
/// 
/// # Safety
/// 
/// Reaching the function is completely *undefined behaviour* (UB). In
/// particular, the compiler assumes that all UB must never happen,
/// and therefore will eliminate all branches that reach to a call to
/// `unreachable_unchecked()`.
/// 
/// 
/// 
/// # Example
/// ```
/// use unreachable::unreachable_unchecked;
/// 
/// fn div(a: u32, b: u32) -> u32 {
///     a.checked_div(b.saturating_add(1))
///         .unwrap_or_else(|| unsafe { unreachable_unchecked() })
/// }
/// 
/// assert_eq!(div(7, 0), 7);
/// assert_eq!(div(9, 1), 4);
/// ```
#[inline]
pub unsafe fn unreachable_unchecked() -> ! {
    NEVER
}

#[cfg(test)]
mod tests {
    use super::unreachable_unchecked;

    #[test]
    fn it_works() {
        let option = Some(10u8);

        let value = match option {
            Some(v) => v,
            None => unsafe { unreachable_unchecked() }
        };

        assert_eq!(value, 10);
    }
}