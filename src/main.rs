use std::io::{self, Write};

const GRID_WIDTH: i32 = 10;
const GRID_HEIGHT: i32 = 10;

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Mission {
    target_x: i32,
    target_y: i32,
    description: String,
    completed: bool,
}

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    facing: Direction,
    mission: Option<Mission>,
}

impl Robot {
    fn new() -> Self {
        Robot {
            x: 0,
            y: 0,
            facing: Direction::North,
            mission: None,
        }
    }

    fn turn_left(&mut self) {
        self.facing = match self.facing {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };
    }

    fn turn_right(&mut self) {
        self.facing = match self.facing {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }

    fn move_forward(&mut self) {
        let (new_x, new_y) = match self.facing {
            Direction::North => (self.x, self.y + 1),
            Direction::East => (self.x + 1, self.y),
            Direction::South => (self.x, self.y - 1),
            Direction::West => (self.x - 1, self.y),
        };

        if new_x >= 0 && new_x < GRID_WIDTH && new_y >= 0 && new_y < GRID_HEIGHT {
            self.x = new_x;
            self.y = new_y;
        } else {
            println!("⚠️  Cannot move forward — would exit the grid!");
        }
    }

    fn status(&self) {
        println!(
            "Robot is at ({}, {}) facing {:?}",
            self.x, self.y, self.facing
        );
    }

    fn draw_grid(&self) {
        println!("\nCurrent Grid:");
        for y in (0..GRID_HEIGHT).rev() {
            for x in 0..GRID_WIDTH {
                if self.x == x && self.y == y {
                    print!(" R ");
                } else {
                    print!(" . ");
                }
            }
            println!();
        }
        println!();
    }

    fn goto(&mut self, target_x: i32, target_y: i32) {
        if target_x < 0 || target_x >= GRID_WIDTH || target_y < 0 || target_y >= GRID_HEIGHT {
            println!("Target out of bounds!");
            return;
        }

        while self.x != target_x {
            if self.x < target_x {
                self.facing = Direction::East;
            } else {
                self.facing = Direction::West;
            }
            self.move_forward();
            self.status();
        }

        while self.y != target_y {
            if self.y < target_y {
                self.facing = Direction::North;
            } else {
                self.facing = Direction::South;
            }
            self.move_forward();
            self.status();
        }

        println!("🎯 Arrived at ({}, {})!", self.x, self.y);
        if let Some(mission) = &mut self.mission {
            if self.x == mission.target_x && self.y == mission.target_y {
                mission.completed = true;
                println!("✅ Mission completed: {}", mission.description);
            }
        }
    }
    
    fn assign_mission(&mut self, x: i32, y: i32, description: String) {
        self.mission = Some(Mission {
            target_x: x,
            target_y: y,
            description,
            completed: false,
        });
        println!("New mission assigned: {}", self.mission.as_ref().unwrap().description);
    }

}

fn main() {
    let mut robot = Robot::new();
    println!("Welcome to the Rust Robot Simulator!");
    println!("Commands: FORWARD, LEFT, RIGHT, STATUS, MAP, GOTO x y, MISSION, or QUIT");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let cmd = parts[0].to_uppercase();

        match cmd.as_str() {
            "FORWARD" => robot.move_forward(),
            "LEFT" => robot.turn_left(),
            "RIGHT" => robot.turn_right(),
            "STATUS" => robot.status(),
            "MAP" => robot.draw_grid(),
            "GOTO" => {
                if parts.len() != 3 {
                    println!("Usage: GOTO x y");
                    continue;
                }

                let x = parts[1].parse::<i32>();
                let y = parts[2].parse::<i32>();

                match (x, y) {
                    (Ok(tx), Ok(ty)) => robot.goto(tx, ty),
                    _ => println!("Invalid coordinates. Please enter integers."),
                }
            }
            "MISSION" => {
                if parts.len() < 4 {
                    println!("Usage: MISSION x y description");
                    continue;
                }
            
                let x = parts[1].parse::<i32>();
                let y = parts[2].parse::<i32>();
                let description = parts[3..].join(" ");
            
                match (x, y) {
                    (Ok(tx), Ok(ty)) => robot.assign_mission(tx, ty, description),
                    _ => println!("Invalid coordinates. Please enter integers."),
                }
            }
            "QUIT" => {
                println!("Exiting the simulator. Goodbye!");
                break;
            }
            _ => println!("Unknown command. Try FORWARD, LEFT, RIGHT, STATUS, MAP, GOTO x y, or QUIT."),
        }
    }
}
