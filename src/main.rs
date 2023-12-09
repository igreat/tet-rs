use macroquad::{miniquad::window::set_window_size, prelude::*};

const WIDTH: usize = 10;
const HEIGHT: usize = 24;

#[macroquad::main("Tetris")]
async fn main() {
    set_window_size(WIDTH as u32 * 20, HEIGHT as u32 * 20);

    let mut board = Board::new();
    let mut piece = Piece {
        tetromino: Tetromino::I,
        x: 0,
        y: 0,
        orientation: Orientation::Up,
    };

    board.add_piece(&piece);

    let mut prev_time_down = get_time();
    let mut prev_time_rotate = get_time();
    let dt = 1.0;
    loop {
        if get_time() - prev_time_down > dt {
            board.remove_piece(&piece);
            piece.move_down();
            board.add_piece(&piece);
            prev_time_down = get_time();
        }
        if get_time() - prev_time_rotate > 3.0 * dt {
            board.remove_piece(&piece);
            piece.rotate();
            board.add_piece(&piece);
            prev_time_rotate = get_time();
        }

        clear_background(WHITE);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color = match board.grid[y][x] {
                    Tetromino::E => WHITE,
                    Tetromino::I => BLUE,
                    Tetromino::O => YELLOW,
                    Tetromino::T => PURPLE,
                    Tetromino::S => GREEN,
                    Tetromino::Z => RED,
                    Tetromino::J => ORANGE,
                    Tetromino::L => BROWN,
                };
                // outer rectangle
                draw_rectangle(x as f32 * 20.0, y as f32 * 20.0, 20.0, 20.0, BLACK);
                // inner rectangle
                draw_rectangle(
                    x as f32 * 20.0 + 1.0,
                    y as f32 * 20.0 + 1.0,
                    18.0,
                    18.0,
                    color,
                );
            }
        }
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        next_frame().await
    }

    // board.add_piece(&piece);
    // board.print();
    // board.remove_piece(&piece);
    // piece.rotate();
    // // piece.move_down();
    // board.add_piece(&piece);
    // board.print();
    // board.remove_piece(&piece);
    // piece.rotate();
    // // piece.move_down();
    // board.add_piece(&piece);
    // board.print();
    // board.remove_piece(&piece);
    // piece.rotate();
    // // piece.move_down();
    // board.add_piece(&piece);
    // board.print();
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
struct Piece {
    tetromino: Tetromino,
    x: usize,
    y: usize,
    orientation: Orientation,
}

impl Piece {
    fn get_base_coords(&self) -> [(usize, usize); 4] {
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
                coords = [(self.x, self.y); 4]; // Default position for other shapes
            }
        };
        coords
    }

    fn get_coords(&self) -> [(usize, usize); 4] {
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

    fn move_up(&mut self) {
        self.y -= 1;
    }
}

enum Orientation {
    Up = 0,
    Right,
    Down,
    Left,
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
            self.grid[y][x] = piece.tetromino;
        }
    }

    fn remove_piece(&mut self, piece: &Piece) {
        for &(x, y) in &piece.get_coords() {
            self.grid[y][x] = Tetromino::E;
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
