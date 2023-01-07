use macroquad::prelude::*;
use super::board::{CELL_SIZE, Board, CellPiece};
use super::{board::Cell, piece::{ChessPiece, Side}};

pub struct PiecePickup {
    pub last_pos: (i8, i8),
    pub side: Side,
    pub piece: ChessPiece
}

pub struct BoardView {
    picked_up_piece: Option<PiecePickup>,
    text_spacing: f32,
    text_params: TextParams
}

impl BoardView {
    pub fn new(text_spacing: f32, text_params: TextParams) -> BoardView {
        BoardView { 
            picked_up_piece: None,
            text_spacing,
            text_params
        }
    }
    pub fn pick_up_piece(&mut self, piece_pickup: PiecePickup) {
        println!("Picked up piece: {:?}, last_pos: ({}, {})", piece_pickup.piece, piece_pickup.last_pos.0, piece_pickup.last_pos.1);
        self.picked_up_piece = Some(piece_pickup);
    }

    pub fn render_picked_up_piece(&self, piece_pickup: &PiecePickup) {
        let (mouse_x, mouse_y) = mouse_position();

        let piece_str = ChessPiece::get_char(&piece_pickup.piece, &piece_pickup.side).to_string();
        let half_cell = CELL_SIZE / 2.0;
        draw_text_ex(&piece_str, mouse_x - half_cell + self.text_spacing, mouse_y + half_cell, self.text_params);

        if is_mouse_button_released(MouseButton::Left) {
            println!("Mouse button was released");
        }
    }
    
    pub fn was_piece_hit(cell_piece: Option<CellPiece>) -> Option<PiecePickup> {
        if let Some(cell_piece) = cell_piece {
            if let Some(piece_side) = cell_piece.cell.get_piece_side() {
                let piece_pickup: PiecePickup = PiecePickup {
                    last_pos: cell_piece.position,
                    piece: piece_side.0,
                    side: piece_side.1
                };
                return Some(piece_pickup);
            }
        }
        None
    }

    pub fn player_input(&self, board: &mut Board) {
        match &self.picked_up_piece {
            Some(piece_pickup) => self.render_picked_up_piece(&piece_pickup),
            None => ()
        }
    }
}

