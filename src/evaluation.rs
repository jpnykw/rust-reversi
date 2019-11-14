use super::assist;

const EVALUATION: [[i16; 8] ;8] = [
	[30, -12, 0, -1, -1, 0, -12, 30],
	[-12, -15, -3, -3, -3, -3, -15, -12],
	[0, -3, 0, -1, -1, 0, -3, 0],
	[-1, -3, -1, -1, -1, -1, -3, -1],
	[-1, -3, -1, -1, -1, -1, -3, -1],
	[0, -3, 0, -1, -1, 0, -3, 0],
	[-12, -15, -3, -3, -3, -3, -15, -12],
	[30, -12, 0, -1, -1, 0, -12, 30]
];

pub fn run (
    _stone: usize,
    _board: [[usize; 8]; 8]
) -> [usize; 2] {
    let mut myself_eval: i16 = 0;
    let mut max_eval: i16 = std::i16::MIN;

    for y in 0..8 {
        for x in 0..8 {
            myself_eval += if _board[y][x] == _stone {
                EVALUATION[y][x]
            } else {
                0
            };
        }
    }

    let mut result: [usize; 2] = [0; 2];
    for position in &assist::run(_stone, _board) {
        let new_eval = myself_eval + EVALUATION[position[1]][position[0]];
        if new_eval > max_eval {
            max_eval = new_eval;
            result[0] = position[0];
            result[1] = position[1];
        }
    }

    result
}

