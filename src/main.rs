#[macro_use]
extern crate colour;

type Pos = (i32, i32);
type TetPos = [Pos; 4];

struct Tetromino {
    rot: i32,
    pos: Box<Fn(Pos) -> TetPos>,
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
        rot: 0,
        pos: Box::new(move |(x, y): Pos| -> TetPos {
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

fn render(current: &TetPos) {
    for grid_y in 0..15 {
        for grid_x in 0..15 {
            if current.iter().any(|&(x, y)| x == grid_x && y == grid_y) {
                yellow!("#");
            } else {
                print!("-");
            }
        }
        println!(" ");
    }
}

fn main() {
    let p = gen(TetType::O);
    let current = &(p.pos)((4, 5));

    render(current);
}
