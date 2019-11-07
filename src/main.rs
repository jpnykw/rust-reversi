use std::io;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

const WINDOW_WIDTH: f64 = 640.0;
const WINDOW_HEIGHT: f64 = 640.0;

pub struct App {
    gl: GlGraphics // OpenGL drawing backend.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const COLOR: f32 = 0.14;
        const BLACK: [f32; 4] = [COLOR, COLOR, COLOR, 1.0];

        let (x, y) = (args.window_size[0] / 2.0,
                      args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |_c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let transform = _c.transform.trans(x, y);


            let size = 40.0;
            let mut _x = 0.0;
            let mut _y = size * -4.0 + size;

            let dx = [-size, size, size, size, size, -size, -size, -size];
            let dy = [size, size, size, -size, -size, -size, -size, size];

            for _i in 0..7 {
                _x = size * -4.0 + size;
                for _j in 0..7 {
                    for _k in 0..4 {
                        line([0.92, 0.92, 0.92, 1.0], 0.5, [
                            _x + dx[_k * 2], _y + dy[_k * 2],
                            _x + dx[_k * 2 + 1], _y + dy[_k * 2 + 1]
                        ], transform, gl);
                    }
                    _x += size;
                }
                _y += size;
            }
        });
    }
}

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

fn get_reversed_board(_x: usize, _y: usize, _stone: usize, _board: [[usize; 8]; 8]) -> [[usize; 8]; 8] {
    let opponent_stone = if _stone == 1 { 2 } else { 1 };
    let dx: [i32; 8] = [0, -1, -1, -1,  0,  1, 1, 1];
    let dy: [i32; 8] = [1,  1,  0, -1, -1, -1, 0, 1];
    let mut new_board = _board;

    for id in 0..8 {
        let mut _x_pos = _x as i32 + dx[id];
        let mut _y_pos = _y as i32 + dy[id];
        if _x_pos < 0 || _x_pos > 7 || _y_pos < 0 || _y_pos > 7 { continue; }

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
                _x_pos = _x as i32;
                _y_pos = _y as i32;

                for _i in 0..count_max {
                    _x_pos += dx[id];
                    _y_pos += dy[id];
                    new_board[_y_pos as usize][_x_pos as usize] = _stone;
                }
            }
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
    let mut is_black_turn = true;
    let mut board: [[usize; 8]; 8] = [[0; 8]; 8];
    board[3][3] = 1;
    board[3][4] = 2;
    board[4][3] = 2;
    board[4][4] = 1;

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Reversi v1.0",
            [WINDOW_WIDTH, WINDOW_HEIGHT]
        )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl)
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
    }

    // CUIのオセロ
    loop {
        let stones = count_stones(board);
        println!("\nStones:\n - White: {}\n - Black: {}", stones[0], stones[1]);
        println!("\nTurn: {}", if is_black_turn { "Black" } else { "White" });
        println!("\n  0 1 2 3 4 5 6 7");

        for y in 0..8 {
            let mut display: String = y.to_string();
            for x in 0..8 { display += if board[y][x] == 1 { " W" } else if board[y][x] == 2 { " B" } else { " ." }; }
            println!("{}", display);
        }

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
            alert();
        } else {
            let stone: usize = if is_black_turn { 2 } else { 1 };

            if board == get_reversed_board(x_id, y_id, stone, board) {
                alert();
            } else {
                board[y_id][x_id] = stone;
                board = get_reversed_board(x_id, y_id, stone, board);
                is_black_turn = !is_black_turn;
            }
        }
    }
}
