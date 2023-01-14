use std::fmt;

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
    won: Cell,
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
        let won = Cell::E;

        let game = Game {
            cells,
            current_player: current_player as u8,
            valid,
            won,
        };

        game
    }

    fn play(&mut self) {}
}

fn main() {
    let mut game = Game::init();
    println!("Initiated the game");
    game.play();
}
