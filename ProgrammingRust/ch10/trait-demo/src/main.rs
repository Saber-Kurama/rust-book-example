use std::io::Write;

// trait 对象
// fn say_hello(out: &mut dyn Write) -> Result<(), std::io::Error> {
//     out.write_all(b"hello world\n");
//     out.flush()
// }

// // 泛型函数和类型参数
// fn say_hello<W: Write>(out: &mut W) -> Result<(), std::io::Error>{
//     out.write_all(b"hello world\n");
//     out.flush()
// }

// fn top_ten<T: Debug + Hash + Eq>(values: T) {

// }

// 定义trait

struct  Canvas {}

struct Broom {}

// 定义一个 trait
trait Visible {
    fn draw(&self, canvas: &mut Canvas);
    fn hit_test(&self, x: i32, y: i32) -> bool;
}

// 实现一个trait
impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        todo!()
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        todo!()
    }
}

pub struct Sink;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

trait Creature: Visible {
    fn position(&self) -> (i32, i32);
    fn facing(&self) -> Direction;
}

trait Creature where Self: Visible {
    
}
fn main() {
    // let mut buf: Vec<u8> = vec![];
    // // let writer: &mut dyn Write = &mut buf;
    // say_hello(&mut buf);
    println!("Hello, world!");
}
