unsafe fn dangerous() {
    // println!("Hello, world");
}
fn main() {
    let mut num = 5;
    // 原始指针只能在unsafe代码块中解引用
    let p1 = &num as *const i32;
    let p2 = &mut num as *mut i32;
    unsafe {
        println!("{}", *p1);
        println!("{}", *p2);
        dangerous();
    }

    let (pointer, length) = get_memory_location();
    let message = get_str_at_location(pointer, length);
    println!("The {} bytes at {:#x} store: {}", length, pointer, message);
}

use std::{slice::from_raw_parts, str::from_utf8_unchecked};

// 获取字符串的内存地址和长度
fn get_memory_location() -> (usize, usize) {
    let string = "Hello World!";
    let pointer = string.as_ptr() as usize;
    let length = string.len();
    (pointer, length)
}

// 在指定的内存地址读取字符串
fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length + 20)) }
}
