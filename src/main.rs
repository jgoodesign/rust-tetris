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

fn render_empty(x: usize, y: usize) {
    //cursor is 1 indexed!
    print!("{}-", cursor::Goto(x as u16 + 1, y as u16 + 1));
}

fn render_active(x: usize, y: usize) {
    print!(
        "{}{}#{}",
        cursor::Goto(x as u16 + 1, y as u16 + 1),
        color::Fg(color::Yellow),
        style::Reset
    );
}

fn render(grid: &[[GridPoint; W]; H]) {
    for (y, row) in grid.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            match point {
                GridPoint::Empty => render_empty(x, y),
                _ => render_active(x, y),
            };
        }
    }

    // print!("{}{}", style::Reset, cursor::Goto(1, H as u16 + 1));
    //do i need this here?
    stdout().flush().unwrap();
}

#[derive(Clone, Copy)]
enum GridPoint {
    Empty,
    Active,
    Locked,
}

const W: usize = 16;
const H: usize = 20;

fn main() {
    // let mut stdin = async_stdin().bytes();

    //clears / resets display
    print!("{}{}", clear::All, cursor::Goto(1, 1));

    let mut blocks: Vec<Tetromino> = vec![];
    let mut grid = [[GridPoint::Empty; W]; H];

    let p = gen(TetType::O);
    // blocks.push(p);
    let current = (p.position)((1, 1));
    push_block(&mut grid, current);

    let mut tick = 1;

    loop {
        if tick > 100 {
            break;
        }

        tick += 1;

        thread::sleep(Duration::from_millis(50));
        render(&grid);
    }
}

fn push_block(grid: &mut [[GridPoint; W]; H], current: TetPos) {
    current.iter().for_each(|&(x, y)| {
        grid[y as usize][x as usize] = GridPoint::Active;
    });
}
