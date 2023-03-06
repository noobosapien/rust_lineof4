use std::fmt;

#[derive(Debug)]
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
        if self.last_point.n {
            let convert_to_index = |p: &Point| -> u8 { (p.j + (p.i * 8)) as u8 };

            let check_point_validity = |point: &Point| {
                let index = convert_to_index(point);
                if index > 63 {
                    return false;
                }

                true
            };

            let old_point = &self.last_point;

            let index = convert_to_index(old_point);
            println!("at {}: {}", index, self.cells[index as usize]);
            //left diagonal
            //right diagonal
            //vertical

            //-----Horizontal-----
            //get 3 points to the left and 3 points to the right
            //check validity of those points
            //check if 4 are in a row fro those points

            let mut pointVectors: Vec<Point> = vec![];

            for i in 0..2 {
                for j in 1..=3 {
                    if i == 0 {
                        //right
                        let new_point = Point {
                            i: old_point.i + j,
                            j: old_point.j,
                            n: true,
                        };

                        if check_point_validity(&new_point) {
                            pointVectors.push(new_point);
                        }
                    } else {
                        //left

                        if (old_point.i as i16 - j as i16) >= 0 {
                            let new_point = Point {
                                i: old_point.i - j,
                                j: old_point.j,
                                n: true,
                            };

                            if check_point_validity(&new_point) {
                                pointVectors.push(new_point);
                            }
                        }
                    }
                }
            }

            // for all points in the point vector check what is present if 3 of the same are in a row
            // declare winner

            let mut count = 0;

            let cur_player = match self.current_player {
                0 => Cell::X,
                1 => Cell::O,
                _ => Cell::E,
            };

            for _point in pointVectors {
                let index = convert_to_index(&_point);
                println!(
                    "at point with current player: {} : {}",
                    self.current_player, self.cells[index as usize]
                );
                let cell = &self.cells[index as usize];

                if *cell != cur_player {
                    break;
                }

                count += 1;
            }

            println!("count = {}", count);

            if count >= 3 {
                return Some(cur_player);
            } else {
                return None;
            }
        }

        None
    }

    fn change_player(&mut self) {
        match self.current_player {
            0 => {
                self.current_player = 1;
            }
            1 => {
                self.current_player = 0;
            }
            _ => {}
        }
    }

    fn add_value(&mut self, value: i32) -> Option<bool> {
        for i in (0..8).rev() {
            let index = (value + (i * 8)) as usize;

            if self.cells[index] == Cell::E {
                match self.current_player {
                    0 => {
                        self.cells[index] = Cell::X;
                    }
                    1 => {
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
                Err(_e) => {}
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

            match self.add_value(value) {
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

            self.change_player();
        }
    }
}

fn main() {
    let mut game = Game::init();
    println!("Initiated the game");
    game.play();
}
