use std::char;
use ncurses::*;

pub struct Snake{
    x: i32,
    y: i32,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            x: 5,
            y: 5,
        }
    }

    pub fn print(&self) {
        mv(self.y, self.x);
        addstr("&");
        mv(13, 0);
        addstr(&format!("x = {}, y = {}", self.x, self.y));
    }

    pub fn move_it(&mut self, key:char) -> bool {
      match key {
          'w' => { if self.y > 1 { self.y -= 1; } },
          'a' => { if self.x > 2 { self.x -= 1; } },
          's' => { if self.y < 11 { self.y += 1; } },
          'd' => { if self.x < 37 { self.x += 1; } },
          'x' => { return true; }
          _ => { return false; }
      }
      return false;
    }
}

