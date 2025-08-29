// src/kernel/cli.rs
//! Tiny one-command CLI for OxideOS that matches your DecodedKey and Console API.
//! Commands: `help`, `echo <text>`

use crate::kernel::{
    console::Console,
    keyboard::read_scancode_nonblock,
    scancode::{decode_scancode, DecodedKey},
};

pub fn repl(con: &mut Console) -> ! {
    let mut buf = [0u8; 64]; // heap-free input buffer
    loop {
        prompt(con);
        let line = read_line(con, &mut buf);
        run_command(con, line);
    }
}

fn prompt(con: &mut Console) {
    con.write_str("> ");
    con.sync_cursor();
}

fn read_line<'a>(con: &mut Console, buf: &'a mut [u8]) -> &'a str {
    let mut len = 0usize;

    loop {
        if let Some(sc) = read_scancode_nonblock() {
            match decode_scancode(sc) {
                DecodedKey::Enter => {
                    con.newline();
                    return core::str::from_utf8(&buf[..len]).unwrap_or("");
                }
                DecodedKey::Backspace => {
                    if len > 0 {
                        len -= 1;
                        con.backspace();
                        con.sync_cursor();
                    }
                }
                DecodedKey::Ascii(b) => {
                    if (b >= 32 && b <= 126) || b == b' ' {
                        if len < buf.len() {
                            buf[len] = b;
                            len += 1;
                            con.putc(b);
                            con.sync_cursor();
                        }
                    }
                }
                DecodedKey::None => { /* ignore */ }
            }
        } else {
            // be nice to the CPU while idle
            unsafe { core::arch::asm!("hlt"); }
        }
    }
}

fn run_command(con: &mut Console, line: &str) {
    let line = line.trim();
    if line.is_empty() { return; }

    if line == "help" {
        con.write_str("Commands:\n");
        con.write_str("  help           - show this help\n");
        con.write_str("  echo <message> - print message\n");
        return;
    }

    if let Some(rest) = line.strip_prefix("echo ") {
        con.write_str(rest);
        con.newline();
        return;
    }

    con.write_str("unknown command: ");
    con.write_str(line);
    con.newline();
}
