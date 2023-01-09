mod chess;
use chess::board::Board;
use chess::board_view::BoardView;
use chess::minimax::opponents_turn;
use chess::piece::Side;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chess".to_owned(),
        fullscreen: false,
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

fn swap_turn(whose_turn: Side) -> Side {
    match whose_turn {
        Side::White => Side::Black,
        Side::Black => Side::White
    }
}


#[macroquad::main(window_conf)]
async fn main() -> Result<(), FontError> {

    let mut current_resolution = (screen_width(), screen_height());
    let mut chess_board = Board::new(current_resolution.0, current_resolution.1).await;
    let mut board_view = BoardView::new(chess_board.text_spacing, chess_board.text_params);

    let background_color = Color::from_rgba(96, 96, 96, 255);

    let player_side = Side::White;
    //-1 or 1
    let mut whose_turn: Side = Side::White;

    loop {
        let screen_width = screen_width();
        let screen_height = screen_height();
        if current_resolution.0 != screen_width || current_resolution.1 != screen_height {
            chess_board.rescale(screen_width, screen_height);
            println!("Screen size changed");
            current_resolution = (screen_width, screen_height);
        }
        clear_background(background_color);

        chess_board.draw();

        if is_mouse_button_pressed(MouseButton::Left) {
            let piece_pickup = BoardView::was_piece_hit(chess_board.get_board_state(), &whose_turn);
            if let Some(piece_pickup) = piece_pickup {
                board_view.pick_up_piece(piece_pickup);
            }
        }

        if board_view.player_input(&mut chess_board) {
            whose_turn = swap_turn(whose_turn);
            println!("whose_turn: {:?}", whose_turn);
        }


        if whose_turn == Side::Black {
            //Clone board before
            let simulated_board = chess_board.clone();
            whose_turn = opponents_turn(&mut chess_board, simulated_board, whose_turn);
        }
        //swap_turn
        next_frame().await
    }
}
