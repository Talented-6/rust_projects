fn match_colours(rbg: (i32, i32, i32)) {
    println!("Comparing a colour with {} red, {} blue, and {} green:", rbg.0, rbg.1, rbg.2);
    let new_vec = vec![(rbg.0, "red"), (rbg.1, "blue"), (rbg.2, "green")]; // Put the colours in a vec. Inside are tuples with the colour names
    let mut all_have_at_least_10 = true; // Start with true. We will set it to false if one colour is less than 10
    for item in new_vec {
        if item.0 < 10 {
            all_have_at_least_10 = false; // Now it's false
            println!("Not much {}.", item.1) // And we print the colour name.
        }
    }
    if all_have_at_least_10 { // Check if it's still true, and print if true
        println!("Each colour has at least 10.")
    }
    println!(); // Add one more line
}


fn main() {
    // tags of loop
    let mut counter = 0;
    let mut counter2 = 0;
    println!("Now entering the first loop.");
    'first_loop: loop {
        // Give the first loop a name
        counter += 1;
        println!("The counter is now: {}", counter);
        if counter > 9 {
            // Starts a second loop inside this loop
            println!("Now entering the second loop.");
            'second_loop: loop {
                // now we are inside 'second_loop
                println!("The second counter is now: {}", counter2);
                counter2 += 1;
                if counter2 == 3 {
                    break 'first_loop; // Break out of 'first_loop so we can exit the program
                }
            }
        }
    }
    // match colors
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_colours(first);
    match_colours(second);
    match_colours(third);
}