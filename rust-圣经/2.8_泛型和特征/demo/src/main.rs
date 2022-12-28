use std::{fmt::Display, ops::Add};

pub trait Summary {
    // fn summarize(&self) -> String;
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, p: Point<T>) -> Point<T> {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSE"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

pub trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

fn draw1(x: Box<dyn Draw>) -> String {
    x.draw()
}

fn draw2(x: &dyn Draw) -> String {
    x.draw()
}

fn main() {
    println!("Hello, world!");
    let post = Post {
        title: "Rust语言简介".to_string(),
        author: "saber".to_string(),
        content: "Rust 棒极了".to_string(),
    };
    let weibo = Weibo {
        username: "saber".to_string(),
        content: "微博发布了rust".to_string(),
    };
    println!("{}", post.summarize());
    println!("{}", weibo.summarize());

    let p1 = Point {
        x: 1.1f32,
        y: 1.1f32,
    };

    let p2 = Point {
        x: 2.1f32,
        y: 2.1f32,
    };

    println!("{:?}", p1 + p2);

    let f6 = File::new("f6.txt");
    println!("{}", f6);
    println!("{:?}", f6);

    let a: u8 = 1;
    let b = 2.0f64;
    println!("{}", draw1(Box::new(a)));
    println!("{}", draw1(Box::new(b)));
    println!("{}", draw2(&a));
    println!("{}", draw2(&b))
}
