// mod ncurses_utils;
mod snake;
mod board;

use std::char;
use ncurses::*;
// use ncurses_utils::*;
use snake::*;
use board::*;

const DEELAY_FOR_KEY: i32 = 500;

fn main() {
  initscr();
  raw();

  keypad(stdscr(), true);
  noecho();

  let mut snake = Snake::new();

  loop {
      clear();
      print_board();
      snake.move_it();
      snake.print();
      timeout(DEELAY_FOR_KEY);
      if snake.change_dir(getch() as u8 as char) {
          break;
      }

      // NCursesUtils::move_pointer(13, 0);
      mv(13, 0);
  }

  getch();
  endwin();

}
