use std::{rc::Rc, borrow::BorrowMut};

use macroquad::prelude::*;
use crate::chess::piece::{ChessPiece, Side};

pub const CELL_SIZE: f32 = 80.0;
const GRID_SIZE: u8 = 8;

const CELL_COLORS: (Color, Color) = (Color::new(0.44314, 0.55294, 0.32941, 1.0), 
                                     Color::new(0.92549, 0.92549, 0.83529, 1.0));

#[derive(Debug)]
enum CellState {
    Empty,
    Piece(ChessPiece)
}

#[derive(Debug)]
struct Rectangle {
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32
}

impl Rectangle {
    fn new(tl_x: f32, tl_y: f32, br_x: f32, br_y: f32) -> Rectangle {
        Rectangle {
            min_x: tl_x,
            min_y: tl_y,
            max_x: br_x,
            max_y: br_y
        }
    }

    fn contains_point(&self, point: (f32, f32)) -> bool {
        if (point.0 >= self.min_x && point.0 <= self.max_x) &&
           (point.1 >= self.min_y && point.1 <= self.max_y) {
               return true;
           }
        return false;
    }
}

pub struct CellPiece<'a> {
    pub cell: &'a Cell,
    pub position: (i8, i8)
}

pub struct Cell {
    state: CellState,
    side: Option<Side>,

    position: (f32, f32),
    color: &'static Color,
}

impl Cell {
    fn new(position: (f32, f32), color: &'static Color) -> Cell {
        Cell {
            position,
            color,

            state: CellState::Empty,
            side: None
        }
    }
    fn get_rectangle(&self) -> Rectangle {
        let br: (f32, f32) = (self.position.0 + CELL_SIZE, self.position.1 + CELL_SIZE);
        Rectangle::new(self.position.0, self.position.1,
                       br.0, br.1)
    }

    pub fn modify_cell(&mut self, state: CellState, side: Option<Side>) {
        self.state = state;
        self.side = side;
    }

    pub fn is_occupied(&self) -> bool {
        match self.state {
            CellState::Piece(_) => true,
            CellState::Empty => false
        }
    }

    pub fn get_piece_side(&self) -> Option<(ChessPiece, Side)> {
        match self.state {
            CellState::Piece(piece) => {
                match self.side {
                    Some(side) => Some((piece, side)),
                    None => None
                }
            },
            CellState::Empty => None
        }
    }
}

pub struct Board {
    board: Rc<Vec<Vec<Cell>>>,
    //TODO Should probably move to BoardView
    pub text_params: TextParams,
    pub text_spacing: f32
}


impl Board {
    fn add_pieces(&mut self) {
        let main_pieces_order: [ChessPiece; 8] = [
            ChessPiece::Rook,
            ChessPiece::Knight,
            ChessPiece::Bishop,
            ChessPiece::King,
            ChessPiece::Queen,
            ChessPiece::Bishop,
            ChessPiece::Knight,
            ChessPiece::Rook,
        ];
        let pawn_rows: [(Side, usize); 2] = [
            (Side::Black, 1),
            (Side::White, 6)
        ];
        let main_rows: [(Side, usize); 2] = [
            (Side::Black, 0),
            (Side::White, 7)
        ];

        let board_ref = Rc::get_mut(&mut self.board).expect("Couldn't borrow board as mutable");

        for (side, row_i) in main_rows {
            let row = &mut board_ref[row_i];
            for (i, cell) in row.iter_mut().enumerate() {
                cell.state = CellState::Piece(main_pieces_order[i].clone());
                cell.side = Some(side.clone());
            }
        }

        for (side, row_i) in pawn_rows {
            let row = &mut board_ref[row_i];
            for cell in row {
                cell.state = CellState::Piece(ChessPiece::Pawn);
                cell.side = Some(side.clone());
            }
        }
    }

    async fn init_text_params() -> Result<TextParams, FontError> {
        let font = load_ttf_font("DejaVuSans.ttf").await?;
        let text_params = TextParams {
            color: BLACK,
            font,
            font_size: 72,
            ..Default::default()
        };
        Ok(text_params)
    }

