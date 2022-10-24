use std::collections::HashMap;
fn main() {
    let my_vec = vec![8, 9, 10];
    let fourth = my_vec.get(3).unwrap_or_else(|| {
        // try to unwrap. If it doesn't work,
        if my_vec.get(0).is_some() {
            // see if my_vec has something at index [0]
            &my_vec[0] // Give the number at index 0 if there is something
        } else {
            &0 // otherwise give a &0
        }
    });
    println!("{}", fourth);
    // HashMap
    let some_numbers = vec![0, 1, 2, 3, 4, 5]; // a Vec<i32>
    let some_words = vec!["zero", "one", "two", "three", "four", "five"]; // a Vec<&str>

    let number_word_hashmap: HashMap<_, _> = some_numbers
        .into_iter() // now it is an iter
        .zip(some_words.into_iter()) // inside .zip() we put in the other iter. Now they are together.
        .collect();

    println!(
        "For key {} we get {}.",
        2,
        number_word_hashmap.get(&2).unwrap()
    );
    // .filter
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let filtered_months: Vec<&str> = months
        .into_iter()
        .filter(|month| month.len() < 5)
        .filter(|month| month.contains("u"))
        .collect();
    println!("{:?}", filtered_months);
    // .cycle
    let even_odd = vec!["even", "odd"];
    let even_odd_vec: Vec<(_, _)> = (0..6).zip(even_odd.into_iter().cycle()).collect();
    println!("{:?}", even_odd_vec);
    // .fold
    let a_string = "I don't have any dashes in me.";
    println!(
        "{}",
        a_string
            .chars() // Now it's an iterator
            .fold(
                "-".to_string(),
                |mut string_so_far, next_char| {
                    // Start with a String "-". Bring it in as mutable each time along with the next char
                    string_so_far.push(next_char); // Push the char on, then '-'
                    string_so_far.push('-');
                    string_so_far
                } // Don't forget to pass it on to the next loop
            )
    );
    // .chunks .windows
    let num_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    for chunk in num_vec.chunks(3) {
        println!("{:?}", chunk);
    }
    println!();
    for window in num_vec.windows(3) {
        println!("{:?}", window);
    }
    // .match_indicates
    let rules = "Rule number 1: No fighting. Rule number 2: Go to bed at 8 pm. Rule number 3: Wake up at 6 am.";
    let rule_locations = rules.match_indices("Rule").collect::<Vec<(_, _)>>(); // This is Vec<usize, &str> but we just tell Rust to do it
    println!("{:?}", rule_locations);
    // .peek and .next
    let locations = vec![
        ("Nevis", 25),
        ("Taber", 8428),
        ("Markerville", 45),
        ("Cardston", 3585),
    ];
    let mut location_iter = locations.iter().peekable();
    while location_iter.peek().is_some() {
        match location_iter.peek() {
            Some((name, number)) if *number < 100 => {
                // .peek() gives us a reference so we need *
                println!("Found a hamlet: {} with {} people", name, number)
            }
            Some((name, number)) => println!("Found a town: {} with {} people", name, number),
            None => break,
        }
        location_iter.next();
    }
}
