use std::cmp::min;

#[derive(Clone)]
pub enum MovePattern {
    N = 0,
    NE = 1,
    E = 2,
    SE = 3,
    S = 4,
    SW = 5,
    W = 6,
    NW = 7
}
pub const RAY_INCREMENTS: [(i8, i8); 8] = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];

pub const fn min_n(n1: i8, n2: i8) -> i8 {
    if n1 < n2 {
        return n1;
    } else {
        return n2;
    }
}

pub const fn square_index_to_tuple(square_index: i8) -> (i8, i8) {
    return ((square_index % 8) + 1, square_index / 8);
}

pub const fn tuple_to_square_index(tuple: (i8, i8)) -> i8 {
    return tuple.0 + (tuple.1 * 8);
}

const fn compute_nr_of_squares_to_edge() -> [[i8; 64]; 8] {
    let mut squares_to_edge: [[i8; 64]; 8] = [[0; 64]; 8];
    let mut i = 0;
    let mut j = 0;

    while i < 8 {
        while j < 8 {
            let n = j;
            let e = 7 - i;
            let s = 7 - j;
            let w = i;

            squares_to_edge[MovePattern::N as usize][tuple_to_square_index((j, i)) as usize] = n;
            squares_to_edge[MovePattern::E as usize][tuple_to_square_index((j, i)) as usize] = e;
            squares_to_edge[MovePattern::S as usize][tuple_to_square_index((j, i)) as usize] = s;
            squares_to_edge[MovePattern::W as usize][tuple_to_square_index((j, i)) as usize] = w;
            squares_to_edge[MovePattern::NE as usize][tuple_to_square_index((j, i)) as usize] = min_n(n, e);
            squares_to_edge[MovePattern::NW as usize][tuple_to_square_index((j, i)) as usize] = min_n(n, w);
            squares_to_edge[MovePattern::SE as usize][tuple_to_square_index((j, i)) as usize] = min_n(s, e);
            squares_to_edge[MovePattern::SW as usize][tuple_to_square_index((j, i)) as usize] = min_n(s, w);

            j += 1;
        }
        i += 1;
    }
    return squares_to_edge;
}
pub const NUM_OF_SQUARES_TO_EDGE: [[i8; 64]; 8] = compute_nr_of_squares_to_edge();

fn find_origin_to_delta(origin: (i8, i8), to: (i8, i8)) -> (i8, i8) {
    let dx = (to.0 - origin.0).abs();
    let dy = (to.1 - origin.1).abs();
    (dx, dy)
}

pub fn is_allowed_move(origin: (i8, i8), to: (i8, i8), pattern: &MovePattern) -> bool {
    let (dx, dy) = find_origin_to_delta(origin, to);
    match pattern {
        _ => false
    }
}

fn is_origin(dx: i8, dy: i8) -> bool {
    dx == 0 && dy == 0
}

pub fn is_in_l_from_origin(origin: (i8, i8), to: (i8, i8)) -> bool {
    let (dx, dy) = find_origin_to_delta(origin, to);
    if dx == 0 || dy == 0 {return false;}
    if dx > 2 || dy > 2 {return false;}
    
    let delta = dx - dy;
    delta.abs() == 1
}

pub fn is_in_square_from_origin(origin: (i8, i8), to: (i8, i8)) -> bool {
    let (dx, dy) = find_origin_to_delta(origin, to);
    dx <= 1 && dy <= 1
}
