use std::io;

// 石の数を数える
fn count_stones(_board: [[usize; 8]; 8]) -> [usize; 2] {
    let mut stones: [usize; 2] = [0; 2];

    for i in 0..8 {
        for j in 0..8 {
            if _board[i][j] > 0 {
                stones[_board[i][j] - 1] += 1;
            }
        }
    }

    return stones;
}

// ひっくり返したあとの盤面を返す
fn get_reversed_board(_x: usize, _y: usize, _stone: usize, _board: [[usize; 8]; 8]) -> [[usize; 8]; 8] {
    // println!("To: ({}, {}), Stone: {}", _x, _y, if _stone == 1 { "WHITE" } else { "BLACK" });

    let opponent_stone = if _stone == 1 { 2 } else { 1 };
    // let mut new_board: [[usize; 8]; 8] = [[0; 8]; 8];
    let dx: [i32; 8] = [0, -1, -1, -1,  0,  1, 1, 1];
    let dy: [i32; 8] = [1,  1,  0, -1, -1, -1, 0, 1];
    let mut new_board = _board;

    for id in 0..8 {
        let mut _x_pos = _x as i32 + dx[id];
        let mut _y_pos = _y as i32 + dy[id];
        if _x_pos < 0 || _x_pos > 7 || _y_pos < 0 || _y_pos > 7 { continue; }

        // 相手の石が対象先に存在する場合に探査を始める
        if _board[_y_pos as usize][_x_pos as usize] == opponent_stone {
            let mut flag = true;
            let mut count_max = 0;

            loop {
                count_max += 1;
                _x_pos += dx[id];
                _y_pos += dy[id];

                if _board[_y_pos as usize][_x_pos as usize] == _stone {
                    break;
                } else if _x_pos < 0 || _x_pos > 7 || _y_pos < 0 || _y_pos > 7 || _board[_y_pos as usize][_x_pos as usize] == 0 {
                    flag = false;
                    break;
                }
            }

            if flag {
                // println!("OK");
                _x_pos = _x as i32;
                _y_pos = _y as i32;

                for i in 0..count_max {
                    _x_pos += dx[id];
                    _y_pos += dy[id];
                    new_board[_y_pos as usize][_x_pos as usize] = _stone;
                }
            }

            // if _x_pos < 0 || _x_pos > 7 || _y_pos < 0 || _y_pos > 7 { continue; }
            // println!("OK");
        }
    }

    return new_board;
}

fn alert() {
    println!("\n==================================================");
    println!("You can't put there.");
    println!("==================================================");
}

fn main() {
    // ターン制御
    let mut is_black_turn = true;

    // ゲームの盤面を初期化する
    let mut board: [[usize; 8]; 8] = [[0; 8]; 8];
    board[3][3] = 1;
    board[3][4] = 2;
    board[4][3] = 2;
    board[4][4] = 1;

    loop {
        // 石の数を表示
        let stones = count_stones(board);
        println!("\nStones:\n - Black: {}\n - White: {}", stones[0], stones[1]);

        // ターンの表示
        println!("\nTurn: {}", if is_black_turn { "Black" } else { "White" });

        // 盤面の描画(CUI)
        println!("\n  0 1 2 3 4 5 6 7");
        for y in 0..8 {
            let mut display: String = y.to_string();
            for x in 0..8 { display += if board[y][x] == 1 { " W" } else if board[y][x] == 2 { " B" } else { " ." }; }
            println!("{}", display);
        }

        // ユーザーの座標入力
        let mut x = String::new();
        let mut y = String::new();
        println!("\nEnter X pos to put.");
        io::stdin().read_line(&mut x).expect("Failed to read line.");
        let x_pos: i32 = x.trim().parse().expect("Please type a number!");

        println!("\nEnter Y pos to put.");
        io::stdin().read_line(&mut y).expect("Failed to read line.");
        let y_pos: i32 = y.trim().parse().expect("Please type a number!");

        let x_id: usize = x_pos as usize;
        let y_id: usize = y_pos as usize;

        if board[y_id][x_id] > 0 {
            // 既に石がある場所
            alert();
        } else {
            let stone: usize = if is_black_turn { 2 } else { 1 };
            if board == get_reversed_board(x_id, y_id, stone, board) {
                // 周りにひっくり返せる石がない
                alert();
            } else {
                board[y_id][x_id] = stone;
                board = get_reversed_board(x_id, y_id, stone, board);
                is_black_turn = !is_black_turn;
            }
        }
    }
}
