 enum Direction {
        North,
        West,
        South,
        East,
    }

fn main(){
    let sp_direction = Direction::North;
    match sp_direction {
        Direction::North => println!("Going North"),
        Direction::West => println!("Going West"),
        Direction::South => println!("Going South"),
        Direction::East => println!("Going East"),
    }
}