extern crate ncurses;
extern crate rand;

use ncurses::*;

const BOARD_LENGHT: u32 = 11;

fn init_ncurses() {
  initscr();
  raw();

  keypad(stdscr(), true);
  noecho();
}

pub fn finish_ncurses() {
  getch();
  endwin();
}

pub fn add_string(txt: &str) {
  addstr(&txt);
}

pub fn move_pointer(y: i32, x: i32) -> i32 {
  mv(y, x)
}

