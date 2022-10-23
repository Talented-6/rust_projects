fn match_number(input: i32) {
    match input {
        number @ 4 => println!(
            "{} is an unlucky number in China (sounds close to 死)!",
            number
        ),
        number @ 13 => println!(
            "{} is unlucky in North America, lucky in Italy! In bocca al lupo!",
            number
        ),
        _ => println!("Looks like a normal number"),
    }
}
fn match_colours(rbg: (i32, i32, i32)) {
    match rbg {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, b, _) if b < 10 => println!("Not much blue"),
        (_, _, g) if g < 10 => println!("Not much green"),
        _ => println!("Each colour has at least 10"),
    }
}
fn main() {
    // # match
    // normal
    let my_number: u8 = 5;
    match my_number {
        0 => println!("it's zero"),
        1 => println!("it's one"),
        2 => println!("it's two"),
        _ => println!("It's some other number"),
    }
    // @
    match_number(50);
    match_number(13);
    match_number(4);
    // if
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_colours(first);
    match_colours(second);
    match_colours(third);
}
