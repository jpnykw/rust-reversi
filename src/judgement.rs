pub fn run (
    _stones: [usize; 2]
) -> String {
    let white = _stones[0];
    let black = _stones[1];

    String::from(if white == black {
        "DRAW"
    } else {
        if white > black {
            "WHITE WON!"
        } else {
            "BLACK WON"
        }
    })
}

