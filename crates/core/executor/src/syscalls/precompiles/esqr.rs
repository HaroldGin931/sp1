use crate::{
    events::{EsqrEvent, PrecompileEvent},
    syscalls::{Syscall, SyscallCode, SyscallContext},
};

pub(crate) struct EsqrSyscall;

impl Syscall for EsqrSyscall {
    fn execute(
        &self,
        rt: &mut SyscallContext,
        syscall_code: SyscallCode,
        arg1: u32,
        arg2: u32,
    ) -> Option<u32> {
        let clk = rt.clk;

        let x_ptr = arg1;
        if x_ptr % 4 != 0 {
            panic!();
        }
        let y_ptr = arg2;
        if y_ptr % 4 != 0 {
            panic!();
        }

        // First read the words for the x value. We can read a slice_unsafe here because we write
        // the computed result to x later.
        // let x = rt.slice_unsafe(x_ptr, WORDS_FIELD_ELEMENT);
        // println!("!! {WORDS_FIELD_ELEMENT:?}");
        // In the original code, the x value is a 256-bit number, so we need to read 8 words. but here x is a 32-bit number, so we only need to read 1 word.
        // Also, riscv is aligned to 32-bit, so we can use 1 as the number of words to read.
        let x = rt.slice_unsafe(x_ptr, 1);

        // Read the y value.
        // Actually, y is not used in this precompile. but I don't know how to handle this in the original code.
        let (y_memory_records, y) = rt.mr_slice(y_ptr, 1);

        // Get the BigUint values for x, y, and the modulus.
        let u32_x = x[0];
        let u32_y = y[0];
        assert!(u32_y == 0);
        let modulus = 4_294_967_295; // (1 << 32) - 1
        let result = [(u32_x * u32_x) % modulus];

        // Increment clk so that the write is not at the same cycle as the read.
        rt.clk += 1;
        // Write the result to x and keep track of the memory records.
        let x_memory_records = rt.mw_slice(x_ptr, &result);

        let lookup_id = rt.syscall_lookup_id;
        let shard = rt.current_shard();
        let event = PrecompileEvent::Esqr(EsqrEvent {
            lookup_id,
            shard,
            clk,
            x_ptr,
            x,
            y_ptr,
            y,
            modulus,
            x_memory_records,
            y_memory_records,
            local_mem_access: rt.postprocess(),
        });
        let sycall_event =
            rt.rt.syscall_event(clk, syscall_code.syscall_id(), arg1, arg2, lookup_id);
        rt.add_precompile_event(syscall_code, sycall_event, event);

        Some(result[0])
    }

    fn num_extra_cycles(&self) -> u32 {
        1
    }
}
