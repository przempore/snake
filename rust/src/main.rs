mod snake;

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


fn main() {
  initscr();
  raw();

  keypad(stdscr(), true);
  noecho();

  let mut snake = snake::Snake::new();

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
