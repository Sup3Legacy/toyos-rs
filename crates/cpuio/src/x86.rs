//! Rust wrappers around the x86-family I/O instructions.

use core::arch::asm;

/// Read a `u8`-sized value from `port`.
pub unsafe fn inb(port: u16) -> u8 {
    // The registers for the `in` and `out` instructions are always the
    // same: `a` for value, and `d` for the port address.
    let result: u8;
    asm!("inb dx, al", in("dx") port, out("al") result);
    result
}

/// Write a `u8`-sized `value` to `port`.
pub unsafe fn outb(value: u8, port: u16) {
    asm!("outb al, dx", in("dx") port, in("al") value);
}

/// Read a `u16`-sized value from `port`.
pub unsafe fn inw(port: u16) -> u16 {
    let result: u16;
    asm!("inw dx, ax", out("ax") result, in("dx") port);
    result
}

/// Write a `u8`-sized `value` to `port`.
pub unsafe fn outw(value: u16, port: u16) {
    asm!("outw ax, dx", in("dx") port, in("ax") value);
}

/// Read a `u32`-sized value from `port`.
pub unsafe fn inl(port: u16) -> u32 {
    let result: u32;
    asm!("inl dx, eax", out("eax") result, in("dx") port);
    result
}

/// Write a `u32`-sized `value` to `port`.
pub unsafe fn outl(value: u32, port: u16) {
    asm!("outl eax, dx", in("dx") port, in("eax") value);
}
