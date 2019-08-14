extern crate termion;

use std::cmp;
use std::io::{stdout, Read, Write};
use std::thread;
use std::time::Duration;
use termion::async_stdin;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style};

type Pos = (u16, u16);
type TetPos = [Pos; 4];

struct Tetromino {
    rotation: i32,
    position: Box<Fn(Pos) -> TetPos>,
}

enum TetType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

fn gen(t: TetType) -> Tetromino {
    Tetromino {
        rotation: 0,
        position: Box::new(move |(x, y): Pos| -> TetPos {
            match t {
                TetType::O => [(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)],
                TetType::T => [(x, y), (x + 1, y), (x + 2, y), (x + 1, y + 1)],
                TetType::S => [(x, y), (x + 1, y), (x + 1, y + 1), (x + 2, y + 1)],
                TetType::Z => [(x, y), (x + 1, y), (x + 1, y - 1), (x + 2, y - 1)],
                TetType::J => [(x, y), (x, y - 1), (x + 1, y - 1), (x + 2, y - 1)],
                TetType::L => [(x, y), (x + 1, y), (x + 2, y), (x + 2, y + 1)],
                TetType::I => [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            }
        }),
    }
}

fn render(blocks: &[Tetromino], tick: u16) {
    //how to pass this in function?
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    //render base grid
    //TODO: get proper terminal size
    for grid_y in 1..15 {
        for grid_x in 1..15 {
            write!(stdout, "{}-", cursor::Goto(grid_x, grid_y)).unwrap();
        }
    }

    blocks.iter().for_each(|block| {
        let current = &(block.position)((1, cmp::min(tick, 15)));
        current.iter().for_each(|&(x, y)| {
            write!(
                stdout,
                "{}{}#{}",
                cursor::Goto(x, y),
                color::Fg(color::Yellow),
                style::Reset
            )
            .unwrap();
        });
    });

    write!(stdout, "{}{}", style::Reset, cursor::Goto(1, 16)).unwrap();
}

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();

    let mut blocks: Vec<Tetromino> = vec![];

    let p = gen(TetType::O);
    blocks.push(p);

    let p2 = gen(TetType::L);
    blocks.push(p2);

    let mut tick = 1;

    loop {
        // write!(stdout, "{}", clear::All).unwrap();

        // let b = stdin.next();
        // write!(stdout, "\r{:?}    <- This demonstrates the async read input char. Between each update a 100 ms. is waited, simply to demonstrate the async fashion. \n\r", b).unwrap();
        if tick > 100 {
            break;
        }

        tick += 1;
        stdout.flush().unwrap();

        thread::sleep(Duration::from_millis(50));
        // stdout.write_all(b"# ").unwrap();
        render(&blocks, tick);
        write!(stdout, "{}", cursor::Goto(1, 16)).unwrap();
        stdout.flush().unwrap();
    }
}
