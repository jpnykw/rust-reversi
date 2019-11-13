use super::reverse;

pub fn run (
    _stone: usize,
    _board: [[usize; 8]; 8]
) -> [[usize; 8]; 8] {
    let mut positions: [[usize; 8]; 8] = [[0; 8]; 8];
    for y in 0..8 {
        for x in 0..8 {
            if _board[y][x] == 0 {
                positions[y][x] = if _board == reverse::run(x, y, _stone, _board) {
                    0
                } else {
                    1
                };
            }
        }
    }

    positions
}

