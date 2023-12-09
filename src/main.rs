use macroquad::{miniquad::window::set_window_size, prelude::*};

const WIDTH: usize = 10;
const HEIGHT: usize = 24;

const SQUARE_SIZE: f32 = 27.0;
const MARGIN_LEFT: f32 = SQUARE_SIZE;
const MARGIN_TOP: f32 = SQUARE_SIZE;

#[macroquad::main("Tetris")]
async fn main() {
    set_window_size(
        ((WIDTH + 2) as f32 * SQUARE_SIZE) as u32,
        ((HEIGHT + 3) as f32 * SQUARE_SIZE) as u32,
    );

    let mut board = Board::new();
    let mut piece = Piece {
        tetromino: Tetromino::T,
        x: 0,
        y: 0,
        orientation: Orientation::Up,
    };

    board.add_piece(&piece);
    let drop_time = 1.0;
    let mut prev_time = get_time();
    loop {
        // if is_key_pressed(KeyCode::Up) {
        //     board.remove_piece(&piece);
        //     piece.rotate();
        //     board.add_piece(&piece);
        // }
        // if is_key_pressed(KeyCode::Down) {
        //     board.remove_piece(&piece);
        //     piece.move_down();
        //     board.add_piece(&piece);
        // }
        // if is_key_pressed(KeyCode::Right) {
        //     board.remove_piece(&piece);
        //     piece.move_right();
        //     board.add_piece(&piece);
        // }
        // if is_key_pressed(KeyCode::Left) {
        //     board.remove_piece(&piece);
        //     piece.move_left();
        //     board.add_piece(&piece);
        // }
        // if get_time() - prev_time > drop_time {
        //     board.remove_piece(&piece);
        //     piece.move_down();
        //     board.add_piece(&piece);
        //     prev_time = get_time();
        // }

        if is_key_pressed(KeyCode::Up) {
            board.move_piece(&mut piece, Move::Rotate);
        }
        if is_key_pressed(KeyCode::Down) {
            board.move_piece(&mut piece, Move::Down);
        }
        if is_key_pressed(KeyCode::Right) {
            board.move_piece(&mut piece, Move::Right);
        }
        if is_key_pressed(KeyCode::Left) {
            board.move_piece(&mut piece, Move::Left);
        }
        if get_time() - prev_time > drop_time {
            board.move_piece(&mut piece, Move::Down);
            prev_time = get_time();
        }

        clear_background(LIGHTGRAY);
        draw_grid(WIDTH, HEIGHT);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color = match board.grid[y][x] {
                    Tetromino::E => {
                        continue;
                    }
                    Tetromino::I => BLUE,
                    Tetromino::O => YELLOW,
                    Tetromino::T => PURPLE,
                    Tetromino::S => GREEN,
                    Tetromino::Z => RED,
                    Tetromino::J => ORANGE,
                    Tetromino::L => BROWN,
                };
                // outer rectangle
                draw_rectangle(
                    x as f32 * SQUARE_SIZE + MARGIN_LEFT,
                    y as f32 * SQUARE_SIZE + MARGIN_TOP,
                    SQUARE_SIZE,
                    SQUARE_SIZE,
                    BLACK,
                );
                let inner_size = SQUARE_SIZE * 0.8;
                let inner_offset = (SQUARE_SIZE - inner_size) / 2.0;
                // inner rectangle
                draw_rectangle(
                    x as f32 * SQUARE_SIZE + inner_offset + MARGIN_LEFT,
                    y as f32 * SQUARE_SIZE + inner_offset + MARGIN_TOP,
                    inner_size,
                    inner_size,
                    color,
                );
            }
        }
        next_frame().await
    }
}

#[derive(Clone, Copy)]
enum Tetromino {
    E = 0, // Empty
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

#[derive(Clone)]
struct Piece {
    tetromino: Tetromino,
    x: isize,
    y: isize,
    orientation: Orientation,
}

impl Piece {
    fn get_base_coords(&self) -> [(isize, isize); 4] {
        let coords; // Initialize an array with default values
        match self.tetromino {
            Tetromino::I => match self.orientation {
                Orientation::Up | Orientation::Down => {
                    coords = [(3, 0), (4, 0), (5, 0), (6, 0)];
                }
                Orientation::Right | Orientation::Left => {
                    coords = [(5, 0), (5, 1), (5, 2), (5, 3)];
                }
            },
            Tetromino::O => {
                coords = [(4, 0), (5, 0), (4, 1), (5, 1)];
            }
            Tetromino::T => match self.orientation {
                Orientation::Up => {
                    coords = [(4, 0), (3, 1), (4, 1), (5, 1)];
                }
                Orientation::Right => {
                    coords = [(4, 0), (5, 1), (4, 1), (4, 2)];
                }
                Orientation::Down => {
                    coords = [(4, 1), (3, 0), (4, 0), (5, 0)];
                }
                Orientation::Left => {
                    coords = [(4, 0), (3, 1), (4, 1), (4, 2)];
                }
            },
            _ => {
                coords = [(0, 0); 4]; // Default position for other shapes
            }
        };
        coords
    }

    fn get_coords(&self) -> [(isize, isize); 4] {
        let base_coords = self.get_base_coords();
        let mut coords = [(0, 0); 4];
        for i in 0..4 {
            // coords[i] = (base_coords[i].0 + self.x, base_coords[i].1 + self.y);
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
        }
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
enum Orientation {
    Up = 0,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
enum Move {
    Left,
    Right,
    Down,
    Rotate,
}

#[derive(Clone, Copy)]
struct Board {
    grid: [[Tetromino; WIDTH]; HEIGHT],
}

impl Board {
    fn new() -> Board {
        Board {
            grid: [[Tetromino::E; WIDTH]; HEIGHT],
        }
    }

    fn add_piece(&mut self, piece: &Piece) {
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

    fn is_colliding(&self, piece: &Piece) -> bool {
        for &(x, y) in &piece.get_coords() {
            match self.grid[y as usize][x as usize] {
                Tetromino::E => continue,
                _ => return true,
            }
        }
        false
    }

    // to check if I can make a move, I'll create a copy of the piece and try to move it
    // if it's possible, I'll update the original piece
    fn can_move(&mut self, piece: &Piece, mov: Move) -> bool {
        // will have to remove the piece from the board, move it, and then add it back
        self.remove_piece(piece);
        let mut piece_copy = piece.clone();
        match mov {
            Move::Left => piece_copy.move_left(),
            Move::Right => piece_copy.move_right(),
            Move::Down => piece_copy.move_down(),
            Move::Rotate => piece_copy.rotate(),
        }
        let can_move = !self.is_out_of_bounds(&piece_copy) && !self.is_colliding(&piece_copy);
        self.add_piece(piece);
        can_move
    }

    fn move_piece(&mut self, piece: &mut Piece, mov: Move) {
        if self.can_move(piece, mov) {
            self.remove_piece(piece);
            match mov {
                Move::Left => piece.move_left(),
                Move::Right => piece.move_right(),
                Move::Down => piece.move_down(),
                Move::Rotate => piece.rotate(),
            }
            self.add_piece(piece);
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

fn draw_grid(width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            draw_rectangle_lines(
                x as f32 * SQUARE_SIZE + MARGIN_LEFT,
                y as f32 * SQUARE_SIZE + MARGIN_TOP,
                SQUARE_SIZE,
                SQUARE_SIZE,
                1.25,
                BLACK,
            );
        }
    }
}
