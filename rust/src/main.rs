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
      snake.move_it();
      snake.print();
      timeout(500);
      if snake.change_dir(getch() as u8 as char) {
          break;
      }
      mv(13, 0);
  }

  finish_ncurses();

}
