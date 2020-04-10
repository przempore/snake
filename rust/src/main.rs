extern crate ncurses;

use std::char;
use ncurses::*;

fn print_board() {
  let line = std::iter::repeat("=").take(40).collect::<String>();
  let boarder = String::from("||");
  let between_boarders = std::iter::repeat(" ").take(36).collect::<String>();
  let boarders = format!("{}{}{}", boarder, between_boarders, boarder);

  addstr(&line);
  let mut n = 1;
  for _ in 0..11 {
    mv(n, 0);
    addstr(&boarders);
    n += 1;
  }
  mv(n, 0);
  addstr(&line);
}

struct Snake{
    x: i32,
    y: i32,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            x: 5,
            y: 5,
        }
    }

    fn print(&self) {
        mv(self.y, self.x);
        addstr("&");
        mv(13, 0);
        addstr(&format!("x = {}, y = {}", self.x, self.y));
    }

    fn move_it(&mut self, key:char) -> bool {
      match key {
          'w' => { if self.y > 1 { self.y -= 1; } return false; },
          'a' => { if self.x > 2 { self.x -= 1; } return false; },
          's' => { if self.y < 11 { self.y += 1; } return false; },
          'd' => { if self.x < 37 { self.x += 1; } return false; },
          'x' => { return true; }
          _ => { return false; }
      }
    }
}

fn main() {
  initscr();
  raw();

  keypad(stdscr(), true);
  noecho();

  let mut snake = Snake::new();

  loop {
      clear();
      print_board();
      snake.print();
      if snake.move_it(getch() as u8 as char) {
          break;
      }
      mv(13, 0);
  }

  getch();
  endwin();
}
