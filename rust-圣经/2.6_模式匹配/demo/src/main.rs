/**
 * match
 */

enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => {
            println!("东方")
        }
        Direction::West => {
            panic!("西方白虎")
        }
        _ => println!("other"),
    }
}
