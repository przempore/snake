mod ncurses_utils;
mod board;
mod snake;
mod point;

use ncurses_utils::*;
use pancurses::{Input};
use snake::*;

const DEELAY_FOR_KEY: i32 = 250;

fn main() {
  let pancurses = Pancurses::new();
  pancurses.init_ncurses();
  game_loop(&pancurses);
  wait_for_x_to_exit(&pancurses);
  pancurses.release_screen();
}

fn game_loop(pancurses: &Pancurses) {
  let mut snake = Snake::new();

  pancurses.getchar_timeout(DEELAY_FOR_KEY);
  loop {
      pancurses.clear_screen();
      snake.print(&pancurses);
      match pancurses.getchar() {
        Some(Input::Character(x)) if snake.change_dir(x) => break,
        _ => (),
      }
      if !snake.move_it(&pancurses) {
        break;
      }

      pancurses.move_pointer(board::HIGHT as i32, 0);
  }
}

fn wait_for_x_to_exit(pancurses: &Pancurses) {
  pancurses.getchar_timeout(-1);
  loop {
    match pancurses.getchar() {
      Some(Input::Character(x)) if x == 'x'  => break,
        _ => {
          pancurses.clear_line(board::HIGHT as i32, 0);
          pancurses.add_string("To exit type x");
        },
    }
  }
}
