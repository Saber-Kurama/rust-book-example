// /**
//  * match
//  */
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }

// fn main() {
//     let dire = Direction::South;
//     match dire {
//         Direction::East => {
//             println!("东方")
//         }
//         Direction::West | Direction::South => {
//             println!("西方白虎")
//         }
//         _ => println!("other"),
//     }
// }

// // 使用 match 表达式赋值

// enum IpAddr {
//     Ipv4,
//     Ipv6,
// }

// fn main() {
//     let ip1 = IpAddr::Ipv4;
//     let ip_str = match ip1 {
//         IpAddr::Ipv4 => "127.0.0.1",
//         _ => "::1",
//     };

//     println!("{}", ip_str)
// }

// // 模式绑定

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("sate 的值 {:?}!", state);
//             25
//         }
//     }
// }
// fn main() {
//     let coin1 = Coin::Quarter(UsState::Alabama);
//     value_in_cents(coin1);
// }

// // if let 匹配

// fn main() {
//     let v = Some(3u8);
//     if let Some(1) = v {
//         println!("匹配")
//     }
//     if let Some(3) = v {
//         println!("匹配")
//     }
//     if let Some(state) = v {
//         println!("匹配 {}", state)
//     }
//     let x = match v {
//         Some(state) => state,
//         _ => 1,
//     };

//     println!("{}", x)
// }

// matches! 宏

// #[derive(Debug)]
// enum MyEnum {
//     Foo,
//     Bar,
// }

// fn main() {
//     let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
//     // v.iter().filter(|x| x == MyEnum::Foo); // 无法将 x 和 枚举成员进行比较
//     let mut x = v.iter().filter(|x| matches!(x, MyEnum::Foo));
//     // let b = x.next();
//     println!("{:?}", x.next());
//     println!("{:?}", x.next());
//     println!("{:?}", x.next());

//     let foo = 'f';

//     assert!(matches!(foo, 'A'..='Z' |  'a' ..= 'z'));

//     let bar = Some(4);
//     assert!(matches!(bar, Some(x) if x > 2));
// }

// // 变量覆盖

// fn main() {
//     let age = Some(30);
//     println!("在匹配前，age是{:?}", age);
//     if let Some(age) = age {
//         println!("匹配出来的age是{}", age);
//     }
//     println!("在匹配后，age是{:?}", age);
// }

// // 解构 Option

// fn main() {
//     fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             None => None,
//             Some(i) => Some(i + 1),
//         }
//     }

//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
//     println!("{:?}", six);
//     println!("{:?}", none);
// }

// // while let 条件循环

// fn main() {
//     // 动态数组
//     let mut stack = Vec::new();

//     // 添加元素
//     stack.push(1);
//     stack.push(2);
//     stack.push(3);

//     while let Some(top) = stack.pop() {
//         print!("{}", top)
//     }
// }

// // for 循环
// fn main() {
//     let v = vec!['a', 'b', 'c'];
//     for (index, value) in v.iter().enumerate() {
//         println!("{} is at index {}", value, index)
//     }
// }

// // 单分支多模式

// fn main() {
//     let x = 1;
//     match x {
//         1 | 2 => println!("1和2"),
//         3 => println!("3"),
//         _ => println!("其他"),
//     }
// }

// // 通过序列 `..=` 匹配值的范围
// fn main() {
//     let x = 5;
//     match x {
//         1..=5 => println!("1到5"),
//         _ => println!("其他"),
//     }
// }

// // 解耦结构体

// fn main() {
//     struct Point {
//         x: i32,
//         y: i32,
//     }

//     let p = Point { x: 10, y: 20 };
//     let Point { x: a, y: b } = p;
//     println!("a is {}, b is {}", a, b);

//     match p {
//         Point { x, y: 21 } => println!("x 值是 {}", x),
//         Point { x: 10, y } => println!("y 值是 {}", y),
//         Point { x, y } => println!("x 值是  {}, y 值是 {}", x, y),
//     }
// }

// // 解构枚举

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// fn main() {
//     let msg = Message::ChangeColor(0, 160, 255);

//     match msg {
//         Message::Quit => {
//             println!("quit")
//         }
//         Message::Move { x, y } => {
//             println!("x is {}, y is {}", x, y)
//         }
//         Message::Write(text) => println!("text is {}", text),
//         Message::ChangeColor(r, g, b) => println!("r is {}, g is {}, b is {}", r, g, b),
//     }
// }

// // 解构嵌套的结构体和枚举

// enum Color {
//     Rgb(i32, i32, i32),
//     Hsv(i32, i32, i32),
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(Color),
// }

// fn main() {
//     let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

//     match msg {
//         Message::ChangeColor(Color::Rgb(r, g, b)) => println!("r is {}, g is {}, b is {}", r, g, b),
//         Message::ChangeColor(Color::Hsv(h, s, v)) => println!("h is {}, s is {}, v is {}", h, s, v),
//         _ => println!("其他"),
//     }
// }

// // 匹配守卫 额外条件

// fn main() {
//     let num = Some(4);
//     match num {
//         Some(x) if x < 5 => println!("{}小于5的", x),
//         Some(x) => println!("{}", x),
//         None => (),
//     }
// }

fn main() {
    let x = Some(10);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
}
