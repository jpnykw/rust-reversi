use std::io;

fn main() {
    // ターン制御
    let mut is_black_turn = true;

    // ゲームの盤面を初期化する
    let mut board: [[i32; 8]; 8] = [[0; 8]; 8];
    board[3][3] = 1;
    board[3][4] = 2;
    board[4][3] = 2;
    board[4][4] = 1;

    loop {
        // ターンの表示
        if is_black_turn {
            println!("Turn: Black");
        } else {
            println!("Turn: White");
        }

        // 盤面の描画(CUI)
        println!("  0 1 2 3 4 5 6 7");
        for y in 0..8 {
            let mut display: String = y.to_string();
            for x in 0..8 {
                if board[y][x] == 1 {
                    display += " W"
                } else if board[y][x] == 2 {
                    display += " B"
                } else {
                    display += " #";
                }
            }

            println!("{}", display);
        }

        // ユーザーの座標入力
        let mut x = String::new();
        let mut y = String::new();
        println!("Enter X pos to put.");
        io::stdin().read_line(&mut x).expect("Failed to read line.");
        println!("Enter Y pos to put.");
        io::stdin().read_line(&mut y).expect("Failed to read line.");

        let x_pos: usize = x.trim().parse().unwrap();
        let y_pos: usize = y.trim().parse().unwrap();

        // TODO:
        // おけるかを確認するロジックの実装

        if board[y_pos][x_pos] > 0 {
            println!("You cant put there!");
        } else {
            // 石を置く
            if is_black_turn {
                board[y_pos][x_pos] = 2;
            } else {
                board[y_pos][x_pos] = 1;
            }

            // TODO:
            // おける場合に盤面を更新するロジックの実装

            is_black_turn = !is_black_turn;
        }

        println!("\n");
    }
}
