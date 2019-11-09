// TODO: Use rustbox for the UI.
extern crate rand;
extern crate rustbox;

use rustbox::Key as RKey;
use rustbox::{Color, RustBox};

// use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use rand::thread_rng;

struct Cell {
    is_bomb: bool,
    is_visible: bool,
}

struct Board {
    grid: Vec<Vec<Cell>>,
    size: usize,
}

impl Board {
    fn new(size: usize, num_of_bombs: usize) -> Self {
        let mut board = Board {
            grid: Vec::new(),
            size: size,
        };

        for i in 0..size {
            board.grid.push(Vec::new());
        }

        // TODO: populate board.
        for i in 0..size {
            for j in 0..size {
                board.grid[i].push(Cell {
                    is_bomb: false,
                    is_visible: false,
                });
            }
        }

        // TODO: insert bombs
        let mut bombs: Vec<usize> = (0..size * size).collect();
        let mut rng = thread_rng();
        bombs.shuffle(&mut rng);

        for index in 0..num_of_bombs {
            let _index = bombs[index];
            let i = _index / size;
            let j = _index % size;
            board.grid[i][j].is_bomb = true;
        }

        return board;
    }

    // We don't chech if current cell is bomb. We should have checked for that when we selected it.
    fn neighbour_bombs(&self, x: usize, y: usize) -> u8 {
        let mut num_of_bombs: u8 = 0;
        for i in (x as isize - 1)..(x as isize + 2) {
            if i < 0 || i >= (self.size as isize) {
                continue;
            }
            for j in (y as isize - 1)..(y as isize + 2) {
                if j < 0 || j >= (self.size as isize) {
                    continue;
                }

                if i as usize == x && j as usize == y {
                    continue;
                }

                if self.grid[i as usize][j as usize].is_bomb {
                    num_of_bombs += 1;
                }
            }
        }
        return num_of_bombs;
    }

    fn reveal_cell(&mut self, x: usize, y: usize) {
        if self.grid[x][y].is_visible {
            return;
        }

        self.grid[x][y].is_visible = true;

        if self.grid[x][y].is_bomb {
            return;
        }

        if self.neighbour_bombs(x, y) != 0 {
            return;
        }

        for i in (x as isize - 1)..(x as isize + 2) {
            if i < 0 || i >= (self.size as isize) {
                continue;
            }
            for j in (y as isize - 1)..(y as isize + 2) {
                if j < 0 || j >= (self.size as isize) {
                    continue;
                }

                self.reveal_cell(i as usize, j as usize);
            }
        }
    }
}

enum Status {
    WON,
    LOST,
    QUIT,
}

struct Game {
    board: Board,
    rustbox: RustBox,
    player_pos: (usize, usize),
}

impl Game {
    fn new(size: usize, num_of_bombs: usize) -> Self {
        let rustbox = match RustBox::init(Default::default()) {
            Result::Ok(v) => v,
            Result::Err(e) => panic!("{}", e),
        };

        let board = Board::new(size, num_of_bombs);

        Game {
            rustbox: rustbox,
            board: board,
            player_pos: (size / 2, size / 2),
        }
    }

    fn draw(&self) {
        let fg = Color::White;
        let bg = Color::Black;

        let mut x = 0;
        let mut y = 0;

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
                if !self.board.grid[i][j].is_visible {
                    ch = '█';
                } else {
                    ch = (self.board.neighbour_bombs(i, j) + 48) as char; // Note: the 48 is there to make it compatible with ascii.

                    if self.board.grid[i][j].is_bomb {
                        ch = 'B';
                    } else if ch == '0' {
                        ch = ' ';
                    }
                }

                let mut str: String;
                if i == self.player_pos.0 && j == self.player_pos.1 {
                    let fg = Color::Red;
                    str = margin_selected.to_owned() + &ch.to_string() + margin_selected + "|";
                    self.rustbox.print(x, y, rustbox::RB_NORMAL, fg, bg, &str);
                } else {
                    str = margin_blank.to_owned() + &ch.to_string() + margin_blank + "|";
                    self.rustbox.print(x, y, rustbox::RB_NORMAL, fg, bg, &str);
                }
                x += boarder.len();
            }

            y += 1;
            x = 0;
            for i in 0..self.board.size {
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

    fn run(&mut self) -> Status {
        loop {
            self.draw();

            match self.rustbox.poll_event(false) {
                Ok(rustbox::Event::KeyEvent(key)) => match key {
                    RKey::Char('q') => return Status::QUIT,
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
                    RKey::Char(' ') => {
                        self.board.reveal_cell(self.player_pos.0, self.player_pos.1);
                        if self.board.grid[self.player_pos.0][self.player_pos.1].is_bomb {
                            return Status::LOST;
                        }
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
    let size = 20;
    // let board = Board::new(size, 10);
    let mut game = Game::new(size, 40);
    // draw(&board);
    game.run();
}
