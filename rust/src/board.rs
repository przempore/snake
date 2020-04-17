use crate::ncurses_utils::*;

const BOARD_SIZE: usize = 40;
// const BOARD_WIDTH: u32 = 36;
// const BOARD_HIGHT: u32 = 36;
const BOARD_LENGHT: u32 = 11;

pub fn print_board() {
  let line = std::iter::repeat("=").take(BOARD_SIZE).collect::<String>();
  let boarder = String::from("||");
  const BOARDERS_OVERLAPS: usize = 4;
  let between_boarders = std::iter::repeat(" ")
                                .take(BOARD_SIZE - BOARDERS_OVERLAPS)
                                .collect::<String>();
  let boarders = format!("{}{}{}", boarder, between_boarders, boarder);

  add_string(&line);
  let mut n = 1;
  for _ in 0..BOARD_LENGHT {
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
