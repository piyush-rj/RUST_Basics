enum Direction {
    North,
    South,
    East,
    West,
}

fn get_direction(direction: Direction) {
    match direction {
        Direction::North => print!("the direction is north"),
        Direction::South => print!("the direction is north"),
        Direction::East => print!("the direction is East"),
        Direction::West => print!("the direction is West"),
    }
}

fn main() {
    println!("hello");
    get_direction(Direction::North);
}
