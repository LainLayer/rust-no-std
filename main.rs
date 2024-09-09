#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn exit(exit_code: usize) {
    unsafe {
        asm! (
            "mov x0, {exit_code}",
            "mov w8, 93",
            "svc 0",
            exit_code = in(reg) exit_code
        );
    }
}

fn print(text: &str) {
    let ptr = text.as_ptr();
    let size: usize = text.len();
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
}

#[no_mangle]
pub extern fn _start() {
    print("Hello World\n");
    exit(0);
}
