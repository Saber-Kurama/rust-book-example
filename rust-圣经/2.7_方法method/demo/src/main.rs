// struct Circle {
//     x: f64,
//     y: f64,
//     radius: f64,
// }

// impl Circle {
//     // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字
//     // 这种方法往往用于初始化当前结构体的实例
//     fn new(x: f64, y: f64, radius: f64) -> Circle {
//         Circle { x, y, radius }
//     }

//     // Circle的方法，&self表示借用当前的Circle结构体
//     fn area(&self) -> f64 {
//         std::f64::consts::PI * (self.radius * self.radius)
//     }
// }

// fn main() {
//     println!("Hello, world!");
//     let c = Circle::new(1.0, 3.0, 5.0);

//     println!("面积：{}", c.area())
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//     // fn name(self) -> () {}
// }

// fn main() {
//     let react1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     println!("面积：{}", react1.area())
// }

// pub struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     pub fn new(width: u32, height: u32) -> Self {
//         Rectangle { width, height }
//     }
//     pub fn width(&self) -> u32 {
//         self.width
//     }
// }

// fn main() {
//     let rect1 = Rectangle::new(10, 10);

//     println!("width: {}", rect1.width())
// }

// pub struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     let rect2 = Rectangle {
//         width: 10,
//         height: 40,
//     };
//     let rect3 = Rectangle {
//         width: 60,
//         height: 45,
//     };

//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
// }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 方法
    }
}

fn main() {
    let m = Message::Write(String::from("Hello"));
    m.call();
}
