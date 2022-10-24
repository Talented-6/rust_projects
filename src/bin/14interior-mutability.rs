use std::cell::Cell; // use cell to achieve the effect of partial mutability
use std::cell::RefCell;
use std::sync::Mutex;
// use std::mem::drop;
// use core::mem::drop;

struct PhoneModel {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    on_sale: Cell<bool>,
}
#[derive(Debug)]
struct User {
    id: u32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>,
    // Many other fields
}

fn main() {
    let super_phone_3000 = PhoneModel {
        company_name: "YY Electronics".to_string(),
        model_name: "Super Phone 3000".to_string(),
        screen_size: 7.5,
        memory: 4_000_000,
        date_issued: 2020,
        on_sale: Cell::new(true),
    };
    super_phone_3000.on_sale.set(false);

    // There are many methods for RefCell. Two of them are .borrow() and .borrow_mut(). With these methods, you can do the same thing you do with & and &mut. The rules are the same:
    // Many borrows is fine,
    // one mutable borrow is fine,
    // but mutable and immutable together is not fine.
    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };
    println!("{:?}", user_1.active);
    let date = 2022;
    user_1.active.replace_with(|_| date < 2000);
    // .replace_with(|_| if date < 2000 { true } else { false });

    //Mutual Exclusion
    let my_mutex = Mutex::new(5); // A new Mutex<i32>. We don't need to say mut
    let mut mutex_changer = my_mutex.lock().unwrap(); // mutex_changer is a MutexGuard
    // It has to be mut because we will change it
    // Now it has access to the Mutex
    // Let's print my_mutex to see:
    println!("{:?}", my_mutex); // This prints "Mutex { data: <locked> }"
    // So we can't access the data with my_mutex now,
    // only with mutex_changer
    println!("{:?}", mutex_changer); // This prints 5. Let's change it to 6.
    *mutex_changer = 6; // mutex_changer is a MutexGuard<i32> so we use * to change the i32
    println!("{:?}", mutex_changer); // Now it says 6
    drop(mutex_changer); // end the Mutex lock
    println!("{:?}", my_mutex);

    // double mutex_changer
    let my_mutex = Mutex::new(5);
    let mut mutex_changer = my_mutex.lock().unwrap();
    let mut other_mutex_changer = my_mutex.try_lock(); // try to get the lock
    if let Ok(value) = other_mutex_changer {
        println!("The MutexGuard has: {}", value)
    } else {
        println!("Didn't get the lock")
    }

}
