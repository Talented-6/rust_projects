// # unit struct
struct FileDirectory;

// # tuple struct
struct Color(u8, u8, u8);

// # named struct
struct SizeAndColor {
    size: u32,
    color: Color,
}

#[derive(Debug, PartialEq)]
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
    TEST(Star),
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
    let my_vec = vec![get_number(-800), get_number(8), Number::TEST(DeadStar)];
    for item in my_vec {
        match item {
            Number::U32(number) => println!("It's a u32 with the value {}", number),
            Number::I32(number) => println!("It's an i32 with the value {}", number),
            // need to derive PartialEq
            // Number::TEST(star) if star == DeadStar => println!("It's an {:?}", star),
            Number::TEST(star) if matches!(star, DeadStar) => println!("It's an {:?}", star),
            _ => (),
        }
    }
    // test
    let array = [String::from("666"), String::from("999")];
    let array: [String; 6] = std::array::from_fn(|_| String::from("Rust is good"));
    println!("{:?}", array);
    let i = get_index();
    println!("{}", array[i]);
    // sugar to taste
    let mut s = 0;
    for i in 1..=100 {
        s += i;
    }
    println!("{}", s);
    // @ binding
    // let x @ SizeAndColor { size, color } = SizeAndColor {
    //     size: 9,
    //     color: Color(1, 1, 1),
    // };
    match 1 {
        num @ (1 | 2) => {
            println!("{}", num);
        }
        _ => (),
    }
    if let num @ (1 | 2) = 2 {
        println!("{}", num);
    };
}

fn get_index() -> usize {
    let mut index = String::new();
    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read the input");
    let index: usize = index.trim().parse().expect("Not a valid number");
    index
}
