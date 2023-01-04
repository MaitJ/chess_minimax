use macroquad::text::{measure_text, TextParams};
use crate::chess::board::CELL_SIZE;

use super::r#move::{MovePattern, is_allowed_move, is_in_square_from_origin, is_in_l_from_origin};

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

    fn generate_allowed_moves(origin: (i8, i8), pattern: Vec<MovePattern>) -> Vec<(i8, i8)> {
        let mut moves: Vec<(i8, i8)> = vec![];
        for i in 0..8 {
            for j in 0..8 {
                let to = (i, j);
                pattern.iter()
                    .map(|pattern| is_allowed_move(origin, to, pattern))
                    .for_each(|legal_move| if legal_move {moves.push(to)});
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

    pub fn get_legal_moves(origin: (i8, i8), piece: &ChessPiece, side: &Side) -> Vec<(i8, i8)> {
        let moves = match piece {
            ChessPiece::King => Self::generate_king_moves(origin),
            ChessPiece::Queen => Self::generate_allowed_moves(origin, vec![MovePattern::Diagonal, 
                                                              MovePattern::Perpendicular, 
                                                              MovePattern::Parallel]),
            ChessPiece::Bishop => Self::generate_allowed_moves(origin, vec![MovePattern::Diagonal]),
            ChessPiece::Rook => Self::generate_allowed_moves(origin, vec![MovePattern::Perpendicular, MovePattern::Parallel]),
            ChessPiece::Knight => Self::generate_knight_moves(origin),
            ChessPiece::Pawn => Self::generate_pawn_moves(origin, side)
        };
        return moves;
    }

}
