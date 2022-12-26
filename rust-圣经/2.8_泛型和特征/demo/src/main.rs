pub trait Summary {
    fn summarize(&self) -> String;
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
}
