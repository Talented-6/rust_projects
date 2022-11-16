// Cons List
use crate::List::{Cons, Nil};
use std::ops::Deref;

fn main() {
    //1. Box<T> smart pointer
    // Box指针可知道指针指向所占的内存空间，实现了Deref和Drop trait
    let _ = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    //2. Deref trait
    let x = 5;
    let y = &x;
    assert_eq!(x, *y);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(x, *y);

    let x = 5;
    let y = MyBox::new(5);
    assert_eq!(x, *y); // actually implictly unwarp the `*y` to `*(y.deref())`
    drop(y); // drop the MyBox<T> instance

    // Deref Coercion函数和方法的隐式解引用转化（当传参时引用与定义的参数类型不匹配）
    let m = MyBox::new(String::from("Rust"));
    // &m &MyBox<String>
    // deref &String &(*m)
    // deref &str &(*m)[..]
    hello(&m);
    //3. Rc<T> reference counting smart pointer（Rc只用于单线程的场景）（Rck通过不可变引用共享只读数据）（常常Rc<RefCell<T>>使用)
    //4. 解决循环引用
}

// tuple struct(actual named tuple)
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    // 关联类型
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("haha");
    }
}
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}
