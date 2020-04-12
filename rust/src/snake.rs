use std::char;
use ncurses::*;
use std::ops::AddAssign;

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

pub struct Snake{
    position: Point,
    direction: Point,
}

impl Snake {
    pub fn new() -> Snake {
        Snake{position: Point {x: 5, y: 5}, direction: Point{x: 0, y: -1}}
    }

    pub fn print(&self) {
        mv(self.position.y, self.position.x);
        addstr("X");
        mv(13, 0);
        addstr(&format!("x = {}, y = {}", self.position.x, self.position.y));
    }

    fn check_boarder_collisions(&self) -> bool {
        if self.position.x < 2 || self.position.x > 37 { return true; }
        if self.position.y < 1 || self.position.y > 11 { return true; }

        return false;
    }

    pub fn move_it(&mut self) {
        if self.check_boarder_collisions() {
            return;
        }

        self.position += self.direction;
    }

    pub fn change_dir(&mut self, key:char) -> bool {
      match key {
          'w' => { self.direction = Point { x: 0, y: -1 }; },
          'a' => { self.direction = Point { x: -1, y: 0 }; },
          's' => { self.direction = Point { x: 0, y: 1 }; },
          'd' => { self.direction = Point { x: 1, y: 0 }; },

          'x' => { return true; }
          _ => { return false; }
      }
      return false;
    }
}

