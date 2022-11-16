use std::env::args;

fn main() {
    let mut args=args();
    let first=args.nth(1).unwrap();
    let second=args.nth(2).unwrap();
}