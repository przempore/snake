use pancurses::*;

pub struct Pancurses {
    window: Window,
}

impl Pancurses {
    pub fn new() -> Self {
        let p = Pancurses { window: initscr() };
        raw();

        p
    }

    pub fn init_ncurses(&self) {
        self.window.keypad(true);
        noecho();
    }

    pub fn add_string(&self, txt: &str) {
        self.window.printw(&txt);
    }

    pub fn move_pointer(&self, y: i32, x: i32) -> i32 {
        self.window.mv(y, x)
    }

    pub fn getchar_timeout(&self, delay: i32) {
        self.window.timeout(delay);
    }

    pub fn clear_screen(&self) {
        self.window.clear();
    }

    pub fn getchar(&self) -> Option<Input> {
        self.window.getch()
    }

    pub fn release_screen(&self) {
        endwin();
    }

    pub fn clear_line(&self, y: i32, x: i32) -> i32 {
        self.move_pointer(y, x);
        self.window.clrtoeol()
    }
}
