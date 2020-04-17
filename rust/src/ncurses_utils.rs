use ncurses::*;

const BOARD_LENGHT: u32 = 11;

pub fn init_ncurses() {
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

pub fn getchar_timeout(delay: i32) {
  timeout(delay);
}

pub fn clear_screen() {
  clear();
}

pub fn getchar() -> i32 {
  getch()
}

pub fn release_screen() {
  endwin();
}
