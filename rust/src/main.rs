mod ncurses_utils;
mod board;
mod snake;
mod point;

use std::char;
use ncurses_utils::*;
use snake::*;
use board::*;

const DEELAY_FOR_KEY: i32 = 250;

fn main() {
  init_ncurses();

  let mut snake = Snake::new();

  getchar_timeout(DEELAY_FOR_KEY);
  loop {
      clear_screen();
      print_board();
      snake.print();
      if snake.change_dir(getchar() as u8 as char) {
          break;
      }
      if !snake.move_it() {
        break;
      }

      move_pointer(13, 0);
  }

  getchar_timeout(-1);
  getchar();

  release_screen();

}
