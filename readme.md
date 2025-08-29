# OxideOS (tiny Rust kernel + mini CLI)

A tiny no_std Rust OS that boots to VGA text mode and provides a minimal CLI.
Current commands:
```
help — show help
```
```
echo <message> — print a message
```

Designed to be small, readable, and easy to extend one command at a time.

## Features (current)

#![no_std], #![no_main] bare-metal kernel

Text console over VGA memory (0xB8000)

Keyboard scancode polling (PS/2)

Tiny CLI loop with fixed buffer, backspace editing, and simple prompt

No allocator or heap required

C-ABI shims (memcmp, memcpy, memmove, memset) so core links cleanly

Boots with bootimage; runs under QEMU

Project layout
src/
  main.rs                    // entry: sets up console and starts CLI
  kernel/
    mod.rs
    console.rs               // VGA text console
    vga.rs                   // VGA helpers (cursor, etc.)
    keyboard.rs              // scancode read (non-blocking)
    scancode.rs              // scancode decoder -> DecodedKey {Ascii, Enter, Backspace, None}
    cli.rs                   // tiny REPL with `help` + `echo`
    io.rs                    // in8/out8 port I/O
    shims.rs                 // memcmp/memcpy/memmove/memset
x86_64-oxideos.json          // target spec (custom)

Prerequisites

Nightly Rust

rust-src and llvm-tools-preview components

bootimage (for building bootable disk images)

QEMU

rustup default nightly
rustup component add rust-src llvm-tools-preview
cargo install bootimage
# If you use a custom target file:
rustup target add x86_64-unknown-none

Build & run
# Build a bootable image (uses core+alloc only; we provide C-ABI shims)
cargo bootimage --target x86_64-oxideos.json -Zbuild-std=core,alloc


Run in QEMU (basic):

qemu-system-x86_64 \
  -drive format=raw,file=target/x86_64-oxideos/debug/bootimage-OxideOs.bin


Run in QEMU with useful debug flags:

qemu-system-x86_64 \
  -drive format=raw,file=target/x86_64-oxideos/debug/bootimage-OxideOs.bin \
  -serial stdio \
  -no-reboot \
  -d int


Tip: -serial stdio shows the serial port in your terminal (we can wire kernel logs to COM1 later).
-no-reboot -d int helps catch exceptions instead of silently rebooting.

Keyboard & cursor notes (important)

Port I/O in long mode can #GP if your IOPL is low or your TSS I/O bitmap blocks ports.

If you see instant reboots/flicker, temporarily avoid port I/O to 0x3D4/0x3D5 (VGA cursor) and PS/2 ports (0x60/0x64), or raise IOPL + set up an IDT/TSS properly.

A safe first step is to disable cursor updates and keep to VGA memory writes only.
(You can later re-enable cursor + keyboard ports once IOPL/IDT/TSS are in place.)