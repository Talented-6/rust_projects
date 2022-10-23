// # unit struct
struct FileDirectory;

// # tuple struct
struct Color(u8, u8, u8);

// # named struct
struct SizeAndColor {
    size: u32,
    color: Color,
}

enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiant = 1000,
    DeadStar, // Think about this one. What number will it have?
}
// # enum with different types of data
enum Number {
    U32(u32),
    I32(i32),
}

fn get_number(input: i32) -> Number {
    let number = match input.is_positive() {
        true => Number::U32(input as u32), // change it to u32 if it's positive
        false => Number::I32(input), // otherwise just give the number because it's already i32
    };
    number
}

fn main() {
    // no contents
    use Star::*;
    let starvec = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant];
    for star in starvec {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star."), // Remember: size doesn't mean anything. It's just a name we chose so we can print it
            size if size >= 80 => println!("This is a good-sized star."),
            _ => println!("That star is pretty big!"),
        }
    }
    println!("What about DeadStar? It's the number {}.", DeadStar as u32);
    // contents
    let my_vec = vec![get_number(-800), get_number(8)];
    for item in my_vec {
        match item {
            Number::U32(number) => println!("It's a u32 with the value {}", number),
            Number::I32(number) => println!("It's an i32 with the value {}", number),
        }
    }
}
