use macroquad::prelude::*;

use crate::constants::*;
use crate::game::{Piece, Tetromino};

pub fn draw_tetris_grid(width: usize, height: usize) {
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

pub fn draw_menu() {
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
}

pub fn draw_game_over(score: u32) {
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
        &format!("Score {}", score),
        WIDTH as f32 * SQUARE_SIZE / 2.0 - 20.0 + 4.0,
        HEIGHT as f32 * SQUARE_SIZE / 2.0 + 50.0 + 4.0,
        50.0,
        BLACK,
    );
    draw_text(
        &format!("Score {}", score),
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
}

pub fn draw_tetro(tetro: &Tetromino, x: usize, y: usize) {
    let color;
    if let Tetromino::E = tetro {
        return;
    } else {
        color = tetro.get_color();
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

pub fn draw_score(score: u32) {
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
        &score.to_string(),
        side_panel_middle + 1.5,
        MARGIN_TOP + SQUARE_SIZE * 2.5 + 1.5,
        35.0,
        BLACK,
    );
    draw_text(
        &score.to_string(),
        side_panel_middle,
        MARGIN_TOP + SQUARE_SIZE * 2.5,
        35.0,
        WHITE,
    );
}

pub fn draw_next_pieces(next_pieces: &[Tetromino]) {
    // display the next pieces
    let side_panel_middle = WIDTH as f32 * SQUARE_SIZE + SIDE_PANEL_WIDTH / 2.0;
    let side_panel_margin_left = side_panel_middle - SIDE_PANEL_WIDTH / 5.0;

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

    for (i, next_piece) in next_pieces.iter().rev().enumerate() {
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
                side_panel_middle + 10.0 + (x - 3) as f32 * SQUARE_SIZE * 0.75 + inner_offset,
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
}
