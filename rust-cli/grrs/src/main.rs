/*
 * @Author: saber
 * @Date: 2021-12-23 21:43:14
 * @LastEditTime: 2021-12-23 21:55:15
 * @LastEditors: saber
 * @Description:
 */
fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    struct Cli {
        pattern: String,
        path: std::path::PathBuf,
    }
    println!("Hello, world!");
}
