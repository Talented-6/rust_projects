use std::path::Components;

trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        let a = format!("u8: {}", *self);
        println!("{}", a);
        a
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        let a = format!("f64: {}", *self);
        println!("{}", a);
        a
    }
}

fn draw1(x: Box<dyn Draw>) -> String {
    x.draw()
}
fn draw2(x: &dyn Draw) -> String {
    x.draw()
}
fn main() {
    let x = 1.1;
    let y = 8u8;
    // dyn 关键字只用在特征对象的类型声明上，在创建时无需使用 dyn
    // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw>
    // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
    dbg!(draw1(Box::new(x)));
    dbg!(draw2(&y));
    let screen = Screen {
        components: vec![1.1, 2.2],
    };
    dbg!(screen.run())
}

// pub struct Screen {
//     components: Vec<Box<dyn Draw>>,
// }

// impl Screen {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

struct Screen<T: Draw> {
    components: Vec<T>,
}
impl<T> Screen<T>
where
    T: Draw,
{
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
