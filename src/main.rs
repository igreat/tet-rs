use macroquad::{miniquad::window::set_window_size, prelude::*};
use std::{thread::sleep, time::SystemTime};

mod constants;
mod draw;
mod game;
mod player;

use constants::*;
use draw::*;
use game::*;
use player::{Player, RandomPlayer};

const MANUAL: bool = true;

#[macroquad::main("Tetris")]
async fn main() {
    set_window_size(
        ((WIDTH + 2) as f32 * SQUARE_SIZE) as u32 + SIDE_PANEL_WIDTH as u32,
        ((HEIGHT + 3) as f32 * SQUARE_SIZE) as u32,
    );

    let mut game_state = GameState::Menu;

    let mut piece_chooser = PieceChooser::new(3);
    let player = RandomPlayer;
    let mut chosen_moves = Vec::new();

    let mut board = Board::new();
    let mut next_shape = piece_chooser.get_next_piece();
    let mut piece = Piece::new(next_shape);

    board.add_piece(&piece);
    let drop_time = 1.0;
    let mut prev_time = get_time();

    let mut num_tetrominos = 0;

    loop {
        set_window_size(
            ((WIDTH + 2) as f32 * SQUARE_SIZE) as u32 + SIDE_PANEL_WIDTH as u32,
            ((HEIGHT + 3) as f32 * SQUARE_SIZE) as u32,
        );
        clear_background(Color::from_rgba(40, 40, 40, 255));
        match game_state {
            GameState::Menu => {
                draw_menu();
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }
            }
            GameState::Playing => {
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
                    if board.will_collide(&piece) {
                        game_state = GameState::GameOver;
                        continue;
                    }
                    board.add_piece(&piece);

                    board.just_dropped = false;

                    num_tetrominos += 1;
                }

                if MANUAL {
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
                } else {
                    if chosen_moves.len() > 0 {
                        board.move_piece(&mut piece, chosen_moves[0]);
                        chosen_moves.remove(0);
                    } else {
                        chosen_moves = player.choose_moves(&board, &piece);
                    }
                }

                if get_time() - prev_time > drop_time {
                    board.move_piece(&mut piece, Move::Down);
                    prev_time = get_time();
                }

                draw_tetris_grid(WIDTH, HEIGHT);
                for y in 0..HEIGHT {
                    for x in 0..WIDTH {
                        draw_tetro(&board.grid[y][x], x, y);
                    }
                }

                draw_score(board.score);
                draw_next_pieces(&piece_chooser.next_pieces);
            }
            GameState::GameOver => {
                draw_game_over(board.score);

                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                    board = Board::new();
                    next_shape = piece_chooser.get_next_piece();
                    piece = Piece::new(next_shape);
                    board.add_piece(&piece);
                    prev_time = get_time();
                    num_tetrominos = 0;
                }
            }
        }
        // add a delay to the game loop if not in manual mode
        if !MANUAL {
            sleep(std::time::Duration::from_millis(100));
        }

        next_frame().await
    }

    println!("Final score: {}", board.score);
}
