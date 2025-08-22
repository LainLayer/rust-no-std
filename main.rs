#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    exit(-2)
}

#[cfg(target_arch="aarch64")]
fn exit(exit_code: i32) -> ! {
    unsafe {
        asm! (
            "svc 0",
            in("x0") exit_code,
            in("w8") 93,
            options(noreturn, nomem, nostack, preserves_flags)
        );
    }
}

#[cfg(target_arch="x86_64")]
fn exit(exit_code: i32) -> ! {
    unsafe {
        asm! (
            "syscall",
            in("rax") 60,
            in("rdi") exit_code,
            options(noreturn, nomem, nostack, preserves_flags)
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
            "mov w8, 64",
            "svc 0",
            in("x1") ptr,
            in("x2") size
        );
    }
}

#[cfg(target_arch="x86_64")]
fn print(text: &str) {
    let ptr = text.as_ptr();
    let size: usize = text.len();
    unsafe {
        asm! (
            "syscall",
            inlateout("rax") 1 => _,
            in("rsi") ptr,
            in("rdx") size,
            in("rdi") 1,
            out("rcx") _,
            out("r11") _,
            options(nostack, readonly, preserves_flags),
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

        // TODO: strip off newlines in aarch64 (see x86_64 implementation)
        return core::str::from_utf8_unchecked(buffer);
    }
}

#[cfg(target_arch="x86_64")]
fn input(buffer: &mut [u8]) -> &str {
    let ptr = buffer.as_mut_ptr();
    let len = buffer.len();
    let mut n: isize;

    unsafe {
        asm!(
            "syscall",
            inlateout("rax") 0usize => n,
            in("rdi") 0,
            in("rsi") ptr,
            in("rdx") len,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );

        if n < 0 {
            exit(-1);
        }

        return core::str::from_utf8_unchecked(buffer.get_unchecked(0..(n as usize))).trim_ascii_end();
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut buffer = [0u8; 1024];
    print("What is your name? ");
    let name = input(&mut buffer);
    print("Hello, ");
    print(name);
    print("!");
    exit(0)
}
