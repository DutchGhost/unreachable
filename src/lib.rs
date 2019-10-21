#![no_std]
//! This crate provides a user implementation of the `unreachable_unchecked()` function.
//! See [unreachable_unchecked](fn.unreachable_unchecked.html).

// An empty enum.
// This enum is used in an external static, to trick the compiler in believing there
// actually exists a value of this type. However, this enum has no variants, and thus
// no value. The faked value is later matched on in the __unreachable_unchecked function,
// which allows the return type of that function to be `!`
enum Void {}

// Tell the compiler there is an external static ___VOID___ pointer to a `Void` type.
// The symbol `___VOID___` is redefined in the internals module.
//
// This should be future proof against linker errors, because the symbol actual exists.
extern "C" {
    // Using `*const` here to allow potential NULL.
    static ___VOID___: *const Void;
}

#[doc(hidden)]
pub mod internals {
    use super::Void;

    // The type used for the internal `___VOID___`. This is a wrapper around a *const *const Void,
    // used to be able to implement Sync. Raw pointers don't implement Sync, and thus are not usable
    // in a static.
    //
    // A wrapper around a raw pointer however can manually
    // implement Sync, and thus make it usable in a static.
    #[repr(C)]
    pub struct InternalVoid {
        inner: *const *const Void,
    }

    // Only implemented to allow this type to be used in a static.
    unsafe impl Sync for InternalVoid {}

    // Redefine `___VOID___`. This is what the linker will link against.
    #[allow(unused_unsafe)]
    #[no_mangle]
    pub static ___VOID___: InternalVoid = unsafe {
        InternalVoid {
            inner: &super::___VOID___ as *const _,
        }
    };

    #[doc(hidden)]
    pub unsafe fn __unreachable_unchecked() -> ! {
        match **___VOID___.inner {}
    }
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
    internals::__unreachable_unchecked()
}

#[cfg(test)]
mod tests {
    use super::unreachable_unchecked;

    #[test]
    fn it_works() {
        let option = Some(10u8);

        let value = match option {
            Some(v) => v,
            None => unsafe { unreachable_unchecked() },
        };

        assert_eq!(value, 10);
    }
}
