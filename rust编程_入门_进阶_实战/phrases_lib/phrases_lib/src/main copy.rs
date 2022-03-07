/*
 * @Author: saber
 * @Date: 2021-12-23 11:21:27
 * @LastEditTime: 2021-12-23 21:14:36
 * @LastEditors: saber
 * @Description: 
 */
// 模块化编程
// crate 是一个Rust的基本单元
// 二进制 crate 和 库 crate
// use phrases_lib::chinese::greetings as cn_greetings;
// use phrases_lib::chinese::farewells as cn_farewells;
use phrases_lib::chinese;
use phrases_lib::english::{ greetings, farewells };
mod my_old {
  pub fn function() {
    println!("my_old function")
  }
}
fn function() {
  println!("function----")
}
fn main() {
  function();
  my_old::function();
  println!("greetings in chinese: {}", chinese::hello());
  println!("farewells in chinese: {}", chinese::goodbye());
  println!("greetings in englist: {}", greetings::hello());
//  println!("hello in chinese: {}", phrases_lib::chinese::greetings::hello()) 
}
