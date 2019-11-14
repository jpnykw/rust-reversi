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

        let mut counter: i16 = 0;

        println!("原点: {:?}", pos);

        loop {
            counter += 1;
            if counter == std::i16::MAX {
                panic!("> 設置場所が見つかりませんでした");
            }

            _stone = if _stone == 1 { 2 } else { 1 };
            let way = assist::run(_stone, _board);
            println!("  > 設置可能場所候補: {:?}", way);

            if way.len() < 1 {
                if flag {
                    println!("  > 終了");
                    break;
                } else {
                    flag = true;
                    println!("  > スキップ");
                }

                continue;
            } else {
                flag = false;
                let rnd: f32 = rng.gen();
                let index = (rnd * way.len() as f32) as usize;

                println!("      > 設置: {:?}", way[index]);
                // println!(" > index: {}", index);

                _board = reverse::run(
                    way[index][0],
                    way[index][1],
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

