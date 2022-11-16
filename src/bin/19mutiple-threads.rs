// Great! I have completed the rewriting using Option and HashMap.
use std::collections::HashMap;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<HashMap<u32, u32>>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match &self.value {
            Some(v) => *(v.clone().entry(arg).or_insert((self.calculation)(arg))),
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(HashMap::new());
                self.value.as_mut().unwrap().insert(arg, v);
                v
            }
        }
    }
}

fn main() {
    let handle0 = thread::spawn(|| {
        for i in 1..=10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // move closure
    let my_string = String::from("Can I go inside the thread?");
    let handle = std::thread::spawn(move || {
        println!("{}", my_string);
    });
    handle0.join().unwrap();
    handle.join().unwrap();

    // message passing
    let (tx, rx) = std::sync::mpsc::channel(); // mutiple producer, single consumer
    let tx1 = tx.clone();
    thread::spawn(move || {
        let val = String::from("1 hi");
        tx.send(val).unwrap();
    });
    // let received= rx.recv().unwrap(); // recv: receive
    // println!("Got: {}", received);
    thread::spawn(move || {
        let vals = vec![
            String::from("2 hi"),
            String::from("2 from"),
            String::from("2 the"),
            String::from("2 thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }
    // 共享内存实现并发(mutual exclusion互斥锁)
    let m = Mutex::new(5);
    // 智能指针自动析构时解锁
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    generate_workout(110, 33);
}

fn generate_workout(intensity: u32, random_number: u32) {
    // 闭包并不强制要求捕获其环境，但是如果想要在闭包中使用其环境的值，就必须要求其捕获其环境
    // 闭包并不强制写明参数和返回值的类型
    // 闭包可省略花括号
    // let add_one_v4 = |x| x + 1;

    // let expensive_result = |num| -> u32 {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!(
            "Today, run for {} minutes!",
            expensive_result.value(intensity)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let _v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}
