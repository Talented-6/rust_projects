fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    println!("{}", add(7, 9));
    forever();
}

fn forever() -> ! {
    // unimplemented!("你好");
    // todo!("你好");
    loop {}
}
