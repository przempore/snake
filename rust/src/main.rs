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

fn print_snake(x: i32, y: i32) {
    mv(y, x);
    addstr("&");
    mv(13, 0);
    addstr(&format!("x = {}, y = {}", x, y));
}

trait MoveIt {
  fn move_it(&self);
}

struct Snake{
    //x: i32,
    //y: i32,
}

impl MoveIt for Snake {
  fn move_it(&self) {
  }
}

fn main() {
  initscr();
  raw();

  keypad(stdscr(), true);
  noecho();

  let mut x:i32 = 5;
  let mut y:i32 = 5;

  loop {
      clear();
      print_board();
      print_snake(x, y);
      mv(13, 0);
      match getch() as u8 as char {
          'w' => { if y > 1 { y -= 1; } },
          'a' => { if x > 2 { x -= 1; } },
          's' => { if y < 11 { y += 1; } },
          'd' => { if x < 37 { x += 1; } },
          'x' => { break; },
          _ => { continue; },
      }
  }

  getch();
  endwin();
}
