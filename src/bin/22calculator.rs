use std::env::args;

fn main() {
    let mut args = args();
    let first = args.nth(1).unwrap();
    let operator: char = args.next().unwrap().chars().next().unwrap();
    let second = args.next().unwrap();
    let first_number: f32 = first.parse().unwrap();
    let second_number: f32 = second.parse::<f32>().unwrap();
    println!(
        "{} {} {} = {}",
        first,
        operator,
        second,
        operate(operator, first_number, second_number)
    );
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        '/' => first_number / second_number,
        _ => panic!("Unknown operator"),
    }
}
