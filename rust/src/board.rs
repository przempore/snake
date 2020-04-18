use crate::ncurses_utils::*;

pub const WIDTH: usize = 40;
pub const SIDE_BOARDER_SIZE: usize = 2;
const WIDTH_INSIDE_BOARD: usize = WIDTH - 2 * SIDE_BOARDER_SIZE;
pub const HIGHT: usize = 13;
pub const TOP_BOTTOM_BOARDER_SIZE: usize = 1;
const HIGHT_INSIDE_BOARD: usize = HIGHT - 2 * TOP_BOTTOM_BOARDER_SIZE;

pub fn print_board() {
  let line = std::iter::repeat("=").take(WIDTH).collect::<String>();
  let boarder = String::from("||");
  let between_boarders = std::iter::repeat(" ")
                                .take(WIDTH_INSIDE_BOARD)
                                .collect::<String>();
  let boarders = format!("{}{}{}", boarder, between_boarders, boarder);

  add_string(&line);
  let mut n = 1;
  for _ in 0..HIGHT_INSIDE_BOARD {
    move_pointer(n,  0);
    add_string(&boarders);
    n += 1;
  }
  move_pointer(n, 0);
  add_string(&line);
}


// pub fn get_new_food(start: u32, end: u32) -> u32 {
//   let mut rng = rand::thread_rng();
//   rng.gen_range(start, end) as u32
// }
