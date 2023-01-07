mod chess;
use chess::board::Board;
use chess::board_view::{BoardView, PiecePickup};
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


#[macroquad::main(window_conf)]
async fn main() -> Result<(), FontError> {

    let mut current_resolution = (screen_width(), screen_height());
    let mut chess_board = Board::new(current_resolution.0, current_resolution.1).await;
    let mut board_view = BoardView::new(chess_board.text_spacing, chess_board.text_params);

    let background_color = Color::from_rgba(96, 96, 96, 255);

    if chess_board.move_piece((1, 0), (2, 2)) {
        println!("can move");
        chess_board.piece_at((2, 2));
    }

    chess_board.move_piece((3, 6), (3, 5));
    chess_board.move_piece((4, 7), (0, 3));
    println!("{:?}", chess_board.get_board_state_bitfield());


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

        let cell_piece = chess_board.check_player_input();
        let piece_pickup = BoardView::was_piece_hit(cell_piece);

        if let Some(piece_pickup) = piece_pickup {
            board_view.pick_up_piece(piece_pickup);
        }

        board_view.player_input(&mut chess_board);
        next_frame().await
    }
}
