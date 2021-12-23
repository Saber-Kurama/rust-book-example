/*
 * @Author: saber
 * @Date: 2021-12-23 11:21:27
 * @LastEditTime: 2021-12-23 21:00:43
 * @LastEditors: saber
 * @Description: 
 */
use phrases_lib::chinese::greetings as cn_greetings;
use phrases_lib::chinese::farewells as cn_farewells;
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
  println!("greetings in chinese: {}", cn_greetings::hello());
  println!("farewells in chinese: {}", cn_farewells::goodbye());
  println!("greetings in englist: {}", greetings::hello());
//  println!("hello in chinese: {}", phrases_lib::chinese::greetings::hello()) 
}
