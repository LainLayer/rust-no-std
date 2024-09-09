#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(target_arch="aarch64")]
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

#[cfg(target_arch="x86_64")]
fn exit(exit_code: usize) {
    unsafe {
        asm! (
            "mov rdi, {exit_code}",
            "mov rax, 60",
            "syscall",
            exit_code = in(reg) exit_code
        );
    }
}


#[cfg(target_arch="aarch64")]
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

#[cfg(target_arch="x86_64")]
fn print(text: &str) {
    let ptr = text.as_ptr();
    let size: usize = text.len();
    unsafe {
        asm! (
            "mov rdi, 1",
            "mov rsi, {ptr}",
            "mov rdx, {size}",
            "mov rax, 1",
            "syscall",
            ptr = in(reg) ptr,
            size = in(reg) size
        );
    }
}

#[no_mangle]
unsafe fn memset(dest: *mut u8, c: u8, n: usize) {
    for i in 0..n {
        *dest.wrapping_add(i.into()) = c;
    }
}

#[cfg(target_arch="aarch64")]
fn input(buffer: &mut [u8; 1024]) -> &str {
    let ptr = (*buffer).as_mut_ptr();

    unsafe {
        asm!(
            "mov x0, 0",
            "mov x1, {buf}",
            "mov x2, 1024",
            "mov w8, 63",
            "svc 0",
            buf = in(reg) ptr
        );

        return core::str::from_utf8_unchecked(buffer);
    }
}

#[cfg(target_arch="x86_64")]
fn input(buffer: &mut [u8; 1024]) -> &str {
    let ptr = (*buffer).as_mut_ptr();

    unsafe {
        asm!(
            "mov rdi, 0",
            "mov rsi, {buf}",
            "mov rdx, 1024",
            "mov rax, 0",
            "syscall",
            buf = in(reg) ptr
        );

        return core::str::from_utf8_unchecked(buffer);
    }
}

#[no_mangle]
pub extern fn _start() {
    let mut buffer = [0u8; 1024];
    let text = input(&mut buffer);
    print(text);
    exit(0);
}
