/*
 * @Author: saber
 * @Date: 2021-12-23 11:21:27
 * @LastEditTime: 2021-12-23 21:14:36
 * @LastEditors: saber
 * @Description: 
 */
// use phrases_lib::chinese::greetings as cn_greetings;
// use phrases_lib::chinese::farewells as cn_farewells;
use phrases_lib::chinese;
use phrases_lib::english::{ greetings, farewells };

mod frist;

mod my_old {
  pub fn function() {
    println!("my_old function")
  }
}
fn function() {
  println!("function----")
}
fn main() {
  frist::helloWorld();
  // function();
  // my_old::function();
  // println!("greetings in chinese: {}", chinese::hello());
  // println!("farewells in chinese: {}", chinese::goodbye());
  // println!("greetings in englist: {}", greetings::hello());
//  println!("hello in chinese: {}", phrases_lib::chinese::greetings::hello()) 
}
