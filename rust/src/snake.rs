use std::char;

use crate::point::*;
use crate::board;

use crate::ncurses_utils::*;

use std::collections::LinkedList;

const STARTING_LENGTH: u32 = 3;
const SIGN: &str = "X";
const START_POSITION: Point = Point {x: 15, y:5};

const STEP_UP: Point = Point { x: 0, y: -1 };
const STEP_LEFT: Point = Point { x: -1, y: 0 };
const STEP_DOWN: Point = Point { x: 0, y: 1 };
const STEP_RIGHT: Point = Point { x: 1, y: 0 };

pub struct Snake{
    body: LinkedList<Point>,
    step_direction: Point,
    body_sign: std::string::String,
}

impl Snake {
    pub fn new() -> Snake {
        let mut snake  = Snake{
            body: LinkedList::new(),
            step_direction: STEP_RIGHT,
            body_sign: String::from(SIGN),
        };
        for _ in 0..STARTING_LENGTH {
            snake.body.push_front(snake.get_head() + STEP_RIGHT);
        }

        snake
    }

    fn set_position(pos: &Point) {

    }

    pub fn print(&self) {
        let mut body_iter = self.body.iter();
        loop {
        match body_iter.next() {
            Some(pos) => {
                move_pointer(pos.y, pos.x);
                add_string(&self.body_sign);
            },
            None => { break }
            }
        }
        move_pointer(board::HIGHT as i32, 0);
        add_string(&format!("x = {}, y = {}", self.get_head().x, self.get_head().y));
    }

    fn check_collisions(&self) -> bool {
        self.get_head().x < board::SIDE_BOARDER_SIZE as i32
        || self.get_head().x > (board::WIDTH - board::SIDE_BOARDER_SIZE - 1) as i32
        || self.get_head().y <  board::TOP_BOTTOM_BOARDER_SIZE as i32
        || self.get_head().y > (board::HIGHT - board::TOP_BOTTOM_BOARDER_SIZE * 2) as i32
    }

    fn get_head(&self) -> Point {
        match self.body.front() {
            None => {
                START_POSITION
            },
            Some(head) => {
                *head
            }
        }
    }

    pub fn move_it(&mut self) {
        if self.check_collisions() {
            return;
        }

        self.body.push_front(self.get_head() + self.step_direction);
        self.body.pop_back();
    }

    pub fn change_dir(&mut self, key:char) -> bool {
      match key {
          'w' => { self.step_direction = STEP_UP; },
          'a' => { self.step_direction = STEP_LEFT; },
          's' => { self.step_direction = STEP_DOWN; },
          'd' => { self.step_direction = STEP_RIGHT; },

          'x' => { return true; }
          _ => { return false; }
      }
      return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_move_one_step_right() {
        let mut snake = Snake::new();
        assert_eq!(START_POSITION + Point {x: 3, y: 0}, snake.get_head());

        snake.move_it();
        assert_eq!(START_POSITION + Point {x: 4, y: 0}, snake.get_head());
    }

    #[test]
    fn should_collide_right_boarder_test() {
        let mut snake = Snake::new();
        for _ in 0..19 {
            snake.move_it();
            assert!(!snake.check_collisions());
        }
        snake.move_it();
        assert!(snake.check_collisions());
    }

    #[test]
    fn should_collide_top_boarder_test() {
        let mut snake = Snake::new();
        snake.change_dir('w');
        for _ in 0..4 {
            snake.move_it();
            assert!(!snake.check_collisions());
        }
        snake.move_it();
        assert!(snake.check_collisions());
    }
    
    #[test]
    fn should_collide_bottom_boarder_test() {
        let mut snake = Snake::new();
        snake.change_dir('s');
        for _ in 0..6 {
            snake.move_it();
            assert!(!snake.check_collisions());
        }
        snake.move_it();
        assert!(snake.check_collisions());
    }
    
    #[test]
    fn should_collide_left_boarder_test() {
        let mut snake = Snake::new();
        snake.change_dir('a');
        for _ in 0..16 {
            snake.move_it();
            assert!(!snake.check_collisions());
        }
        snake.move_it();
        assert!(snake.check_collisions());
    }
}

