use std::str::FromStr;

#[derive(Copy, Clone)]
enum Direction {
    Forward,
    Up,
    Down,
}

#[derive(Copy, Clone)]
struct Command {
    direction: Direction,
    distance: usize,
}

trait Pilot {
    fn up(&mut self, n: usize);
    fn down(&mut self, n: usize);
    fn forward(&mut self, n: usize);

    fn execute(&mut self, command: Command) {
        let Command {
            direction,
            distance,
        } = command;
        match direction {
            Direction::Up => self.up(distance),
            Direction::Down => self.down(distance),
            Direction::Forward => self.forward(distance),
        }
    }
}

struct Position1 {
    x: usize,
    y: usize,
}

impl Position1 {
    fn new() -> Self {
        Position1 { x: 0, y: 0 }
    }
}

impl Pilot for Position1 {
    fn up(&mut self, n: usize) {
        self.y -= n
    }

    fn down(&mut self, n: usize) {
        self.y += n
    }

    fn forward(&mut self, n: usize) {
        self.x += n
    }
}

struct Position2 {
    x: usize,
    y: usize,
    aim: usize,
}

impl Position2 {
    fn new() -> Self {
        Position2 { x: 0, y: 0, aim: 0 }
    }
}

impl Pilot for Position2 {
    fn up(&mut self, n: usize) {
        self.aim -= n
    }

    fn down(&mut self, n: usize) {
        self.aim += n
    }

    fn forward(&mut self, n: usize) {
        self.x += n;
        self.y += n * self.aim
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(cmd: &str) -> Result<Self, Self::Err> {
        let (direction_str, distance_str) = cmd
            .split_once(' ')
            .ok_or(format!("Malformed Command: {}", cmd))?;
        let distance = distance_str
            .parse()
            .map_err(|_| format!("Could not parse distance: {}", distance_str))?;
        let direction = match direction_str {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(format!("Invalid Direction: {}", direction_str)),
        }?;
        Ok(Command {
            direction,
            distance,
        })
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut pos1 = Position1::new();
    let mut pos2 = Position2::new();
    input.lines().map(|l| l.parse().unwrap()).for_each(|cmd| {
        pos1.execute(cmd);
        pos2.execute(cmd)
    });
    println!("{}, {}", pos1.x * pos1.y, pos2.x * pos2.y);
}
