pub enum MovePattern {
    Diagonal,
    Perpendicular,
    Parallel
}

fn find_origin_to_delta(origin: (i8, i8), to: (i8, i8)) -> (i8, i8) {
    let dx = (to.0 - origin.0).abs();
    let dy = (to.1 - origin.1).abs();
    (dx, dy)
}

pub fn is_allowed_move(origin: (i8, i8), to: (i8, i8), pattern: &MovePattern) -> bool {
    let (dx, dy) = find_origin_to_delta(origin, to);
    match pattern {
        MovePattern::Diagonal => !is_origin(dx, dy) && (dx == dy),
        MovePattern::Parallel => !is_origin(dx, dy) && (dy == 0),
        MovePattern::Perpendicular => !is_origin(dx, dy) && (dx == 0)
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
