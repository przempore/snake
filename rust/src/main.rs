mod ncurses_utils;
mod board;
mod snake;
mod point;

use std::char;
use ncurses_utils::*;
use snake::*;
use board::*;

const DEELAY_FOR_KEY: i32 = 500;

fn main() {
  init_ncurses();

  let mut snake = Snake::new();

  loop {
      clear_screen();
      print_board();
      snake.move_it();
      snake.print();
      getchar_timeout(DEELAY_FOR_KEY);
      if snake.change_dir(getchar() as u8 as char) {
          break;
      }

      move_pointer(13, 0);
  }

  release_screen();

}
