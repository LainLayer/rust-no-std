#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern fn _start() {
    let data: &str = "balls\n";
    let ptr = data.as_ptr();
    let size: u64 = 6;
    unsafe {
        asm! (
            "mov x0, 1",
            "mov x1, {ptr}",
            "mov x2, {size}",
            "mov w8, 64",
            "svc 0",
            ptr = in(reg) ptr,
            size = in(reg) size
        );
    }

    unsafe {
        asm! (
            "mov x0, 0",
            "mov w8, 93",
            "svc 0"
        );
    }
}
