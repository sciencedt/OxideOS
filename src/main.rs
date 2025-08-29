#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod kernel;

use kernel::{console::Console, /* keyboard::read_scancode_nonblock */};
// use kernel::scancode::{decode_scancode, DecodedKey};
use kernel::cli;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut con = Console::new(0x1F);
    con.clear();
    // after clear:
    kernel::vga::disable_cursor();
    // NEW: make the hardware cursor visible and sync to (0,0)
    con.enable_cursor();
    con.sync_cursor();
    con.write_str("OxideOS!: Type on your keyboard...");
    con.sync_cursor();

    // hand over control (never returns)
    cli::repl(&mut con);
    // loop {
    //     if let Some(sc) = read_scancode_nonblock() {
    //         match decode_scancode(sc) {
    //             DecodedKey::Ascii(b) => con.putc(b),
    //             DecodedKey::Enter    => con.newline(),
    //             DecodedKey::Backspace=> con.backspace(),
    //             DecodedKey::None     => { /* ignore */ }
    //         }
    //     }
    // }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop { unsafe { core::arch::asm!("hlt"); } }
}

