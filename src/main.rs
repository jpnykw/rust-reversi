use std::io;

fn main() {
    // ターン制御
    let mut isBlackTurn = true;

    // ゲームの盤面を初期化する
    let mut board: [[i32; 8]; 8] = [[0; 8]; 8];
    board[3][3] = 1;
    board[3][4] = 2;
    board[4][3] = 2;
    board[4][4] = 1;

    loop {
        if isBlackTurn {
            println!("Turn: Black");
        } else {
            println!("Turn: White");
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

        // 石を置く
        if isBlackTurn {
            board[y_pos][x_pos] = 2;
        } else {
            board[y_pos][x_pos] = 1;
        }

        println!("{:?}", board);

        println!("\n");
        isBlackTurn = !isBlackTurn;
    }
}
