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
    is_marked: bool,
}

struct Board {
    grid: Vec<Vec<Cell>>,
    size: usize,
    num_of_bombs: usize,
}

impl Board {
    fn new(size: usize, num_of_bombs: usize) -> Self {
        let mut board = Board {
            grid: Vec::new(),
            size: size,
            num_of_bombs: num_of_bombs,
        };

        for _i in 0..size {
            board.grid.push(Vec::new());
        }

        // TODO: populate board.
        for i in 0..size {
            for _j in 0..size {
                board.grid[i].push(Cell {
                    is_bomb: false,
                    is_visible: false,
                    is_marked: false,
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

    fn chech_win(&self) -> bool {
        let mut count = 0;

        for i in 0..self.grid.len() {
            for j in 0..self.grid.len() {
                if !self.grid[i][j].is_visible {
                    count += 1;
                }
            }
        }

        return count == self.num_of_bombs;
    }

    fn mark_cell_toggle(&mut self, x: usize, y: usize) {
        if !self.grid[x][y].is_visible {
            self.grid[x][y].is_marked = !self.grid[x][y].is_marked;
        }
    }
}

enum Status {
    WON,
    LOST,
    QUIT,
}

struct Game<'a> {
    board: Board,
    rustbox: &'a RustBox,
    player_pos: (usize, usize),
}

impl<'a> Game<'a> {
    fn new(size: usize, num_of_bombs: usize, rustbox: &'a RustBox) -> Self {
        let board = Board::new(size, num_of_bombs);

        Game {
            rustbox: rustbox,
            board: board,
            player_pos: (size / 2, size / 2),
        }
    }

    fn draw(&self) {
        let mut x = 0;
        let mut y = 0;

        let mut fg = Color::White;
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

                if !self.board.grid[i][j].is_visible {
                    ch = '█';
                    if self.board.grid[i][j].is_marked {
                        fg = Color::Yellow;
                    }
                } else {
                    ch = (self.board.neighbour_bombs(i, j) + 48) as char; // Note: the 48 is there to make it compatible with ascii.

                    if self.board.grid[i][j].is_bomb {
                        ch = 'B';
                    } else if ch == '0' {
                        ch = ' ';
                    }
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
                    RKey::Char('m') => {
                        self.board
                            .mark_cell_toggle(self.player_pos.0, self.player_pos.1);
                    }
                    _ => continue,
                },
                Err(e) => panic!("{}", e),
                _ => panic!("Something happend"),
            };

            if self.board.chech_win() {
                return Status::WON;
            }
        }
    }
}

fn main() {
    let size = 20;
    // let board = Board::new(size, 10);
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let mut game = Game::new(size, 40, &rustbox);
    match game.run() {
        Status::WON => {
            rustbox.print(
                0,
                0,
                rustbox::RB_NORMAL,
                Color::Green,
                Color::Black,
                "You Won!!!",
            );
        }

        Status::LOST => {
            rustbox.print(
                0,
                0,
                rustbox::RB_NORMAL,
                Color::Red,
                Color::Black,
                "You Lost!!!",
            );
        }

        _ => return,
    }
    rustbox.present();
    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => match key {
                _ => return,
            },
            Err(e) => panic!("{}", e),
            _ => panic!("Something happend"),
        }
    }
}
