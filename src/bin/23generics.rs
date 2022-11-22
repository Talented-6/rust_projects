fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// fn max(ls: &(impl PartialOrd + Copy)) -> impl PartialOrd + Copy {}
// fn max<T: PartialOrd + Copy>(ls: &[T]) -> &T {
//     let mut max_index: usize = 0;
//     for (index, item) in ls.iter().enumerate() {
//         if item > &ls[max_index as usize] {
//             max_index = index;
//         }
//     }
//     &ls[max_index]
// }
fn max<T: PartialOrd + Copy>(ls: &[T]) -> &T {
    let mut max = &ls[0];
    for i in ls.iter() {
        if i > max {
            max = i;
        }
    }
    max
}

fn main() {
    println!("{}", add(7, 9));
    println!("{}", max(&[99, 44, 333, 55]));
    // forever();
}

fn forever() -> ! {
    // unimplemented!("你好");
    // todo!("你好");
    loop {}
}
