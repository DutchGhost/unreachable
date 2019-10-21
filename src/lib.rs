#![no_std]
//! This crate provides a user implementation of the `unreachable_unchecked()` function. See [unreachable_unchecked](fn.unreachable_unchecked.html).

/// An empty enum.
/// Any match on an instance of this is unreachable.
enum Void {}

/// Tell the compiler there is an external static of the `Void` type.
extern {
    static VOID: Void;
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
    match VOID {}
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