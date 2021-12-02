use std::str::FromStr;

enum Direction{
    Forward,
    Up,
    Down
}

struct Command {
    direction: Direction,
    distance: usize
}

struct Position {
    x: usize,
    y: usize,
    aim: usize
}

impl FromStr for Command {
    type Err = String;

    fn from_str(cmd: &str) -> Result<Self, Self::Err> {
        let (direction_str, distance_str) = cmd.split_once(' ').ok_or(format!("Malformed Command: {}", cmd))?;
        let distance = distance_str.parse().or(Err(format!("Could not parse distance: {}", distance_str)))?;
        let direction = match direction_str {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(format!("Invalid Direction: {}", direction_str))
        }?;
        Ok(Command {direction, distance})
    }
}

fn execute_command_part1(position: Position, command: Command) -> Position {
    let Position {x, y, aim} = position;
    match command.direction {
        Direction::Forward => Position {x: x + command.distance, y, aim},
        Direction::Up => Position {x, y: y - command.distance, aim},
        Direction::Down => Position {x, y: y + command.distance, aim}
    }
}

fn execute_command_part2(position: Position, command: Command) -> Position {
    let Position {x, y, aim} = position;
    match command.direction {
        Direction::Forward => Position {x: x + command.distance, y: y + command.distance * aim, aim},
        Direction::Up => Position {x, y, aim: aim - command.distance},
        Direction::Down => Position {x, y, aim: aim + command.distance}
    }
}

fn main() {
    let input = include_str!("input.txt");
    let Position {x, y, aim: _} = input.split_terminator('\n').map(|l| l.parse().unwrap()).fold(Position {x: 0, y: 0, aim: 0}, execute_command_part1);
    println!("{}", x*y);
    let Position {x, y, aim: _} = input.split_terminator('\n').map(|l| l.parse().unwrap()).fold(Position {x: 0, y: 0, aim: 0}, execute_command_part2);
    println!("{}", x*y)
}
