// mod ncurses_utils;
extern crate ncurses;

use ncurses::*;

const BOARD_SIZE: usize = 40;
// const BOARD_WIDTH: u32 = 36;
// const BOARD_HIGHT: u32 = 36;
const BOARD_LENGHT: u32 = 11;

// use rand::*;

pub fn print_board() {
  let line = std::iter::repeat("=").take(BOARD_SIZE).collect::<String>();
  let boarder = String::from("||");
  const BOARDERS_OVERLAPS: usize = 4;
  let between_boarders = std::iter::repeat(" ")
                                .take(BOARD_SIZE - BOARDERS_OVERLAPS)
                                .collect::<String>();
  let boarders = format!("{}{}{}", boarder, between_boarders, boarder);

  // NCursesUtils::add_string(&line);
  addstr(&line);
  let mut n = 1;
  for _ in 0..BOARD_LENGHT {
    mv(n, 0);
    // NCursesUtils::move_pointer(n,  0);
    // NCursesUtils::add_string(&boarders);
    addstr(&boarders);
    n += 1;
  }
  // NCursesUtils::move_pointer(n, 0);
    mv(n, 0);
  // NCursesUtils::add_string(&line);
  addstr(&line);
}


// pub fn get_new_food(start: u32, end: u32) -> u32 {
//   let mut rng = rand::thread_rng();
//   rng.gen_range(start, end) as u32
// }
