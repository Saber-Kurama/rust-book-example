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

// 变量覆盖

fn main() {
    let age = Some(30);
    println!("在匹配前，age是{:?}", age);
    if let Some(age) = age {
        println!("匹配出来的age是{}", age);
    }
    println!("在匹配后，age是{:?}", age);
}
