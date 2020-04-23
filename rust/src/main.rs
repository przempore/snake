mod ncurses_utils;
mod board;
mod snake;
mod point;

use std::char;
use ncurses_utils::*;
use snake::*;

const DEELAY_FOR_KEY: i32 = 250;

fn main() {
  init_ncurses();
  game_loop();
  wait_for_x_to_exit();
  release_screen();
}

fn game_loop() {
  let mut snake = Snake::new();

  getchar_timeout(DEELAY_FOR_KEY);
  loop {
      clear_screen();
      snake.print();
      if snake.change_dir(getchar() as u8 as char) {
          break;
      }
      if !snake.move_it() {
        break;
      }

      move_pointer(board::HIGHT as i32, 0);
  }
}

fn wait_for_x_to_exit() {
  getchar_timeout(-1);
  loop {
    let c = getchar() as u8 as char;
    match c {
      'x' => break,
      _ => {
        clear_line(board::HIGHT as i32, 0);
        add_string("To exit type x");
      },
    }
  }
}
