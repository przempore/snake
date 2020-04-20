use ncurses::*;

pub fn init_ncurses() {
  initscr();
  raw();

  keypad(stdscr(), true);
  noecho();
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

pub fn clear_line(y: i32, x: i32) -> i32 {
  move_pointer(y, x);
  clrtoeol()
}
