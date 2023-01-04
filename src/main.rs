mod chess;
use chess::board::Board;
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

    let background_color = Color::from_rgba(96, 96, 96, 255);

    if chess_board.move_piece((1, 0), (2, 2)) {
        println!("can move");
        chess_board.piece_at((2, 2));
    }


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
        next_frame().await
    }
}
