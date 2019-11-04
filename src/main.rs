use std::io;

fn main() {
    loop {
        // ユーザーの座標入力
        let mut x = String::new();
        let mut y = String::new();
        println!("Enter X pos to put.");
        io::stdin().read_line(&mut x).expect("Failed to read line.");
        println!("Enter Y pos to put.");
        io::stdin().read_line(&mut y).expect("Failed to read line.");

        println!("You entered ({}, {})", x.trim(), y.trim());
    }
}
