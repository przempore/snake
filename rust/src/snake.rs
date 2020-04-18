use std::char;

use crate::point::*;

use crate::ncurses_utils::*;

const START_POSITION: Point = Point {x: 5, y:5};
const STEP_UP: Point = Point { x: 0, y: -1 };
const STEP_LEFT: Point = Point { x: -1, y: 0 };
const STEP_DOWN: Point = Point { x: 0, y: 1 };
const STEP_RIGHT: Point = Point { x: 1, y: 0 };

pub struct Snake{
    head_position: Point,
    step_direction: Point,
}

impl Snake {
    pub fn new() -> Snake {
        Snake{head_position: START_POSITION, step_direction: STEP_RIGHT}
    }

    pub fn print(&self) {
        move_pointer(self.head_position.y, self.head_position.x);
        add_string("X");
        move_pointer(13, 0);
        add_string(&format!("x = {}, y = {}", self.head_position.x, self.head_position.y));
    }

    fn check_boarder_collisions(&self) -> bool {
        if self.head_position.x < 2 || self.head_position.x > 37 { return true; }
        if self.head_position.y < 1 || self.head_position.y > 11 { return true; }

        return false;
    }

    pub fn get_head_possition(&self) -> Point {
        self.head_position
    }

    pub fn move_it(&mut self) {
        if self.check_boarder_collisions() {
            return;
        }

        self.head_position += self.step_direction;
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
        assert_eq!(START_POSITION, snake.get_head_possition());

        snake.move_it();
        assert_eq!(START_POSITION + Point {x: 1, y: 0}, snake.get_head_possition());
    }
}

