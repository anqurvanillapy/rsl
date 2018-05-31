#![feature(alloc, allocator_api, heap_api)]

use alloc::allocator::Alloc;
use alloc::allocator::Layout;
use std::{mem, ptr};

/**
 *  Bad alloc
 *  =========
 *
 *  > "Please welcome to the stage, OOM killer!!"
 */

fn f0() {
    unsafe {
        let siz = 512 * 1024 * 1024;
        let layout = Layout::from_size_align_unchecked(siz, 1024);
        let mut raw: *mut i32 =
            mem::transmute(Alloc::alloc(&mut std::heap::Global, layout));
        for i in 0..(siz / 4) {
            ptr::write(raw, i as i32);
            raw = raw.offset(1);
        }
    }
}

fn f1() {
    let mut vec = Vec::new();
    for _ in 0..512 * 1024 * 1024 {
        vec.push(Box::new(42));
    }
}

fn main() {
    f0();
    f1();
}
