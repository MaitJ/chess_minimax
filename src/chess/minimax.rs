use macroquad::rand;

use crate::{Side, Board, swap_turn};
use super::board::{Cell, Move};
use super::{board::BoardPiece, piece::ChessPiece};

struct MoveRating {
    from: (i8, i8),
    to: (i8, i8),
    rating: i32
}

fn best_move(mut moves: Vec<MoveRating>, whose_turn: Side) -> MoveRating {
    let random = rand::gen_range(0, moves.len());
    let mut best_move_rating: MoveRating = moves.remove(random);
    let best_move = match whose_turn {
        //Max = 1
        Side::White => {
            moves.into_iter()
                .for_each(|move_rating| if move_rating.rating > best_move_rating.rating {best_move_rating= move_rating})
        }
        //Min = -1
        Side::Black => {
            moves.into_iter()
                .for_each(|move_rating| if move_rating.rating < best_move_rating.rating {best_move_rating= move_rating})
        }
    };
    best_move_rating
}

static mut EVALUATED_MOVES: i32 = 0;
static mut MINIMAX_CALLS: i32 = 0;

fn evaluate_move(l_move: &Move, board: &Board, whose_turn: Side) -> MoveRating {
    unsafe {
        EVALUATED_MOVES += 1;
    }
    MoveRating {
        from: l_move.from,
        to: l_move.to,
        rating: Board::evaluate_board_score(board.get_board_state(), whose_turn) * -1
    }
}

fn minimax(depth: u8, l_move: &Move, whose_turn: Side, board: &mut Board) -> MoveRating {
    unsafe {
        MINIMAX_CALLS += 1;
    }
    if depth == 0 {return evaluate_move(l_move, board, whose_turn);}

    let all_moves_for_side = board.get_all_moves_for_side(whose_turn);

    let swapped_turn = swap_turn(whose_turn);
    let mut rated_moves: Vec<MoveRating> = vec![];
    for possible_move in all_moves_for_side {
        board.make_move(&possible_move);
        let best_move = minimax(depth - 1, &possible_move, swapped_turn, board);
        board.unmake_move(&possible_move);
        rated_moves.push(MoveRating {
            from: possible_move.from,
            to: possible_move.to,
            rating: best_move.rating
        });
    }
    if rated_moves.len() == 0 {return MoveRating {from: (0, 0), to: (0, 0), rating: 0};}
    best_move(rated_moves, whose_turn)
}

pub fn opponents_turn(board: &mut Board, mut simulated_board: Board, whose_turn: Side) -> Side {
    unsafe {
        EVALUATED_MOVES = 0;
        MINIMAX_CALLS = 0;
    }

    let mut rated_moves: Vec<MoveRating> = vec![];
    let blank_move = Move {
        from: (0, 0),
        to: (0, 0)
    };
    let last_move = std::mem::replace(&mut board.last_move, blank_move);

    let best_move = minimax(10, &last_move, whose_turn, &mut simulated_board);
    unsafe {
        println!("Evaluated_moves: {}", EVALUATED_MOVES);
        println!("Minimax_calls: {}", MINIMAX_CALLS);
    }
    //let best_move = best_move(rated_moves, whose_turn);

    println!("best_move, from: ({}, {}), to: ({}, {})", best_move.from.0, best_move.from.1, best_move.to.0, best_move.to.1);
    board.move_piece(best_move.from, best_move.to, whose_turn);
    
    swap_turn(whose_turn)
    //get_all_legal_moves_for_player();
    //get the best step and play it.
}
