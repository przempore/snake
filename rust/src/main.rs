mod board;
mod ncurses_utils;
mod point;
mod snake;

use ncurses_utils::*;
use pancurses::Input;
use snake::*;
use std::env;

const DEELAY_FOR_KEY: i32 = 250;

fn main() {
    let pancurses = Pancurses::new();
    pancurses.init_ncurses();
    loop {
        if !game_loop(&pancurses) {
            break;
        }
    }
    wait_for_x_to_exit(&pancurses);
    pancurses.release_screen();
}

fn game_loop(pancurses: &Pancurses) -> bool {
    let args: Vec<String> = env::args().collect();
    let mut dont_eat_self = false;
    if args.len() > 1 {
        dont_eat_self = args[1] == "--dont_eat_self";
    }
    let mut snake = Snake::new(dont_eat_self);

    pancurses.getchar_timeout(DEELAY_FOR_KEY);
    loop {
        pancurses.clear_screen();
        snake.print(&pancurses);
        pancurses.move_pointer((board::HIGHT + 1) as i32, 0);
        pancurses.add_string("To exit hit DEL!");
        match pancurses.getchar() {
            Some(Input::Character(x)) => snake.change_dir(x),
            Some(Input::KeyDC) => break,
            _ => (),
        }
        if !snake.move_it(&pancurses) {
            break;
        }

        pancurses.move_pointer(board::HIGHT as i32, 0);
    }

    pancurses.clear_line(board::HIGHT as i32, 0);
    pancurses.add_string("To restart type R");
    pancurses.getchar_timeout(-1);
    if let Some(Input::Character(input)) = pancurses.getchar() {
        input == 'r'
    } else {
        false
    }
}

fn wait_for_x_to_exit(pancurses: &Pancurses) {
    pancurses.clear_line((board::HIGHT + 1) as i32, 0);
    pancurses.clear_line(board::HIGHT as i32, 0);
    pancurses.add_string("To exit hit DEL!");
    pancurses.getchar_timeout(-1);
    loop {
        match pancurses.getchar() {
            Some(Input::KeyDC) => break,
            _ => (),
        }
    }
}
