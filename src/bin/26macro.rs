#[macro_export]
macro_rules! test {
    ($($x:expr),*) => {{
        // let temp_vec = vec![..];
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }};
}
fn main() {
    println!("{:#?}", test!(9, 4, 3, 4, 5, 8));
}
