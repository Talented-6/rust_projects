use std::cell::{Cell, RefCell};
type CharacterVec = Vec<char>;
type SkipFourTakeFive<'a> = std::iter::Take<std::iter::Skip<std::slice::Iter<'a, char>>>;

fn returns<'a>(input: &'a CharacterVec) -> SkipFourTakeFive {
    input.iter().skip(4).take(5)
}

fn main() {
    println!("Hello, world!")
}
