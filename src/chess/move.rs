use super::piece;

pub enum MovePattern {
    Diagonal,
    Perpendicular,
    Parallel
}

pub fn is_allowed_move(origin: (f32, f32), to: (f32, f32), pattern: &MovePattern) -> bool {
    match pattern {
        MovePattern::Diagonal => is_diagonal_from_origin(origin, to),
        MovePattern::Parallel => is_parallel_from_origin(origin, to),
        MovePattern::Perpendicular => is_perpendicular_from_origin(origin, to)
    }
}

pub fn is_in_square_from_origin(origin: (f32, f32), to: (f32, f32)) -> bool {
    let dx = (to.0 - origin.0).abs();
    let dy = (to.1 - origin.1).abs();

    if dx == 0.0 && dy == 0.0 {return false};
    dx <= 1.0 && dy <= 1.0
}

fn is_diagonal_from_origin(origin: (f32, f32), to: (f32, f32)) -> bool {
    let dx = (to.0 - origin.0).abs();
    let dy = (to.1 - origin.1).abs();
    dx == dy
}

fn is_perpendicular_from_origin(origin: (f32, f32), to: (f32, f32)) -> bool {
    let dx = (to.0 - origin.0).abs();
    let dy = (to.1 - origin.1).abs();
    dx == 0.0
}

fn is_parallel_from_origin(origin: (f32, f32), to: (f32, f32)) -> bool {
    let dx = (to.0 - origin.0).abs();
    let dy = (to.1 - origin.1).abs();
    dy == 0.0
}

