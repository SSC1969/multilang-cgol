use std::{
    collections::hash_map::RandomState,
    fmt,
    hash::{BuildHasher, Hasher},
    thread, time,
};

struct Game {
    board: Vec<Vec<bool>>,
    w: usize,
    h: usize,
}

impl Game {
    /// Creates a new Grid that is entirely false (dead)
    fn new(w: usize, h: usize) -> Self {
        let mut v = vec![vec![false; w]; h];

        // mediocre rng using hashmap's random state to enable ~1/4 cells
        for _ in 0..((w * h) / 3) {
            let x = RandomState::new().build_hasher().finish() % w as u64;
            let y = RandomState::new().build_hasher().finish() % h as u64;

            v[y as usize][x as usize] = true;
        }

        Self { board: v, w, h }
    }

    /// Sets the cell at (`x`, `y`) to `v`
    fn set_cell(&mut self, x: usize, y: usize, v: bool) {
        self.board[y][x] = v;
    }

    /// Gets the cell value at (`x`, `y`), wrapping toroidally around the grid (e.g. -1 will be w - 1)
    fn get_cell(&self, x: i32, y: i32) -> bool {
        let mut nx = x + self.w as i32;
        nx %= self.w as i32;
        let mut ny = y + self.h as i32;
        ny %= self.h as i32;

        self.board[ny as usize][nx as usize]
    }

    /// Computes what the next state of the cell at (`x`, `y`) will be
    fn next_cell(&self, x: i32, y: i32) -> bool {
        let mut alive_neighbours = 0;

        for i in -1..2 {
            for j in -1..2 {
                if (i != 0 || j != 0) && (self.get_cell(x + i, y + j)) {
                    alive_neighbours += 1;
                }
            }
        }

        // Return following the rules of CGoL:
        //  exactly 3 neighbours: on,
        //  exactly 2 neighbours: keep current state,
        //  otherwise: off
        (alive_neighbours == 3) || (alive_neighbours == 2 && self.get_cell(x, y))
    }

    /// Steps forward one game tick and updates the board accordingly
    fn step(&mut self) {
        let mut new = Game::new(self.w, self.h);
        for y in 0..self.h {
            for x in 0..self.w {
                new.set_cell(x, y, self.next_cell(x as i32, y as i32));
            }
        }

        *self = new;
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for y in 0..self.h {
            for x in 0..self.w {
                buf.push(if self.get_cell(x as i32, y as i32) {
                    '*'
                } else {
                    ' '
                });
            }
            buf.push('\n');
        }
        write!(f, "{}", buf)
    }
}

fn main() {
    let mut g = Game::new(40, 15);
    for _ in 0..300 {
        // clear screen and set cursor to the first character
        print!("\x1B[2J\x1B[1;1H");
        print!("{g}");
        thread::sleep(time::Duration::from_millis(50));
        g.step();
    }
}
