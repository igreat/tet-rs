use crate::constants::*;
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub enum GameState {
    Menu,
    Playing,
    GameOver,
}

#[derive(Clone, Copy)]
pub enum Tetromino {
    E = 0, // Empty
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl Tetromino {
    pub fn get_color(&self) -> Color {
        match self {
            Tetromino::E => Color::from_hex(0x000000),
            Tetromino::I => Color::from_hex(0x00F0F0),
            Tetromino::O => Color::from_hex(0xF0F000),
            Tetromino::T => Color::from_hex(0xA000F0),
            Tetromino::S => Color::from_hex(0x00F000),
            Tetromino::Z => Color::from_hex(0xF00000),
            Tetromino::J => Color::from_hex(0x0000F0),
            Tetromino::L => Color::from_hex(0xF0A000),
        }
    }
}

#[derive(Clone)]
pub struct Piece {
    tetromino: Tetromino,
    x: isize,
    y: isize,
    orientation: Orientation,
}

impl Piece {
    pub fn new(tetromino: Tetromino) -> Piece {
        Piece {
            tetromino,
            x: 0,
            y: 0,
            orientation: Orientation::Up,
        }
    }
    fn get_base_coords(&self) -> [(isize, isize); 4] {
        match self.tetromino {
            Tetromino::I => match self.orientation {
                Orientation::Up | Orientation::Down => [(3, 0), (4, 0), (5, 0), (6, 0)],
                Orientation::Right | Orientation::Left => [(5, 0), (5, 1), (5, 2), (5, 3)],
            },
            Tetromino::O => [(4, 0), (5, 0), (4, 1), (5, 1)],
            Tetromino::T => match self.orientation {
                Orientation::Up => [(4, 0), (3, 1), (4, 1), (5, 1)],
                Orientation::Right => [(4, 0), (5, 1), (4, 1), (4, 2)],
                Orientation::Down => [(4, 1), (3, 0), (4, 0), (5, 0)],
                Orientation::Left => [(4, 0), (3, 1), (4, 1), (4, 2)],
            },
            Tetromino::L => match self.orientation {
                Orientation::Up => [(3, 1), (4, 1), (5, 1), (5, 0)],
                Orientation::Right => [(4, 0), (4, 1), (4, 2), (5, 2)],
                Orientation::Down => [(3, 0), (4, 0), (5, 0), (3, 1)],
                Orientation::Left => [(3, 0), (4, 0), (4, 1), (4, 2)],
            },
            Tetromino::J => match self.orientation {
                Orientation::Up => [(3, 1), (4, 1), (5, 1), (3, 0)],
                Orientation::Right => [(4, 0), (4, 1), (4, 2), (5, 0)],
                Orientation::Down => [(3, 0), (4, 0), (5, 0), (5, 1)],
                Orientation::Left => [(4, 0), (4, 1), (4, 2), (3, 2)],
            },
            Tetromino::S => match self.orientation {
                Orientation::Up | Orientation::Down => [(4, 0), (5, 0), (3, 1), (4, 1)],
                Orientation::Right | Orientation::Left => [(4, 0), (4, 1), (5, 1), (5, 2)],
            },
            Tetromino::Z => match self.orientation {
                Orientation::Up | Orientation::Down => [(3, 0), (4, 0), (4, 1), (5, 1)],
                Orientation::Right | Orientation::Left => [(5, 0), (4, 1), (5, 1), (4, 2)],
            },
            Tetromino::E => [(0, 0), (0, 0), (0, 0), (0, 0)], // useless case
        }
    }

    pub fn get_coords(&self) -> [(isize, isize); 4] {
        let base_coords = self.get_base_coords();
        let mut coords = [(0, 0); 4];
        for i in 0..4 {
            coords[i] = (base_coords[i].0 + self.x, base_coords[i].1 + self.y);
        }
        coords
    }
}

impl Piece {
    fn rotate(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        };
    }

    fn move_left(&mut self) {
        self.x -= 1;
    }

    fn move_right(&mut self) {
        self.x += 1;
    }

    fn move_down(&mut self) {
        self.y += 1;
    }
}

