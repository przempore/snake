use std::char;

use crate::point::*;
use crate::board;

use crate::ncurses_utils::*;

use std::collections::LinkedList;

const STARTING_LENGTH: u32 = 3;
const SIGN: &str = "O";
const FAIL_SIGN: &str = "X";
const START_POSITION: Point = Point {
    x: (board::WIDTH / 2) as i32,
    y: (board::HIGHT / 2) as i32
};

const STEP_UP: Point = Point { x: 0, y: -1 };
const STEP_LEFT: Point = Point { x: -1, y: 0 };
const STEP_DOWN: Point = Point { x: 0, y: 1 };
const STEP_RIGHT: Point = Point { x: 1, y: 0 };

pub struct Snake {
    body: LinkedList<Point>,
    step_direction: Point,
    body_sign: std::string::String,
    board: board::Board,
}

impl Snake {
    pub fn new() -> Self {
        let mut snake  = Snake {
            body: LinkedList::new(),
            step_direction: STEP_RIGHT,
            body_sign: String::from(SIGN),
            board: board::Board::new(),
        };
        for _ in 0..STARTING_LENGTH {
            snake.body.push_front(snake.get_head() + STEP_RIGHT);
        }

        snake
    }

    pub fn print(&mut self) {
        self.board.print_board();
        let mut body_iter = self.body.iter();
        loop {
            if let Some(pos) = body_iter.next() {
                    move_pointer(pos.y, pos.x);
                    add_string(&self.body_sign);
            } else {
                break
            }
        }
        self.print_head_position();
        self.print_score();
    }

    pub fn move_it(&mut self) -> bool {
        if self.check_boarder_collisions() {
            self.print_collision();
            self.print_fail_head();
            return false;
        } else if self.check_food_collision() {
            self.board.draw_new_food();
            self.body.push_front(self.get_head() + self.step_direction);
        }

        self.body.push_front(self.get_head() + self.step_direction);
        self.body.pop_back();

        return true;
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

    fn check_food_collision(&self) -> bool {
        self.get_head() == self.board.get_food()
    }

    fn check_boarder_collisions(&self) -> bool {
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

    fn print_collision(&self) {
        let collision_text = format!("Board collision!Game over!");
        move_pointer((board::HIGHT / 2) as i32,
                     (board::WIDTH / 2 - (collision_text.len() / 2)) as i32);
        add_string(&collision_text);
        let text = format!("score: {}", self.body.len());
        move_pointer((board::HIGHT / 2 + 1)  as i32,
                     (board::WIDTH / 2 - (text.len() / 2)) as i32);
        add_string(&text);
    }

    fn print_head_position(&self) {
        move_pointer(board::HIGHT as i32, 0);
        add_string(&format!("x = {}, y = {}", self.get_head().x, self.get_head().y));
    }

    fn print_fail_head(&self) {
        move_pointer(self.get_head().y, self.get_head().x);
        add_string(FAIL_SIGN);
    }

    fn print_score(&self) {
        let text = format!("score: {}", self.body.len());
        move_pointer(board::HIGHT as i32, (board::WIDTH - text.len()) as i32);
        add_string(&text);
        move_pointer(0, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_move_one_step_right() {
        let mut snake = Snake::new();
        assert_eq!(START_POSITION + Point {x: 3, y: 0}, snake.get_head());

        assert!(snake.move_it());
        assert_eq!(START_POSITION + Point {x: 4, y: 0}, snake.get_head());
    }

    #[test]
    fn should_collide_right_boarder_test() {
        let mut snake = Snake::new();
        for _ in 0..15 { // todo: find value from board::
            assert!(snake.move_it());
        }
        assert!(!snake.move_it());
    }

    #[test]
    fn should_collide_top_boarder_test() {
        let mut snake = Snake::new();
        snake.change_dir('w');
        for _ in 0..6 { // todo: find value from board::
            assert!(snake.move_it());
        }
        assert!(!snake.move_it());
    }

    #[test]
    fn should_collide_bottom_boarder_test() {
        let mut snake = Snake::new();
        snake.change_dir('s');
        for _ in 0..6 { // todo: find value from board::
            assert!(snake.move_it());
        }
        assert!(!snake.move_it());
    }

    #[test]
    fn should_collide_left_boarder_test() {
        let mut snake = Snake::new();
        snake.change_dir('a');
        for _ in 0..22 { // todo: find value from board::
            assert!(snake.move_it());
        }
        assert!(!snake.move_it());
    }
}

