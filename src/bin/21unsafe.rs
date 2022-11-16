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
}
