use crate::chess::piece::{ChessPiece, Side};
use macroquad::prelude::*;

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

pub struct Cell {
    state: CellState,
    side: Option<Side>,

    position: (f32, f32),
    color: &'static Color,
    text_offset: f32
}

impl Cell {
    fn new(position: (f32, f32), color: &'static Color, text_offset: f32) -> Cell {
        Cell {
            position,
            color,
            text_offset,

            state: CellState::Empty,
            side: None
        }
    }
    fn get_rectangle(&self) -> Rectangle {
        let br: (f32, f32) = (self.position.0 + CELL_SIZE, self.position.1 + CELL_SIZE);
        Rectangle::new(self.position.0, self.position.1,
                       br.0, br.1)
    }
}

pub struct Board {
    board: Vec<Vec<Cell>>,
    text_params: TextParams
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

        for (side, row_i) in main_rows {
            let row = &mut self.board[row_i];
            for (i, cell) in row.into_iter().enumerate() {
                cell.state = CellState::Piece(main_pieces_order[i].clone());
                cell.side = Some(side.clone());
                cell.text_offset = ChessPiece::get_center_offset(&main_pieces_order[i], &self.text_params);
            }
        }

        for (side, row_i) in pawn_rows {
            let row = &mut self.board[row_i];
            for cell in row {
                cell.state = CellState::Piece(ChessPiece::Pawn);
                cell.side = Some(side.clone());
                cell.text_offset = ChessPiece::get_center_offset(&ChessPiece::Pawn, &self.text_params);
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

        let mut board: Vec<Vec<Cell>> = vec![];
        let start_x = (screen_width - GRID_SIZE as f32 * CELL_SIZE) / 2.0;
        let start_y = (screen_height - GRID_SIZE as f32 * CELL_SIZE) / 2.0;

        let mut color_switch = true;
        for i in 0..GRID_SIZE {
            let cur_y = start_y + (i as f32 * CELL_SIZE);
            let mut row: Vec<Cell> = vec![];
            for j in 0..GRID_SIZE {
                let cur_x = start_x + (j as f32 * CELL_SIZE);
                if !color_switch {
                    row.push(Cell::new((cur_x, cur_y), &CELL_COLORS.0, 0.0));
                } else {
                    row.push(Cell::new((cur_x, cur_y), &CELL_COLORS.1, 0.0));
                }
                color_switch = !color_switch;
            }
            color_switch = !color_switch;
            board.push(row);
        }

        let mut board = Board {
            board,
            text_params
        };
        board.add_pieces();
        return board;
    }

    pub fn piece_at(&self, point: (i8, i8)) {
        let cell = &self.board[point.0 as usize][point.1 as usize];
        println!("piece: {:?}", cell.state);
    }

    //TODO Maybe make this return a Result
    pub fn move_piece(&mut self, origin: (i8, i8), to: (i8, i8)) -> bool {
        let origin_cell: &mut Cell = &mut self.board[origin.1 as usize][origin.0 as usize];

        let side = match origin_cell.side {
            Some(side) => side,
            None => return false
        };
        
        match origin_cell.state {
            CellState::Piece(piece) => {
                let legal_moves = ChessPiece::get_legal_moves(origin, &piece, &Side::White);
                println!("legal_moves: {:?}", legal_moves);
                if legal_moves.contains(&to) {
                    origin_cell.state = CellState::Empty;
                    origin_cell.side = None;

                    //TODO Add text_spacing
                    self.board[to.1 as usize][to.0 as usize].state = CellState::Piece(piece);
                    self.board[to.1 as usize][to.0 as usize].side = Some(side);
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

        for (i, row) in &mut self.board.iter_mut().enumerate() {
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
                draw_text_ex(&piece_str, cell.position.0 + cell.text_offset, cell.position.1 + CELL_SIZE, self.text_params);
            }
        }
    }

    fn check_player_input(cell: &Cell) {
        if cell.get_rectangle().contains_point(mouse_position()) {
        }
    }

    pub fn draw(&self) {
        for row in &self.board {
            for cell in row {
                Self::check_player_input(&cell);
                self.draw_cell(cell);
            }
        }
    }

}
