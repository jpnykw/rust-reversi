use std::io;
use std::f64;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

const GRID_SIZE: f64 = 50.0;
const WINDOW_WIDTH: f64 = 640.0;
const WINDOW_HEIGHT: f64 = 640.0;

pub struct App {
    gl: GlGraphics
}

impl App {
    fn render(&mut self, args: &RenderArgs, _board: [[usize; 8]; 8]) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.14, 0.14, 0.14, 1.0];
        const WHITE: [f32; 4] = [0.85, 0.85, 0.85, 1.0];
        const GREEN: [f32; 4] = [0.03, 0.51, 0.23, 1.0];

        let square = rectangle::square(
            -GRID_SIZE * 4.0,
            -GRID_SIZE * 4.0,
            GRID_SIZE * 8.0
        );

        let stone = rectangle::square(0.0, 0.0, 20.0);

        let (x, y) = (args.window_size[0] / 2.0,
                      args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |_c, gl| {
            clear(WHITE, gl);

            // GRID
            let mut _x = 0.0;
            let mut _y = GRID_SIZE * -4.0 + GRID_SIZE;
            let transform = _c.transform.trans(x, y);

            let dx = [-GRID_SIZE, GRID_SIZE, GRID_SIZE, GRID_SIZE, GRID_SIZE, -GRID_SIZE, -GRID_SIZE, -GRID_SIZE];
            let dy = [GRID_SIZE, GRID_SIZE, GRID_SIZE, -GRID_SIZE, -GRID_SIZE, -GRID_SIZE, -GRID_SIZE, GRID_SIZE];

            rectangle(GREEN, square, transform, gl);

            for _i in 0..7 {
                _x = GRID_SIZE * -4.0 + GRID_SIZE;
                for _j in 0..7 {
                    for _k in 0..4 {
                        line(BLACK, 2.0, [
                            _x + dx[_k * 2], _y + dy[_k * 2],
                            _x + dx[_k * 2 + 1], _y + dy[_k * 2 + 1]
                        ], transform, gl);

                        if _board[_i][_j] > 0 {
                            let trans = _c.transform.trans(
                                _x + GRID_SIZE * 6.0 - 15.0,
                                _y + GRID_SIZE * 6.0 - 15.0
                            );

                            circle_arc(
                                // BLACK,
                                if _board[_i][_j] == 1 { WHITE } else { BLACK },
                                10.0, 0.0, f64::consts::PI * 1.9999, stone, trans, gl
                            )
                        };
                    }
                    _x += GRID_SIZE;
                }
                _y += GRID_SIZE;
            }

            // STONES
            // circle_arc(BLACK, stone, transform, gl);
            // circle_arc(BLACK, 10.0, 0.0, f64::consts::PI*1.9999, stone, transform, gl);
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

    let mut app = App {
        gl: GlGraphics::new(opengl)
    };

    let mut events = Events::new(EventSettings::new());
    let mut cursor = [0.0, 0.0];

    while let Some(e) = events.next(&mut window) {
        // 画面のレンダリング
        if let Some(r) = e.render_args() {
            app.render(&r, board);
        }
        // マウスの任意のボタンを押したときに発火
        if let Some(Button::Mouse(button)) = e.press_args() {
            // println!("Pressed mouse button '{:?}'", button); // OK
            if button == piston::MouseButton::Left {
                println!("Left Clicked!");
            }
        }
        // マウスの座標取得
        e.mouse_cursor(|pos| {
            cursor = pos;
            // println!("Mouse at  ({} {})", pos[0], pos[1]); // OK
        });
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
