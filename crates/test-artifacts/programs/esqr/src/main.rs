#![no_main]

// Deliverd from fibonacci-program-tests
sp1_zkvm::entrypoint!(main);

use sp1_zkvm::syscalls::syscall_esqr;

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a system call which handles reading inputs
    // from the prover.
    let mut e: u32 = 99;
    let result = syscall_esqr(&mut e as *mut u32 as *mut [u32; 1]);
}
