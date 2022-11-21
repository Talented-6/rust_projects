use hello_macro::HelloMacro;

struct Sunfei;

impl HelloMacro for Sunfei {
    fn hello_macro() {
        println!("Hello, Macro! My name is Sunfei!");
    }
}

struct Sunface;

impl HelloMacro for Sunface {
    fn hello_macro() {
        println!("Hello, Macro! My name is Sunface!");
    }
}

fn main() {
    Sunfei::hello_macro();
}
