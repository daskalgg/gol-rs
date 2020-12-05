// TODO: Use rustbox for the UI.
extern crate rand;
extern crate rustbox;

use rustbox::Key as RKey;
use rustbox::{Color, RustBox};

// use rand::{thread_rng, Rng};
// use rand::seq::SliceRandom;
// use rand::thread_rng;

struct Cell {
    is_alive: bool,
    was_alive: bool,
}

struct Board {
    grid: Vec<Vec<Cell>>,
    size: usize,
}

impl Board {
    fn new(size: usize) -> Self {
        let mut board = Board {
            grid: Vec::new(),
            size: size,
        };

        for _i in 0..size {
            board.grid.push(Vec::new());
        }

        // TODO: populate board.
        for i in 0..size {
            for _j in 0..size {
                board.grid[i].push(Cell {
                    is_alive: false,
                    was_alive: false,
                });
            }
        }

        return board;
    }

    fn alive_cell_toggle(&mut self, x: usize, y: usize) {
        self.grid[x][y].is_alive = !self.grid[x][y].is_alive;
        self.grid[x][y].was_alive = self.grid[x][y].is_alive;
    }

    fn was_alive(&self, x: usize, y: usize) -> bool {
        return self.grid[x][y].was_alive;
    }

    fn is_alive(&self, x: usize, y: usize) -> bool {
        return self.grid[x][y].is_alive;
    }

    fn num_of_alive_neighbors(&self, x: isize, y: isize) -> usize {
        let mut noan = 0;
        for i in (x - 1)..(x + 2) {
            for j in (y - 1)..(y + 2) {
                if i == j
                    || i < 0
                    || i >= (self.size as isize)
                    || j < 0
                    || j >= (self.size as isize)
                {
                    continue;
                }

                if self.was_alive(i as usize, j as usize) {
                    noan += 1;
                }
            }
        }
        return noan;
    }

    fn backup(&mut self) {
        for i in 0..self.size {
            for j in 0..self.size {
                self.grid[i][j].was_alive = self.grid[i][j].is_alive;
            }
        }
    }

    fn update(&mut self) {
        self.backup();
        for i in 0..self.size {
            for j in 0..self.size {
                let noan = self.num_of_alive_neighbors(i as isize, j as isize);
                if noan < 2 || noan > 3 {
                    self.grid[i][j].is_alive = false;
                }
                if noan == 3 {
                    self.grid[i][j].is_alive = true;
                }
                if noan == 2 && self.is_alive(i, j) {
                    self.grid[i][j].is_alive = true;
                }
            }
        }
    }
}

struct Game<'a> {
    board: Board,
    rustbox: &'a RustBox,
    player_pos: (usize, usize),
}

impl<'a> Game<'a> {
    fn new(size: usize, rustbox: &'a RustBox) -> Self {
        let board = Board::new(size);

        Game {
            rustbox: rustbox,
            board: board,
            player_pos: (size / 2, size / 2),
        }
    }

    fn draw(&self) {
        let mut x = 0;
        let mut y = 0;

        let fg = Color::White;
        let bg = Color::Black;

        let boarder = "+---";
        let margin_blank = " ";
        let margin_selected = "█";
        for _i in 0..self.board.size {
            self.rustbox
                .print(x, y, rustbox::RB_NORMAL, fg, bg, &boarder);
            x += boarder.len();
        }

        self.rustbox
            .print_char(x, y, rustbox::RB_NORMAL, fg, bg, '+');
        y += 1;
        x = 0;

        for i in 0..self.board.size {
            self.rustbox
                .print_char(x, y, rustbox::RB_NORMAL, fg, bg, '|');
            x += 1;
            for j in 0..self.board.size {
                let mut ch: char = ' ';

                let mut fg = Color::White;
                let bg = Color::Black;

                if self.board.grid[i][j].is_alive {
                    ch = '█';
                }

                let str: String;
                if i == self.player_pos.0 && j == self.player_pos.1 {
                    fg = Color::Red;
                    str = margin_selected.to_owned() + &ch.to_string() + margin_selected + "|";
                } else {
                    str = margin_blank.to_owned() + &ch.to_string() + margin_blank + "|";
                }

                self.rustbox.print(x, y, rustbox::RB_NORMAL, fg, bg, &str);
                x += boarder.len();
            }

            y += 1;
            x = 0;
            for _i in 0..self.board.size {
                self.rustbox
                    .print(x, y, rustbox::RB_NORMAL, fg, bg, &boarder);
                x += boarder.len();
            }
            self.rustbox
                .print_char(x, y, rustbox::RB_NORMAL, fg, bg, '+');
            y += 1;
            x = 0;
        }
        self.rustbox.present();
    }

    fn run(&mut self) {
        loop {
            self.draw();

            match self.rustbox.poll_event(false) {
                Ok(rustbox::Event::KeyEvent(key)) => match key {
                    RKey::Char('q') => return,
                    RKey::Char('k') => {
                        if self.player_pos.0 == 0 {
                            self.player_pos.0 = self.board.size - 1;
                        } else {
                            self.player_pos.0 -= 1;
                        }
                    }
                    RKey::Char('j') => {
                        if self.player_pos.0 == self.board.size - 1 {
                            self.player_pos.0 = 0;
                        } else {
                            self.player_pos.0 += 1;
                        }
                    }
                    RKey::Char('h') => {
                        if self.player_pos.1 == 0 {
                            self.player_pos.1 = self.board.size - 1;
                        } else {
                            self.player_pos.1 -= 1;
                        }
                    }
                    RKey::Char('l') => {
                        if self.player_pos.1 == self.board.size - 1 {
                            self.player_pos.1 = 0;
                        } else {
                            self.player_pos.1 += 1;
                        }
                    }
                    RKey::Char(' ') => self
                        .board
                        .alive_cell_toggle(self.player_pos.0, self.player_pos.1),
                    RKey::Char('n') => {
                        self.board.update();
                    }
                    _ => continue,
                },
                Err(e) => panic!("{}", e),
                _ => panic!("Something happend"),
            };
        }
    }
}

fn main() {
    let size = 10;
    // let board = Board::new(size, 10);
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let mut game = Game::new(size, &rustbox);
    game.run();
    rustbox.present();

    // loop {
    //     match rustbox.poll_event(false) {
    //         Ok(rustbox::Event::KeyEvent(key)) => match key {
    //             _ => return,
    //         },
    //         Err(e) => panic!("{}", e),
    //         _ => panic!("Something happend"),
    //     }
    // }
}
