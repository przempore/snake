extern crate ncurses;
use ncurses::*;

pub fn init_ncurses() {
  initscr();
  raw();

  keypad(stdscr(), true);
  noecho();
}

pub fn print_board() {
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

pub fn finish_ncurses() {
  getch();
  endwin();
}
