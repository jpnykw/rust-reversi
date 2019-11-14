use super::evaluation;
use super::reverse;
use super::assist;

pub fn get (
    _stone: usize,
    _board: [[usize; 8]; 8]
) -> [[usize; 2]; 2] {
    let mut i = 0;
    let mut eval_id_1 = 0;
    let mut eval_id_2 = 0;
    let mut eval_max_1 = std::i16::MIN;
    let mut eval_max_2 = std::i16::MIN;

    let position = assist::run(_stone, _board);
    for vec in &position {
        let mut _board_sub = reverse::run(vec[0], vec[1], _stone, _board);
        _board_sub[vec[1]][vec[0]] = _stone;

        let eval = evaluation::calc(_stone, _board_sub);
        // println!("Pos: {:?}, Eval: {}", vec, eval);

        if eval_max_1 < eval {
            eval_max_1 = eval;
            eval_id_1 = i;
        } else if eval_max_2 < eval {
            eval_max_2 = eval;
            eval_id_2 = i;
        }

        i += 1;
    }

    [
        position[eval_id_1],
        position[eval_id_2]
    ]
}

pub fn run (
    _stone: usize,
    _board: [[usize; 8]; 8]
) -> [usize; 2] {
    // TODO: がんばる！

    let _board_sub = _board;
    let stack = get(_stone, _board_sub);

    // println!(" -ID 1: {},\n -Vec: {:?}", eval_id_1, position[eval_id_1]);
    // println!(" -ID 2: {},\n -Vec: {:?}", eval_id_2, position[eval_id_2]);

    // [0; 2]
    stack[0]
}

