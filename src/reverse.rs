pub fn run (
    _x: usize,
    _y: usize,
    _stone: usize,
    _board: [[usize; 8]; 8],
) -> [[usize; 8]; 8] {
    let opponent_stone = if _stone == 1 { 2 } else { 1 };
    let dx: [i32; 8] = [0, -1, -1, -1, 0, 1, 1, 1];
    let dy: [i32; 8] = [1, 1, 0, -1, -1, -1, 0, 1];
    let mut new_board = _board;

    for id in 0..8 {
        let mut _x_pos = _x as i32 + dx[id];
        let mut _y_pos = _y as i32 + dy[id];
        if _x_pos < 0 || _x_pos > 7 || _y_pos < 0 || _y_pos > 7 {
            continue;
        }

        if _board[_y_pos as usize][_x_pos as usize] == opponent_stone {
            let mut flag = true;
            let mut count_max = 0;

            loop {
                count_max += 1;
                _x_pos += dx[id];
                _y_pos += dy[id];

                if _x_pos < 0
                    || _x_pos > 7
                    || _y_pos < 0
                    || _y_pos > 7
                    || _board[_y_pos as usize][_x_pos as usize] == 0
                {
                    flag = false;
                    break;
                } else if _board[_y_pos as usize][_x_pos as usize] == _stone {
                    break;
                }
            }

            if flag {
                _x_pos = _x as i32;
                _y_pos = _y as i32;

                for _i in 0..count_max {
                    _x_pos += dx[id];
                    _y_pos += dy[id];
                    new_board[_y_pos as usize][_x_pos as usize] = _stone;
                }
            }
        }
    }

    new_board
}

