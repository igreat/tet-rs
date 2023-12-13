use macroquad::{miniquad::window::set_window_size, prelude::*};
use std::time::SystemTime;

mod game;

use game::{Board, GameState, Move, Piece, PieceChooser, Tetromino};
use game::{HEIGHT, TETROMINO_LIMIT, WIDTH};

// UI STUFF
const SQUARE_SIZE: f32 = 27.0;
const MARGIN_LEFT: f32 = SQUARE_SIZE;
const MARGIN_TOP: f32 = SQUARE_SIZE;
const SIDE_PANEL_WIDTH: f32 = SQUARE_SIZE * 5.0;

#[macroquad::main("Tetris")]
async fn main() {
    set_window_size(
        ((WIDTH + 2) as f32 * SQUARE_SIZE) as u32 + SIDE_PANEL_WIDTH as u32,
        ((HEIGHT + 3) as f32 * SQUARE_SIZE) as u32,
    );

    let mut game_state = GameState::Menu;

    let mut piece_chooser = PieceChooser::new(3);

    let mut board = Board::new();
    let mut next_shape = piece_chooser.get_next_piece();
    let mut piece = Piece::new(next_shape);

    board.add_piece(&piece);
    let drop_time = 1.0;
    let mut prev_time = get_time();

    let mut num_tetrominos = 0;

    loop {
        match game_state {
            GameState::Menu => {
                set_window_size(
                    ((WIDTH + 2) as f32 * SQUARE_SIZE) as u32 + SIDE_PANEL_WIDTH as u32,
                    ((HEIGHT + 3) as f32 * SQUARE_SIZE) as u32,
                );

                clear_background(Color::from_rgba(40, 40, 40, 255));
                // shadow
                draw_text(
                    "Tetris",
                    WIDTH as f32 * SQUARE_SIZE / 2.0 - 40.0 + 5.0,
                    HEIGHT as f32 * SQUARE_SIZE / 2.0 + 5.0,
                    100.0,
                    BLACK,
                );
                draw_text(
                    "Tetris",
                    WIDTH as f32 * SQUARE_SIZE / 2.0 - 40.0,
                    HEIGHT as f32 * SQUARE_SIZE / 2.0,
                    100.0,
                    YELLOW,
                );

                // shadow
                draw_text(
                    "Press space to start",
                    WIDTH as f32 * SQUARE_SIZE / 2.0 - 20.0 + 5.0,
                    HEIGHT as f32 * SQUARE_SIZE / 2.0 + 50.0 + 5.0,
                    25.0,
                    BLACK,
                );
                draw_text(
                    "Press space to start",
                    WIDTH as f32 * SQUARE_SIZE / 2.0 - 20.0 + 5.0,
                    HEIGHT as f32 * SQUARE_SIZE / 2.0 + 50.0,
                    25.0,
                    WHITE,
                );

                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }

                next_frame().await
            }
            GameState::Playing => {
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
                    piece = Piece::new(next_shape);

                    // check collision with the new piece
                    if board.is_colliding(&piece) {
                        game_state = GameState::GameOver;
                        continue;
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

                clear_background(Color::from_rgba(40, 40, 40, 255));
                draw_grid(WIDTH, HEIGHT);
                for y in 0..HEIGHT {
                    for x in 0..WIDTH {
                        let color;
                        if let Tetromino::E = board.grid[y][x] {
                            continue;
                        } else {
                            color = board.grid[y][x].get_color();
                        }
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
                    Color::from_rgba(70, 70, 70, 255),
                );
                // "Score" shadow
                draw_text(
                    "Score",
                    side_panel_middle + 1.5,
                    MARGIN_TOP + SQUARE_SIZE + 1.7,
                    35.0,
                    BLACK,
                );
                draw_text(
                    "Score",
                    side_panel_middle,
                    MARGIN_TOP + SQUARE_SIZE + 0.2,
                    35.0,
                    GOLD,
                );
                // score shadow
                draw_text(
                    &board.score.to_string(),
                    side_panel_middle + 1.5,
                    MARGIN_TOP + SQUARE_SIZE * 2.5 + 1.5,
                    35.0,
                    BLACK,
                );
                draw_text(
                    &board.score.to_string(),
                    side_panel_middle,
                    MARGIN_TOP + SQUARE_SIZE * 2.5,
                    35.0,
                    WHITE,
                );

                // display the next pieces
                // draw bounding box for the next pieces
                draw_rectangle(
                    side_panel_margin_left,
                    MARGIN_TOP * 5.0,
                    SIDE_PANEL_WIDTH,
                    SQUARE_SIZE * 8.0,
                    Color::from_rgba(70, 70, 70, 255),
                );
                // "Next" shadow
                draw_text(
                    "Next",
                    side_panel_middle + 10.0 + 1.5,
                    MARGIN_TOP * 5.0 + 1.7 + SQUARE_SIZE,
                    35.0,
                    BLACK,
                );
                draw_text(
                    "Next",
                    side_panel_middle + 10.0,
                    MARGIN_TOP * 5.0 + SQUARE_SIZE,
                    35.0,
                    GOLD,
                );

                let mut next_pieces = piece_chooser.next_pieces.clone();
                next_pieces.reverse();
                for (i, next_piece) in next_pieces.iter().enumerate() {
                    for &(x, y) in &(Piece::new(*next_piece)).get_coords() {
                        let color;
                        if let Tetromino::E = next_piece {
                            continue;
                        } else {
                            color = next_piece.get_color();
                        }

                        // outer rectangle
                        draw_rectangle(
                            side_panel_middle + 10.0 + (x - 3) as f32 * SQUARE_SIZE * 0.75,
                            MARGIN_TOP * 5.0
                                + y as f32 * SQUARE_SIZE * 0.75
                                + SQUARE_SIZE * 2.0 * 0.75
                                + SQUARE_SIZE * i as f32 * 3.0 * 0.75,
                            SQUARE_SIZE * 0.75,
                            SQUARE_SIZE * 0.75,
                            BLACK,
                        );

                        // inner rectangle
                        let inner_size = SQUARE_SIZE * 0.4 * (0.75 / 0.5);
                        let inner_offset = (SQUARE_SIZE * 0.75 - inner_size) / 2.0;
                        draw_rectangle(
                            side_panel_middle
                                + 10.0
                                + (x - 3) as f32 * SQUARE_SIZE * 0.75
                                + inner_offset,
                            MARGIN_TOP * 5.0
                                + y as f32 * SQUARE_SIZE * 0.75
                                + SQUARE_SIZE * 2.0 * 0.75
                                + SQUARE_SIZE * i as f32 * 3.0 * 0.75
                                + inner_offset,
                            inner_size,
                            inner_size,
                            color,
                        );
                    }
                }

                next_frame().await
            }
            GameState::GameOver => {
                set_window_size(
                    ((WIDTH + 2) as f32 * SQUARE_SIZE) as u32 + SIDE_PANEL_WIDTH as u32,
                    ((HEIGHT + 3) as f32 * SQUARE_SIZE) as u32,
                );

                clear_background(Color::from_rgba(40, 40, 40, 255));
                // shadow
                draw_text(
                    "Game over!",
                    WIDTH as f32 * SQUARE_SIZE / 2.0 - 120.0 + 5.0,
                    HEIGHT as f32 * SQUARE_SIZE / 2.0 + 5.0,
                    100.0,
                    BLACK,
                );
                draw_text(
                    "Game over!",
                    WIDTH as f32 * SQUARE_SIZE / 2.0 - 120.0,
                    HEIGHT as f32 * SQUARE_SIZE / 2.0,
                    100.0,
                    RED,
                );

                // score shadow
                draw_text(
                    &format!("Score {}", board.score),
                    WIDTH as f32 * SQUARE_SIZE / 2.0 - 20.0 + 4.0,
                    HEIGHT as f32 * SQUARE_SIZE / 2.0 + 50.0 + 4.0,
                    50.0,
                    BLACK,
                );
                draw_text(
                    &format!("Score {}", board.score),
                    WIDTH as f32 * SQUARE_SIZE / 2.0 - 20.0,
                    HEIGHT as f32 * SQUARE_SIZE / 2.0 + 50.0,
                    50.0,
                    WHITE,
                );

                // shadow
                draw_text(
                    "Press space to restart",
                    WIDTH as f32 * SQUARE_SIZE / 2.0 - 40.0 + 5.0,
                    HEIGHT as f32 * SQUARE_SIZE / 2.0 + 100.0 + 5.0,
                    25.0,
                    BLACK,
                );
                draw_text(
                    "Press space to restart",
                    WIDTH as f32 * SQUARE_SIZE / 2.0 - 40.0 + 5.0,
                    HEIGHT as f32 * SQUARE_SIZE / 2.0 + 100.0,
                    25.0,
                    WHITE,
                );

                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                    board = Board::new();
                    next_shape = piece_chooser.get_next_piece();
                    piece = Piece::new(next_shape);
                    board.add_piece(&piece);
                    prev_time = get_time();
                    num_tetrominos = 0;
                }

                next_frame().await
            }
        }
    }

    println!("Final score: {}", board.score);
}

