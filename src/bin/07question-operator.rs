// fn parse_str(input:&str)->Result<i32,std::num::ParseIntError>{
//     // if it returns Err, it will return directly
//     let parse_number=input.parse::<i32>()?;
//     Ok(parse_number)
// }

// ? operator
fn parse_str(input: &str) -> Result<i32, std::num::ParseIntError> {
    //Todo understand this conversion
    let parsed_number = input
        .parse::<u16>()?
        .to_string()
        .parse::<u32>()?
        .to_string()
        .parse::<i32>()?; // Add a ? each time to check and pass it on
    Ok(parsed_number)
}

// unwrap
fn get_fourth(input: &Vec<i32>) -> i32 {
    let fourth = input.get(3).unwrap_or(&99);
    *fourth
}

fn main() {
    let str_vec = vec!["Seven", "8", "9.0", "nice", "6060"];
    for item in str_vec {
        let parsed = parse_str(item);
        println!("{:?}", parsed);
    }
    let my_vec = vec![9, 0, 10];
    let fourth = get_fourth(&my_vec);
}
