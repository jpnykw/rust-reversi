use super::reverse;

pub fn run (
    _stone: usize,
    _board: [[usize; 8]; 8]
) -> Vec<[usize; 2]> {
    let mut positions: Vec<[usize; 2]> = Vec::with_capacity(64);

    for y in 0..8 {
        for x in 0..8 {
            if _board[y][x] == 0 && _board !=reverse::run(x, y, _stone, _board) {
                positions.push([x, y]);
            }
        }
    }

    positions
}

