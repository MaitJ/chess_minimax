use std::rc::Rc;

use macroquad::text::{measure_text, TextParams};
use crate::chess::board::CELL_SIZE;
use super::board::Cell;
use super::r#move::{MovePattern, is_allowed_move, is_in_square_from_origin, is_in_l_from_origin, NUM_OF_SQUARES_TO_EDGE, tuple_to_square_index, RAY_INCREMENTS};
use super::r#move::{MovePattern::N, MovePattern::NW, MovePattern::W, MovePattern::SW, MovePattern::S, MovePattern::SE, MovePattern::E, MovePattern::NE};

#[derive(Clone, Copy)]
pub enum Side {
    White,
    Black
}

#[derive(Clone, Copy, Debug)]
pub enum ChessPiece {
    King = 0,
    Queen = 1,
    Bishop = 2,
    Pawn = 3,
    Knight = 4,
    Rook = 5
}

impl ChessPiece {
    pub fn get_char(piece: &ChessPiece, side: &Side) -> char {
        if let Side::White = side {
            return ChessPiece::white_piece_chars(piece);
        } else {
            return ChessPiece::black_piece_chars(piece);
        }
    }

    fn white_piece_chars(piece: &ChessPiece) -> char {
        return match piece {
            ChessPiece::King => '\u{2654}',
            ChessPiece::Queen => '\u{2655}',
            ChessPiece::Rook => '\u{2656}',
            ChessPiece::Bishop => '\u{2657}',
            ChessPiece::Knight => '\u{2658}',
            ChessPiece::Pawn => '\u{2659}'
        }
    }

    fn black_piece_chars(piece: &ChessPiece) -> char {
        return match piece {
            ChessPiece::King => '\u{265A}',
            ChessPiece::Queen => '\u{265B}',
            ChessPiece::Rook => '\u{265C}',
            ChessPiece::Bishop => '\u{265D}',
            ChessPiece::Knight => '\u{265E}',
            ChessPiece::Pawn => '\u{265F}'
        }
    }
    pub fn get_center_offset(piece: &ChessPiece, text_params: &TextParams) -> f32 {
        let piece_str = ChessPiece::get_char(piece, &Side::White).to_string();
        let text_dimensions = measure_text(&piece_str, Some(text_params.font), text_params.font_size, text_params.font_scale);
        return (CELL_SIZE - text_dimensions.width) / 2.0;
    }

    fn generate_allowed_moves(origin: (i8, i8), patterns: Vec<MovePattern>, board: Rc<Vec<Vec<Cell>>>) -> Vec<(i8, i8)> {
        let mut moves: Vec<(i8, i8)> = vec![];
        for pattern in patterns {
            for square in 0..NUM_OF_SQUARES_TO_EDGE[pattern.clone() as usize][tuple_to_square_index(origin) as usize] {
                let direction_increment = RAY_INCREMENTS[pattern.clone() as usize].clone();
                
                let dir_vec = (direction_increment.0 * square, direction_increment.1 * square);
                let to_square = (origin.0 + dir_vec.0, origin.1 + dir_vec.1);

                if board[to_square.1 as usize][to_square.0 as usize].is_occupied() { break; }

                moves.push(to_square);
            }
        }

        return moves;
    }

    fn generate_king_moves(origin: (i8, i8)) -> Vec<(i8, i8)> {
        let mut moves: Vec<(i8, i8)> = vec![];
        for i in 0..8 {
            for j in 0..8 {
                let to = (i, j);
                if is_in_square_from_origin(origin, to) {moves.push((i, j));}
            }
        }
        return moves;
    }

    fn generate_knight_moves(origin: (i8, i8)) -> Vec<(i8, i8)> {
        let mut moves: Vec<(i8, i8)> = vec![];
        for i in 0..8 {
            for j in 0..8 {
                let to = (i, j);
                if is_in_l_from_origin(origin, to) {moves.push((i, j));}
            }
        }
        return moves;
    }

    fn generate_pawn_moves(origin: (i8, i8), side: &Side) -> Vec<(i8, i8)> {
        let (x, y) = origin;
        match side {
            Side::White => vec![(x, y - 1)],
            Side::Black => vec![(x, y + 1)]
        }
    }

    pub fn get_legal_moves(occupied_cells: Vec<Vec<bool>>, pseudolegal_moves: Vec<(i8, i8)>) -> Vec<(i8, i8)> {
        let legal_moves: Vec<(i8, i8)> = vec![];
        
        return legal_moves;
    }

    pub fn get_pseudolegal_moves(board: Rc<Vec<Vec<Cell>>>, origin: (i8, i8), piece: &ChessPiece, side: &Side) -> Vec<(i8, i8)> {
        let moves = match piece {
            ChessPiece::King => Self::generate_king_moves(origin),
            ChessPiece::Queen => Self::generate_allowed_moves(origin, vec![N, NE, E, SE, S, SW, W, NW], board),
            ChessPiece::Bishop => Self::generate_allowed_moves(origin, vec![NW, NE, SW, SE], board),
            ChessPiece::Rook => Self::generate_allowed_moves(origin, vec![N, E, W, S], board),
            ChessPiece::Knight => Self::generate_knight_moves(origin),
            ChessPiece::Pawn => Self::generate_pawn_moves(origin, side)
        };
        return moves;
    }

}
