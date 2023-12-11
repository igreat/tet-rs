use macroquad::{miniquad::window::set_window_size, prelude::*};
use std::time::SystemTime;

const WIDTH: usize = 10;
const HEIGHT: usize = 24;

// UI STUFF
const SQUARE_SIZE: f32 = 27.0;
const MARGIN_LEFT: f32 = SQUARE_SIZE;
const MARGIN_TOP: f32 = SQUARE_SIZE;
const SIDE_PANEL_WIDTH: f32 = SQUARE_SIZE * 5.0;

const PLACEMENT_DELAY: f64 = 0.5;

// the maximum number of tetrominos that can be placed on the board
const TETROMINO_LIMIT: usize = 400;

#[macroquad::main("Tetris")]
async fn main() {
    set_window_size(
        ((WIDTH + 2) as f32 * SQUARE_SIZE) as u32 + SIDE_PANEL_WIDTH as u32,
        ((HEIGHT + 3) as f32 * SQUARE_SIZE) as u32,
    );

    let mut piece_chooser = PieceChooser::new(3);

    let mut board = Board::new();
    let mut next_shape = piece_chooser.get_next_piece();
    let mut piece = Piece {
        tetromino: next_shape,
        x: 0,
        y: 0,
        orientation: Orientation::Up,
    };

    board.add_piece(&piece);
    let drop_time = 1.0;
    let mut prev_time = get_time();

    let mut num_tetrominos = 0;

    loop {
        set_window_size(
            ((WIDTH + 2) as f32 * SQUARE_SIZE) as u32 + SIDE_PANEL_WIDTH as u32,
            ((HEIGHT + 3) as f32 * SQUARE_SIZE) as u32,
        );

        if num_tetrominos >= TETROMINO_LIMIT {
            println!("Out of pieces!");
            break;
        }

        // check if the previous piece is placed
        if board.is_placed(&piece) || board.just_dropped {
            board.clear_lines();

            // get the next piece
            next_shape = piece_chooser.get_next_piece();
            piece = Piece {
                tetromino: next_shape,
                x: 0,
                y: 0,
                orientation: Orientation::Up,
            };

            // check collision with the new piece
            if board.is_colliding(&piece) {
                println!("Game over!");
                break;
            }
            board.add_piece(&piece);

            board.just_dropped = false;

            num_tetrominos += 1;
        }

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
        if is_key_pressed(KeyCode::Space) {
            board.move_piece(&mut piece, Move::Drop);
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

        // display the score
        let side_panel_middle = WIDTH as f32 * SQUARE_SIZE + SIDE_PANEL_WIDTH / 2.0;
        let side_panel_margin_left = side_panel_middle - SIDE_PANEL_WIDTH / 5.0;
        // draw bounding box for the whole score
        draw_rectangle(
            side_panel_margin_left,
            MARGIN_TOP,
            SIDE_PANEL_WIDTH,
            SQUARE_SIZE * 3.0,
            GRAY,
        );
        draw_text(
            "Score",
            side_panel_middle,
            MARGIN_TOP + SQUARE_SIZE,
            35.0,
            BLACK,
        );
        draw_text(
            &board.score.to_string(),
            side_panel_middle,
            MARGIN_TOP + SQUARE_SIZE * 2.5,
            35.0,
            BLACK,
        );

        // display the next pieces
        // draw bounding box for the next pieces
        draw_rectangle(
            side_panel_margin_left,
            MARGIN_TOP * 5.0,
            SIDE_PANEL_WIDTH,
            SQUARE_SIZE * 8.0,
            GRAY,
        );
        draw_text(
            "Next",
            side_panel_middle + 10.0,
            MARGIN_TOP * 5.0 + SQUARE_SIZE,
            35.0,
            BLACK,
        );

        let mut next_pieces = piece_chooser.next_pieces.clone();
        next_pieces.reverse();
        for (i, next_piece) in next_pieces.iter().enumerate() {
            let x = side_panel_middle - SIDE_PANEL_WIDTH / 2.0;
            let y = MARGIN_TOP + SQUARE_SIZE * 4.0 + SQUARE_SIZE * i as f32 * 4.0;
            for &(x, y) in &(Piece {
                tetromino: *next_piece,
                x: 0,
                y: 0,
                orientation: Orientation::Up,
            })
            .get_coords()
            {
                let color = match next_piece {
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
                    side_panel_margin_left + x as f32 * SQUARE_SIZE * 0.5,
                    MARGIN_TOP * 5.0
                        + y as f32 * SQUARE_SIZE * 0.5
                        + SQUARE_SIZE * 4.0 * 0.5
                        + SQUARE_SIZE * i as f32 * 4.0 * 0.5,
                    SQUARE_SIZE * 0.5,
                    SQUARE_SIZE * 0.5,
                    BLACK,
                );

                // inner rectangle
                let inner_size = SQUARE_SIZE * 0.4;
                let inner_offset = (SQUARE_SIZE * 0.5 - inner_size) / 2.0;
                draw_rectangle(
                    side_panel_margin_left + x as f32 * SQUARE_SIZE * 0.5 + inner_offset,
                    MARGIN_TOP * 5.0
                        + y as f32 * SQUARE_SIZE * 0.5
                        + SQUARE_SIZE * 4.0 * 0.5
                        + SQUARE_SIZE * i as f32 * 4.0 * 0.5
                        + inner_offset,
                    inner_size,
                    inner_size,
                    color,
                );
            }
        }

        next_frame().await
    }

    println!("Final score: {}", board.score);
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

    fn get_coords(&self) -> [(isize, isize); 4] {
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
    Drop,
}

#[derive(Clone, Copy)]
struct Board {
    grid: [[Tetromino; WIDTH]; HEIGHT],
    score: u32,
    just_dropped: bool,
    is_placed_time: Option<f64>,
}

impl Board {
    fn new() -> Board {
        Board {
            grid: [[Tetromino::E; WIDTH]; HEIGHT],
            score: 0,
            just_dropped: false,
            is_placed_time: None,
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

    fn is_placed(&mut self, piece: &Piece) -> bool {
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

    fn move_piece(&mut self, piece: &mut Piece, mov: Move) {
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

    fn clear_lines(&mut self) {
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

        if clears > 0 {
            println!("Score: {}", self.score);
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

struct PieceChooser {
    next_pieces: Vec<Tetromino>,
    max_next_pieces: usize,
    // TODO: currently doesn't do anything
    seed: u64,
}

impl PieceChooser {
    fn new(max_next_pieces: usize) -> PieceChooser {
        PieceChooser {
            next_pieces: (0..max_next_pieces)
                .map(|_| PieceChooser::_get_random_piece())
                .collect(),
            max_next_pieces,
            seed: 0,
        }
    }

    fn get_next_piece(&mut self) -> Tetromino {
        let new_shape = PieceChooser::_get_random_piece();

        let next_shape;
        if self.next_pieces.len() == self.max_next_pieces {
            next_shape = self.next_pieces.pop().unwrap()
        } else {
            // this branch should technically never be reached
            // but I'll include this as a failsafe
            next_shape = PieceChooser::_get_random_piece();
        }

        self.next_pieces.insert(0, new_shape);

        next_shape
    }

    fn _get_random_piece() -> Tetromino {
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
