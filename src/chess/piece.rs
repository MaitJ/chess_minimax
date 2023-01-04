use macroquad::text::{measure_text, TextParams};
use crate::chess::board::{Cell, CELL_SIZE};

use super::r#move::{MovePattern, is_allowed_move, is_in_square_from_origin};

#[derive(Clone, Copy)]
pub enum Side {
    White,
    Black
}

#[derive(Clone, Copy)]
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

    fn generate_allowed_moves(origin: (f32, f32), pattern: Vec<MovePattern>) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = vec![];
        for i in 0..8 {
            for j in 0..8 {
                let to = (i as f32, j as f32);

                pattern.iter()
                    .map(|pattern| is_allowed_move(origin, to, pattern))
                    .for_each(|legal_move| if legal_move {moves.push((i, j))});
            }
        }
        return moves;
    }

    fn generate_king_moves(origin: (f32, f32)) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = vec![];
        for i in 0..8 {
            for j in 0..8 {
                let to = (i as f32, j as f32);

                if is_in_square_from_origin(origin, to) {moves.push((i, j));}
            }
        }
        return moves;
    }
    

    pub fn get_legal_moves(piece: &ChessPiece, origin: (f32, f32)) -> Vec<(usize, usize)> {
        let moves = match piece {
            ChessPiece::King => Self::generate_king_moves(origin),
            ChessPiece::Queen => Self::generate_allowed_moves(origin, vec![MovePattern::Diagonal, 
                                                              MovePattern::Perpendicular, 
                                                              MovePattern::Parallel]),
            ChessPiece::Bishop => Self::generate_allowed_moves(origin, vec![MovePattern::Diagonal]),
            ChessPiece::Rook => Self::generate_allowed_moves(origin, vec![MovePattern::Perpendicular, MovePattern::Parallel]),
            ChessPiece::Knight => unimplemented!(),
            ChessPiece::Pawn => unimplemented!()
        };
        return moves;
    }

}
