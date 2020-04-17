use std::char;
use std::ops::{AddAssign, Add};

use crate::ncurses_utils::*;

const START_POSITION: Point = Point {x: 5, y:5};
const STEP_UP: Point = Point { x: 0, y: -1 };
const STEP_LEFT: Point = Point { x: -1, y: 0 };
const STEP_DOWN: Point = Point { x: 0, y: 1 };
const STEP_RIGHT: Point = Point { x: 1, y: 0 };

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

pub struct Snake{
    position: Point,
    step_direction: Point,
}

impl Snake {
    pub fn new() -> Snake {
        Snake{position: START_POSITION, step_direction: STEP_RIGHT}
    }

    pub fn print(&self) {
        move_pointer(self.position.y, self.position.x);
        add_string("X");
        move_pointer(13, 0);
        add_string(&format!("x = {}, y = {}", self.position.x, self.position.y));
    }

    fn check_boarder_collisions(&self) -> bool {
        if self.position.x < 2 || self.position.x > 37 { return true; }
        if self.position.y < 1 || self.position.y > 11 { return true; }

        return false;
    }

    pub fn get_possition(&self) -> Point {
        self.position
    }

    pub fn move_it(&mut self) {
        if self.check_boarder_collisions() {
            return;
        }

        self.position += self.step_direction;
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
        assert_eq!(START_POSITION, snake.get_possition());

        snake.move_it();
        assert_eq!(START_POSITION + Point {x: 1, y: 0}, snake.get_possition());
    }
}

