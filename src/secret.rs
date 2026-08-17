use std::sync::atomic::{compiler_fence, Ordering};

pub(crate) fn clear_string(value: &mut String) {
    // SAFETY: every initialized byte is overwritten with zero, which remains
    // valid UTF-8, and the String length/capacity/allocation are unchanged.
    let bytes = unsafe { value.as_mut_vec() };
    for byte in bytes {
        // SAFETY: `byte` points into the uniquely borrowed String allocation.
        unsafe { std::ptr::write_volatile(byte, 0) };
    }
    compiler_fence(Ordering::SeqCst);
}
