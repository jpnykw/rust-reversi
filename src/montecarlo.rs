use rand::Rng;
use super::count;
use super::assist;
use super::reverse;
// use super::evaluation;

pub fn run (
    level: usize,
    _stone: usize,
    _board: [[usize; 8]; 8]
) -> [usize; 2] {
    // for id in 0..level {
        let mut score_max = std::i16::MIN;
        let mut vec_result: [usize; 2] = [0; 2];
        let mut rng = rand::thread_rng();

        for vec_base in &assist::run(_stone, _board) {
            let mut score = 0;
            for _i in 0..level {
                // 候補に置く
                let mut _board_sub = reverse::run(
                    vec_base[0], vec_base[1],
                    _stone, _board
                );
                _board_sub[vec_base[1]][vec_base[0]] = _stone;

                let mut _stone_sub = _stone;
                let mut flag = 0;

                loop {
                    _stone_sub = if _stone_sub == 1 { 2 } else { 1 };
                    let vec = assist::run(_stone_sub, _board_sub);

                    if vec.len() == 0 {
                        flag += 1;

                        if flag == 2 {
                            break;
                        }
                    } else {
                        flag = 0;

                        let index: f32 = rng.gen();
                        let pos = vec[(index * vec.len() as f32) as usize];

                        _board_sub = reverse::run(
                            pos[0], pos[1],
                            _stone_sub,
                            _board_sub
                        );

                        _board_sub[pos[1]][pos[0]] = _stone_sub;
                    }
                }

                let stones = count::run(_board_sub);

                score += if stones[0] > stones[1] {
                    if _stone == 2 { 0 } else { 1 }
                } else {
                    if _stone == 2 { 1 } else { 0 }
                };

                // score += evaluation::calc(_stone_sub, _board_sub);
            }

            println!("  - Pos: {:?}", vec_base);
            println!("      - Score: {}", score);

            if score_max < score {
                score_max = score;
                vec_result[0] = vec_base[0];
                vec_result[1] = vec_base[1];
            }
        }

    vec_result
}

