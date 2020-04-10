mod snake;
mod ncurses_utils;

use std::char;
use ncurses::*;
use ncurses_utils::*;
use snake::*;

fn main() {
  init_ncurses();

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

  finish_ncurses();

}