#[derive(Clone, Copy)]
pub enum Orientation {
    Up = 0,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
pub enum Move {
    Left,
    Right,
    Down,
    Rotate,
    Drop,
}

#[derive(Clone, Copy)]
pub struct Board {
    pub grid: [[Tetromino; WIDTH]; HEIGHT],
    pub score: u32,
    pub just_dropped: bool,
    is_placed_time: Option<f64>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            grid: [[Tetromino::E; WIDTH]; HEIGHT],
            score: 0,
            just_dropped: false,
            is_placed_time: None,
        }
    }

    pub fn add_piece(&mut self, piece: &Piece) {
        for &(x, y) in &piece.get_coords() {
            self.grid[y as usize][x as usize] = piece.tetromino;
        }
    }

    fn remove_piece(&mut self, piece: &Piece) {
        for &(x, y) in &piece.get_coords() {
            self.grid[y as usize][x as usize] = Tetromino::E;
        }
    }

    fn is_out_of_bounds(&self, piece: &Piece) -> bool {
        for &(x, y) in &piece.get_coords() {
            if x < 0 || x >= WIDTH as isize || y >= HEIGHT as isize {
                return true;
            }
        }
        false
    }

    pub fn is_colliding(&self, piece: &Piece) -> bool {
        for &(x, y) in &piece.get_coords() {
            match self.grid[y as usize][x as usize] {
                Tetromino::E => continue,
                _ => return true,
            }
        }
        false
    }

    pub fn is_placed(&mut self, piece: &Piece) -> bool {
        // first I'll have to remove the piece from the board
        self.remove_piece(piece);
        let mut is_placed = false;
        // then check if it can move down
        for &(x, y) in &piece.get_coords() {
            if y == HEIGHT as isize - 1 {
                is_placed = true;
                break;
            }
            match self.grid[(y + 1) as usize][x as usize] {
                Tetromino::E => continue,
                _ => {
                    is_placed = true;
                    break;
                }
            }
        }
        self.add_piece(piece);
        if is_placed {
            match self.is_placed_time {
                None => {
                    self.is_placed_time = Some(get_time()); // start the timer
                }
                Some(time) => {
                    if get_time() - time > PLACEMENT_DELAY {
                        self.is_placed_time = None;
                        return true;
                    }
                }
            }
        } else if self.is_placed_time.is_some() {
            // if the piece isn't placed but the timer is running, then the timer should be reset
            self.is_placed_time = None;
        }
        false
    }

    fn adjust_rotation(&mut self, piece: &mut Piece) {
        // check if the piece is out of bounds and the closest way to make it in bounds
        let mut x_offset = 0;
        let coords = piece.get_coords();
        for &(x, _) in &coords {
            if x < 0 && -x > x_offset {
                x_offset = -x;
            } else if (x >= WIDTH as isize) && -(x - (WIDTH as isize - 1)) < x_offset {
                x_offset = -(x - (WIDTH as isize - 1));
            }
        }
        piece.x += x_offset;
    }

    fn can_move(&mut self, piece: &Piece, mov: Move) -> bool {
        // will have to remove the piece from the board, move it, and then add it back
        self.remove_piece(piece);
        let mut piece_copy = piece.clone();
        match mov {
            Move::Left => piece_copy.move_left(),
            Move::Right => piece_copy.move_right(),
            Move::Down => piece_copy.move_down(),
            Move::Rotate => {
                piece_copy.rotate();
                self.adjust_rotation(&mut piece_copy)
            }
            Move::Drop => {
                self.drop_piece(&mut piece_copy, false);
                // make sure to remove the final piece
                self.remove_piece(&piece_copy);
            }
        }
        let can_move = !self.is_out_of_bounds(&piece_copy) && !self.is_colliding(&piece_copy);
        self.add_piece(piece);
        can_move
    }

    pub fn move_piece(&mut self, piece: &mut Piece, mov: Move) {
        if self.can_move(piece, mov) {
            self.remove_piece(piece);
            match mov {
                Move::Left => piece.move_left(),
                Move::Right => piece.move_right(),
                Move::Down => {
                    piece.move_down();
                    self.score += 1;
                }
                Move::Rotate => {
                    piece.rotate();
                    self.adjust_rotation(piece);
                }
                Move::Drop => self.drop_piece(piece, true),
            }
            self.add_piece(piece);
        }
    }

    fn drop_piece(&mut self, piece: &mut Piece, update_score: bool) {
        while self.can_move(piece, Move::Down) {
            self.remove_piece(piece);
            piece.move_down();
            self.add_piece(piece);
            if update_score {
                self.score += 1;
            }
        }

        self.just_dropped = true;
        // make sure to reset the placement timer
        self.is_placed_time = None;
    }

    pub fn clear_lines(&mut self) {
        let mut clears = 0;
        let mut y = HEIGHT - 1;
        while y > 0 {
            let mut is_clear = true;
            for x in 0..WIDTH {
                if let Tetromino::E = self.grid[y][x] {
                    is_clear = false;
                    break;
                }
            }
            if is_clear {
                clears += 1;
                if y == HEIGHT - 1 {
                    for x in 0..WIDTH {
                        self.grid[y][x] = Tetromino::E;
                    }
                }
                for y2 in (1..=y).rev() {
                    for x in 0..WIDTH {
                        self.grid[y2][x] = self.grid[y2 - 1][x];
                    }
                }
            } else {
                y -= 1;
            }
        }

        match clears {
            1 => self.score += 25,
            2 => self.score += 100,
            3 => self.score += 400,
            4 => self.score += 1600,
            _ => (),
        }
    }

    fn print(&self) {
        for row in self.grid.iter() {
            for &cell in row {
                match cell {
                    Tetromino::E => print!("."),
                    _ => print!("X"),
                }
            }
            println!();
        }
        println!();
    }
}

pub struct PieceChooser {
    pub next_pieces: Vec<Tetromino>,
    max_next_pieces: usize,
    // TODO: currently doesn't do anything
    seed: u64,
}

impl PieceChooser {
    pub fn new(max_next_pieces: usize) -> PieceChooser {
        PieceChooser {
            next_pieces: (0..max_next_pieces)
                .map(|_| PieceChooser::get_random_piece())
                .collect(),
            max_next_pieces,
            seed: 0,
        }
    }

    pub fn get_next_piece(&mut self) -> Tetromino {
        let new_shape = PieceChooser::get_random_piece();
        let next_shape = if self.next_pieces.len() == self.max_next_pieces {
            self.next_pieces.pop().unwrap()
        } else {
            // this branch should technically never be reached
            // but I'll include this as a failsafe
            PieceChooser::get_random_piece()
        };

        self.next_pieces.insert(0, new_shape);

        next_shape
    }

    fn get_random_piece() -> Tetromino {
        *rand::ChooseRandom::choose(&vec![
            Tetromino::I,
            Tetromino::O,
            Tetromino::T,
            Tetromino::S,
            Tetromino::Z,
            Tetromino::J,
            Tetromino::L,
        ])
        .unwrap()
    }
}