    pub async fn new(screen_width: f32, screen_height: f32) -> Board {
        //TOOD Error handle this better
        let text_params = Self::init_text_params().await.expect("Failed to open font");

        let mut board: Rc<Vec<Vec<Cell>>> = Rc::new(vec![]);
        //TODO Definately have to replace get_mut
        let mut_board = Rc::get_mut(&mut board).expect("Couldn't get board as mutable");
        let start_x = (screen_width - GRID_SIZE as f32 * CELL_SIZE) / 2.0;
        let start_y = (screen_height - GRID_SIZE as f32 * CELL_SIZE) / 2.0;

        let mut color_switch = true;
        for i in 0..GRID_SIZE {
            let cur_y = start_y + (i as f32 * CELL_SIZE);
            let mut row: Vec<Cell> = vec![];
            for j in 0..GRID_SIZE {
                let cur_x = start_x + (j as f32 * CELL_SIZE);
                if !color_switch {
                    row.push(Cell::new((cur_x, cur_y), &CELL_COLORS.0));
                } else {
                    row.push(Cell::new((cur_x, cur_y), &CELL_COLORS.1));
                }
                color_switch = !color_switch;
            }
            color_switch = !color_switch;
            mut_board.push(row);
        }

        let mut board = Board {
            board,
            text_params,
            text_spacing: ChessPiece::get_center_offset(&ChessPiece::Pawn, &text_params)
        };
        board.add_pieces();
        return board;
    }

    pub fn piece_at(&self, point: (i8, i8)) {
        let cell = &self.board[point.0 as usize][point.1 as usize];
        println!("piece: {:?}", cell.state);
    }
    

    pub fn get_board_state(&self) -> Rc<Vec<Vec<Cell>>> {
        unimplemented!();
    }

    pub fn get_board_state_bitfield(&self) -> Vec<Vec<bool>> {
        let mut bitfield: Vec<Vec<bool>> = vec![];
        for row in self.board.iter() {
            let mut bitfield_row: Vec<bool> = vec![];
            for cell in row {
                match cell.state {
                    CellState::Empty => bitfield_row.push(false),
                    CellState::Piece(_) => bitfield_row.push(true)
                }
            }
            bitfield.push(bitfield_row);
        }
        return bitfield;
    }

    //TODO Maybe make this return a Result
    pub fn move_piece(&mut self, origin: (i8, i8), to: (i8, i8)) -> bool {
        let board_ref = &mut self.board;
        let origin_cell: &mut Cell = &mut board_ref[origin.1 as usize][origin.0 as usize];

        let side = match origin_cell.side {
            Some(side) => side,
            None => return false
        };
        
        match origin_cell.state {
            CellState::Piece(piece) => {
                let pseudolegal_moves = ChessPiece::get_pseudolegal_moves(Rc::clone(board_ref), origin, &piece, &Side::White);
                println!("legal_moves: {:?}", pseudolegal_moves);
                if pseudolegal_moves.contains(&to) {
                    origin_cell.modify_cell(CellState::Empty, None);
                    origin_cell.state = CellState::Empty;
                    origin_cell.side = None;

                    //TODO Add text_spacing
                    board_ref[to.1 as usize][to.0 as usize].state = CellState::Piece(piece);
                    board_ref[to.1 as usize][to.0 as usize].side = Some(side);
                    return true;
                }
                false
            },
            CellState::Empty => false
        }
    }

    pub fn rescale(&mut self, screen_width: f32, screen_height: f32) {
        let start_x = (screen_width - GRID_SIZE as f32 * CELL_SIZE) / 2.0;
        let start_y = (screen_height - GRID_SIZE as f32 * CELL_SIZE) / 2.0;

        let board_ref = Rc::get_mut(&mut self.board).expect("Couldn't borrow board as mutable");
        for (i, row) in board_ref.iter_mut().enumerate() {
            let cur_y = start_y + (i as f32 * CELL_SIZE);
            for (j, cell) in row.iter_mut().enumerate() {
                let cur_x = start_x + (j as f32 * CELL_SIZE);
                cell.position = (cur_x, cur_y);
            }
        }
    }

    fn draw_cell(&self, cell: &Cell) {
        draw_rectangle(cell.position.0, cell.position.1, CELL_SIZE, CELL_SIZE, *cell.color);

        if let CellState::Piece(piece) = cell.state {
            if let Some(side) = cell.side {
                let piece_str = ChessPiece::get_char(&piece, &side).to_string();
                draw_text_ex(&piece_str, cell.position.0 + self.text_spacing, cell.position.1 + CELL_SIZE, self.text_params);
            }
        }
    }

    pub fn check_player_input<'a>(&'a self) -> Option<CellPiece> {
        let board_state = &self.board;

        for i in 0..8 {
            for j in 0..8 {
                let cell = &board_state[j as usize][i as usize];
                if cell.get_rectangle().contains_point(mouse_position()) && 
                    is_mouse_button_pressed(MouseButton::Left) {
                    return Some(CellPiece {
                        cell,
                        position: (j, i)
                    });
                }
            }
        }

        None
    }

    pub fn highlight_cell(&self, position: (i8, i8), color: Color) {
        unimplemented!();
    }

    pub fn draw(&self) {
        for row in self.board.iter() {
            for cell in row {
                self.draw_cell(cell);
            }
        }
    }
}
