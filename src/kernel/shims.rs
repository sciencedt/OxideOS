// src/kernel/shims.rs
// Tiny C-ABI shims so core can link on bare metal.

#[unsafe(no_mangle)]
pub extern "C" fn memcmp(a: *const u8, b: *const u8, len: usize) -> i32 {
    for i in 0..len {
        let ai = unsafe { *a.add(i) };
        let bi = unsafe { *b.add(i) };
        if ai != bi {
            return (ai as i32) - (bi as i32);
        }
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn memcpy(dst: *mut u8, src: *const u8, len: usize) -> *mut u8 {
    for i in 0..len {
        unsafe { *dst.add(i) = *src.add(i) };
    }
    dst
}

#[unsafe(no_mangle)]
pub extern "C" fn memmove(dst: *mut u8, src: *const u8, len: usize) -> *mut u8 {
    // handle overlap
    if (dst as usize) <= (src as usize) || (dst as usize) >= (src as usize + len) {
        return memcpy(dst, src, len);
    }
    for i in (0..len).rev() {
        unsafe { *dst.add(i) = *src.add(i) };
    }
    dst
}

#[unsafe(no_mangle)]
pub extern "C" fn memset(dst: *mut u8, val: i32, len: usize) -> *mut u8 {
    let byte = (val & 0xFF) as u8;
    for i in 0..len {
        unsafe { *dst.add(i) = byte };
    }
    dst
}
