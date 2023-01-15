use std::fmt;

struct Point {
    i: u8,
    j: u8,
    n: bool,
}

#[derive(PartialEq)]
enum Cell {
    E,
    X,
    O,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cell::E => write!(f, " "),
            Cell::X => write!(f, "X"),
            Cell::O => write!(f, "O"),
        }
    }
}

struct Game {
    cells: Vec<Cell>,
    current_player: u8,
    valid: bool,
    last_point: Point,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::new();

        string += "  1  2  3  4  5  6  7  8\n";

        for i in 0..8 {
            for j in 0..8 {
                string += format!("| {}", self.cells[j + (i * 8)]).as_str();
            }

            string += "|\n";
        }

        write!(f, "{}", string)
    }
}

impl Game {
    fn init() -> Game {
        let mut cells: Vec<Cell> = Vec::with_capacity(64 as usize);

        for _i in 0..8 {
            for _j in 0..8 {
                // cells[j + (i * 8)] = Cell::E;
                cells.push(Cell::E);
            }
        }

        let current_player = 0;
        let valid = true;
        let last_point = Point {
            i: 0,
            j: 0,
            n: false,
        };

        let game = Game {
            cells,
            current_player: current_player as u8,
            valid,
            last_point,
        };

        game
    }

    fn check_validity(&mut self) -> Result<bool, bool> {
        for i in 0..8 {
            for j in 0..8 {
                if self.cells[j + (i * 8)] == Cell::E {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    fn check_winner(&mut self) -> Option<Cell> {
        println!("{} {}", self.last_point.i, self.last_point.j);

        None
    }

    fn add_value_and_change_player(&mut self, value: i32) -> Option<bool> {
        for i in (0..8).rev() {
            let index = (value + (i * 8)) as usize;

            if self.cells[index] == Cell::E {
                match self.current_player {
                    0 => {
                        self.current_player = 1;
                        self.cells[index] = Cell::X;
                    }
                    1 => {
                        self.current_player = 0;
                        self.cells[index] = Cell::O;
                    }
                    _ => self.cells[index] = Cell::E,
                }

                self.last_point = Point {
                    i: i as u8,
                    j: value as u8,
                    n: true,
                };

                return Some(true);
            }
        }

        None
    }

    fn play(&mut self) {
        loop {
            println!("{}\n", self);

            match self.check_validity() {
                Ok(val) => {
                    if !val {
                        println!("There is no winner.\n");
                        self.valid = false;
                        break;
                    }
                }
                Err(e) => {}
            }

            let mut input: String = String::new();

            match self.current_player {
                0 => println!("Player 'X' select a lane.\n"),
                1 => println!("Player 'O' select a lane \n"),
                _ => {
                    println!("Error.\n");
                    break;
                }
            };

            std::io::stdin().read_line(&mut input).unwrap();

            let mut value = match input.trim().parse::<i32>() {
                Ok(n) => {
                    if n <= 8 && n > 0 {
                        n
                    } else {
                        -10
                    }
                }
                Err(_e) => -10,
            };

            if value < 0 {
                println!("Please select a valid number.\n");
                continue;
            }

            value -= 1;

            match self.add_value_and_change_player(value) {
                Some(_value) => {}
                None => {
                    println!("Please select a different lane because this lane is full.\n");
                    continue;
                }
            }

            match self.check_winner() {
                Some(n) => {
                    println!("Player {} is the winner.", n);
                    break;
                }
                None => {}
            }
        }
    }
}

fn main() {
    let mut game = Game::init();
    println!("Initiated the game");
    game.play();
}
