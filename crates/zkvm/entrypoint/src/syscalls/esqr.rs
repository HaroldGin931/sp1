#[cfg(target_os = "zkvm")]
use core::arch::asm;

/// Executes the extend SQR operation on the given word array.
/// deliverd from sha_extend.rs
///
/// ### Safety
///
/// The caller must ensure that `w` is valid pointer to data that is aligned along a four byte
/// boundary.
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn syscall_sqr_extend(w: *mut [u32; 1]) {
    #[cfg(target_os = "zkvm")]
    unsafe {
        asm!(
            "ecall",
            in("t0") crate::syscalls::SQR_EXTEND,
            in("a0") w,
            in("a1") 0
        );
    }

    #[cfg(not(target_os = "zkvm"))]
    unreachable!()
}
