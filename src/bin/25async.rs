use std::sync::{Arc, Barrier};
use std::thread;
// use std::time::Duration;
use colored::*;

fn main() {
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));
    for i in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("Before wait {}", i.to_string().green().bold());
            b.wait();
            println!("After wait {}", i.to_string().red().bold());
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
