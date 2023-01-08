use super::board;
use std::cell::RefCell;
use macroquad::prelude::*;
use super::board::{CELL_SIZE, Board, CellPiece};
use super::{piece::{ChessPiece, Side}};

const LEGAL_MOVE_CELL_COLOR: Color = Color::new(0.31, 0.54, 0.38, 0.7);

#[derive(Clone)]
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
        println!("Picked up piece: {:?}, last_pos: ({}, {})", piece_pickup.piece, piece_pickup.last_pos.1, piece_pickup.last_pos.0);
        self.picked_up_piece = Some(piece_pickup);
    }

    pub fn render_picked_up_piece(&self, piece_pickup: &PiecePickup) {
        let (mouse_x, mouse_y) = mouse_position();

        let piece_str = ChessPiece::get_char(&piece_pickup.piece, &piece_pickup.side).to_string();
        let half_cell = CELL_SIZE / 2.0;
        draw_text_ex(&piece_str, mouse_x - half_cell + self.text_spacing, mouse_y + half_cell, self.text_params);
    }
    
    pub fn highlight_legal_moves(legal_moves: &Vec<(i8, i8)>, board: &Vec<Vec<board::Cell>>) {
        for legal_move in legal_moves {
            let cell = &board[legal_move.1 as usize][legal_move.0 as usize];
            cell.highlight_cell(LEGAL_MOVE_CELL_COLOR);
        }
    }
    
    pub fn was_piece_hit(board: &Vec<Vec<board::Cell>>) -> Option<PiecePickup> {
        let cell_piece = BoardView::check_player_input(board);
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

    pub fn drop_piece(&mut self) {
        self.picked_up_piece = None;
    }

    pub fn check_for_new_position(&mut self, board: &mut Board, piece_pickup: PiecePickup) {
        if is_mouse_button_released(MouseButton::Left) {
            let board_state = board.get_board_state();
            let cell_piece = Self::check_player_input(board_state);
            match cell_piece {
                Some(piece) => {
                    println!("New pos: ({}, {})", piece.position.1, piece.position.0);
                    if board.move_piece(piece_pickup.last_pos, piece.position) {
                        self.drop_piece();
                    }
                },
                None => ()
            };
        }
    }

    pub fn check_player_input(board_state: &Vec<Vec<board::Cell>>) -> Option<CellPiece> {
        for i in 0..8 {
            for j in 0..8 {
                let cell = &board_state[j as usize][i as usize];
                if cell.get_rectangle().contains_point(mouse_position()) {
                    return Some(CellPiece {
                        cell,
                        position: (i, j)
                    });
                }
            }
        }

        None
    }

    pub fn player_input(&mut self, board: &mut Board) {
        match &self.picked_up_piece {
            Some(piece_pickup) => {
                let board_state = board.get_board_state();
                let legal_moves = ChessPiece::get_pseudolegal_moves(board_state, piece_pickup.last_pos, &piece_pickup.piece, &piece_pickup.side);
                self.render_picked_up_piece(&piece_pickup);
                Self::highlight_legal_moves(&legal_moves, board_state);

                self.check_for_new_position(board, piece_pickup.clone());
            },
            None => ()
        }
    }
}

