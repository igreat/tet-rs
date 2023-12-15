use crate::game::*;
use rand::Rng;

pub trait Player {
    fn choose_moves(&self, board: &Board, piece: &Piece) -> Vec<Move>;
}

pub struct RandomPlayer;

impl Player for RandomPlayer {
    fn choose_moves(&self, board: &Board, piece: &Piece) -> Vec<Move> {
        let mut moves = Vec::new();
        let mut rng = rand::thread_rng();
        // let mut piece = piece.clone();
        // let mut board = board.clone();

        // get a bunch of random moves (even invalid ones)
        for _ in 0..10 {
            let move_ = match rng.gen_range(0..5) {
                0 => Move::Left,
                1 => Move::Right,
                2 => Move::Down,
                3 => Move::Rotate,
                4 => Move::Drop,
                _ => panic!("random number generator is broken"),
            };
            moves.push(move_);
        }

        // return all
        moves
    }
}
