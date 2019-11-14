use super::evaluation;
use super::reverse;
use super::assist;
use super::count;
use rand::Rng;

pub fn run (
    _base_stone: usize,
    _base_board: [[usize; 8]; 8]
) -> [usize; 2] {
    let mut rng = rand::thread_rng();
    let base_way = assist::run(_base_stone, _base_board);

    let mut _max = 0;
    let mut _pos = [0; 2];

    for pos in &base_way {
        println!("\nNEW:\n");
        let mut flag = false;
        let mut _stone = _base_stone;
        let mut _board = _base_board;

        _board = reverse::run(
            pos[0],
            pos[1],
            _stone, _board
        );

        _board[pos[1]][pos[0]] = _stone;

        let mut counter: i16 = 0;

        println!("Origin: {:?}", pos);

        loop {
            println!("      > Update: {:?}\n", _board);

            counter += 1;
            if counter == std::i16::MAX {
                panic!("Couldn't find the positions can put");
            }

            _stone = if _stone == 1 { 2 } else { 1 };
            let way = assist::run(_stone, _board);
            println!("  > Positions Stack: {:?}", way);

            if way.len() == 0 {
                if flag {
                    println!("  > End Turn");
                    break;
                } else {
                    println!("  > Skip Turn");
                    flag = true;
                }
            } else {
                flag = false;
                // let rnd: f32 = rng.gen();
                // let index = (rnd * (way.len() - 1) as f32) as usize;
                // println!("      > Put: {:?}", way[index]);

                let _pos = evaluation::run(_stone, _board);
                let _x = _pos[0];
                let _y = _pos[1];

                _board[_y][_x] = _stone;
                _board = reverse::run(
                    _x,
                    _y,
                    _stone, _board
                );
            }
        }

        // println!("　> 置けたか: {}", a);

        let remaining = count::run(_board)[_base_stone - 1];
        if remaining > _max {
            // println!("最大 {} -> {}", _max, remaining);
            _max = remaining;
            _pos[0] = pos[0];
            _pos[1] = pos[1];
            // println!("書き換え");
            // println!("{:?}", pos);
            println!("　> 座標更新: {:?}", pos);
        }
    }

    // return Vec::with_capacity(2);
    // [0; 2]
    let rnd: f32 = rng.gen();
    let index = (rnd * base_way.len() as f32) as usize;
    println!("return -> {:?}", _pos);
    base_way[index]
}

