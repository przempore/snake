use rand;

use crate::ncurses_utils::*;
use crate::point::*;
use rand::Rng;

pub const WIDTH: usize = 40;
pub const SIDE_BOARDER_SIZE: usize = 2;
const WIDTH_INSIDE_BOARD: usize = WIDTH - 2 * SIDE_BOARDER_SIZE;
pub const HIGHT: usize = 13;
pub const TOP_BOTTOM_BOARDER_SIZE: usize = 1;
const HIGHT_INSIDE_BOARD: usize = HIGHT - 2 * TOP_BOTTOM_BOARDER_SIZE;
const FOOD_SIGN: &str = "&";

pub struct Board {
    food: Point,
}

impl Board {
    pub fn new() -> Board {
        Board {
            food: Board::draw_food(
                (SIDE_BOARDER_SIZE as u32, (WIDTH - SIDE_BOARDER_SIZE) as u32),
                (
                    TOP_BOTTOM_BOARDER_SIZE as u32,
                    (HIGHT - TOP_BOTTOM_BOARDER_SIZE) as u32,
                ),
            ),
        }
    }

    pub fn print_board(&mut self, pancurses: &Pancurses) {
        let line = std::iter::repeat("=").take(WIDTH).collect::<String>();
        let boarder = String::from("||");
        let between_boarders = std::iter::repeat(" ")
            .take(WIDTH_INSIDE_BOARD)
            .collect::<String>();
        let boarders = format!("{}{}{}", boarder, between_boarders, boarder);

        pancurses.add_string(&line);
        let mut n = 1;
        for _ in 0..HIGHT_INSIDE_BOARD {
            pancurses.move_pointer(n, 0);
            pancurses.add_string(&boarders);
            n += 1;
        }
        pancurses.move_pointer(n, 0);
        pancurses.add_string(&line);
        pancurses.move_pointer(self.food.y, self.food.x);
        pancurses.add_string(&FOOD_SIGN);
    }

    pub fn draw_new_food(&mut self) -> Point {
        self.food = Board::draw_food(
            (SIDE_BOARDER_SIZE as u32, (WIDTH - SIDE_BOARDER_SIZE) as u32),
            (
                TOP_BOTTOM_BOARDER_SIZE as u32,
                (HIGHT - TOP_BOTTOM_BOARDER_SIZE) as u32,
            ),
        );
        self.food
    }

    pub fn get_food(&self) -> Point {
        self.food
    }

    fn draw_food(range_x: (u32, u32), range_y: (u32, u32)) -> Point {
        let (start_x, end_x) = range_x;
        let (start_y, end_y) = range_y;
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen_range(start_x, end_x) as i32,
            y: rng.gen_range(start_y, end_y) as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_range_be_between() {
        Board::draw_food(
            (SIDE_BOARDER_SIZE as u32, (WIDTH - SIDE_BOARDER_SIZE) as u32),
            (
                TOP_BOTTOM_BOARDER_SIZE as u32,
                (HIGHT - TOP_BOTTOM_BOARDER_SIZE) as u32,
            ),
        );
    }
}