fn draw_grid(width: usize, height: usize) {
    // inner rectangle fill
    // every alternating line is a different color
    let mut darkenning_factor;
    for y in 0..height {
        for x in 0..width {
            if y % 2 == 0 {
                darkenning_factor = 0.9;
            } else {
                darkenning_factor = 1.0;
            }
            let color = if (x + y) % 2 == 0 {
                Color::from_rgba(
                    (40.0 * darkenning_factor) as u8,
                    (40.0 * darkenning_factor) as u8,
                    (40.0 * darkenning_factor) as u8,
                    255,
                )
            } else {
                Color::from_rgba(
                    (50.0 * darkenning_factor) as u8,
                    (50.0 * darkenning_factor) as u8,
                    (50.0 * darkenning_factor) as u8,
                    255,
                )
            };
            draw_rectangle(
                x as f32 * SQUARE_SIZE + MARGIN_LEFT,
                y as f32 * SQUARE_SIZE + MARGIN_TOP,
                SQUARE_SIZE,
                SQUARE_SIZE,
                color,
            );
        }
    }

    // draw bounding box
    draw_rectangle_lines(
        MARGIN_LEFT,
        MARGIN_TOP,
        width as f32 * SQUARE_SIZE,
        height as f32 * SQUARE_SIZE,
        4.0,
        BLACK,
    );
}
