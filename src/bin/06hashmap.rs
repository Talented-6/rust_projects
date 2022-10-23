use std::collections::BinaryHeap;
use std::collections::HashMap;

fn show_remainder(input: &BinaryHeap<i32>) -> Vec<i32> {
    // This function shows the remainder in the BinaryHeap. Actually an iterator would be
    // faster than a function - we will learn them later.
    let mut remainder_vec = vec![];
    for number in input {
        remainder_vec.push(*number)
    }
    remainder_vec
}
fn main() {
    // get
    let mut book_hashmap = HashMap::new();

    book_hashmap.insert(1, "L'Allemagne Moderne");
    book_hashmap.insert(1, "Le Petit Prince");
    book_hashmap.insert(1, "섀도우 오브 유어 스마일");
    book_hashmap.insert(1, "Eye of the World");

    println!("{:?}", book_hashmap.get(&1));
    // entry
    //use HashMap but can mark the number of repeated items
    let book_collection = vec![
        "L'Allemagne Moderne",
        "Le Petit Prince",
        "Eye of the World",
        "Eye of the World",
    ];

    let mut book_hashmap = HashMap::new();

    for book in book_collection {
        let return_value = book_hashmap.entry(book).or_insert(0); // return_value is a mutable reference. If nothing is there, it will be 0
        *return_value += 1; // Now return_value is at least 1. And if there was another book, it will go up by 1
    }

    for (book, number) in book_hashmap {
        println!("{}, {}", book, number);
    }
    // another example
    let data = vec![
        // This is the raw data
        ("male", 9),
        ("female", 5),
        ("male", 0),
        ("female", 6),
        ("female", 5),
        ("male", 10),
    ];

    let mut survey_hash = HashMap::new();

    for item in data {
        // This gives a tuple of (&str, i32)
        survey_hash.entry(item.0).or_insert(Vec::new()).push(item.1); // This pushes the number into the Vec inside
    }

    for (male_or_female, numbers) in survey_hash {
        println!("{:?}: {:?}", male_or_female, numbers);
    }
    // binary map
    let many_numbers = vec![0, 5, 10, 15, 20, 25, 30]; // These numbers are in order
    let mut my_heap = BinaryHeap::new();
    for number in many_numbers {
        my_heap.push(number);
    }
    while let Some(number) = my_heap.pop() {
        // .pop() returns Some(number) if a number is there, None if not. It pops from the front
        println!(
            "Popped off {}. Remaining numbers are: {:?}",
            number, show_remainder(&my_heap)
        );
    }
}
