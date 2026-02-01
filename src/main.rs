mod grid;
mod snake;

use crate::grid::{GameState, Grid};
use clap::Parser;
use clap_num::number_range;
use std::io::{Stdout, Write, stdout};
use std::thread;
use std::time::Duration;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::{AlternateScreen, IntoAlternateScreen};
use termion::{color, terminal_size};

fn between_1_and_20(s: &str) -> Result<u8, String> {
    number_range(s, 1, 20)
}

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 1, value_parser=between_1_and_20)]
    difficulty: u8,
}

fn main() {
    let args = Args::parse();
    let sleep = Duration::from_millis(200 - ((args.difficulty as u64) * 10));
    let (width, height) = terminal_size().unwrap();

    {
        let mut screen = stdout()
            .into_raw_mode()
            .unwrap()
            .into_alternate_screen()
            .unwrap();

        init_screen(&mut screen);
        splash_screen(&mut screen, width, height, args.difficulty);

        let mut keys = termion::async_stdin().keys();

        let mut grid = Grid::new(width / 2, height);
        grid.show_border(&mut screen, args.difficulty);

        let mut game_state = GameState::Running;
        while game_state == GameState::Running {
            let mut input = None;
            while let Some(Ok(key)) = keys.next() {
                input = Some(key);
            }

            if let Some(key) = input {
                match key {
                    termion::event::Key::Esc => {
                        game_state = GameState::GameOver;
                        break;
                    }
                    _ => grid.control(key),
                }
            }

            let (to_clean_x, to_clean_y) = grid.snake_to_clean();
            game_state = grid.update();
            if game_state == GameState::Running {
                grid.clean_snake(&mut screen, to_clean_x, to_clean_y);
            }

            grid.show(&mut screen);

            thread::sleep(sleep);
        }

        if game_state == GameState::GameOver {
            game_over(&mut screen, width, height, grid.score());
        } else {
            win(&mut screen, width, height)
        }
        reset_screen(&mut screen);
    }
}

fn init_screen(screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
    write!(screen, "{}", termion::cursor::Hide).unwrap();
    screen.flush().unwrap();
}

fn reset_screen(screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
    write!(
        screen,
        "{}{}{}",
        termion::cursor::Show,
        color::Fg(color::Reset),
        color::Bg(color::Reset)
    )
    .unwrap();
    screen.flush().unwrap();
}

fn splash_screen(
    screen: &mut AlternateScreen<RawTerminal<Stdout>>,
    width: u16,
    height: u16,
    difficulty: u8,
) {
    write!(
        screen,
        "{}{}{}{} Snake Game ! ",
        termion::cursor::Goto(width / 2 - 7, height / 2),
        termion::style::Bold,
        color::Bg(color::LightYellow),
        color::Fg(color::Black)
    )
    .unwrap();

    write!(
        screen,
        "{}{}{}{} Grid size: {}x{} ",
        termion::cursor::Goto(width / 2 - 9, height / 2 + 1),
        termion::style::Reset,
        color::Bg(color::Reset),
        color::Fg(color::LightYellow),
        width / 2,
        height
    )
    .unwrap();

    write!(
        screen,
        "{}{}{}{} Difficulty: {} ",
        termion::cursor::Goto(width / 2 - 8, height / 2 + 2),
        termion::style::Reset,
        color::Bg(color::Reset),
        color::Fg(color::LightYellow),
        difficulty
    )
    .unwrap();

    screen.flush().unwrap();

    thread::sleep(Duration::from_secs(2));
    write!(screen, "{}{}", termion::clear::All, color::Bg(color::Reset)).unwrap();
    screen.flush().unwrap();
}

fn game_over(
    screen: &mut AlternateScreen<RawTerminal<Stdout>>,
    width: u16,
    height: u16,
    score: u16,
) {
    write!(
        screen,
        "{}{}{}{} GAME OVER ! ",
        termion::cursor::Goto(width / 2 - 6, height / 2),
        termion::style::Bold,
        color::Fg(color::Black),
        color::Bg(color::Red)
    )
    .unwrap();

    write!(
        screen,
        "{}{}{}{} Score : {} ",
        termion::cursor::Goto(width / 2 - 5, height / 2 + 1),
        termion::style::Reset,
        color::Fg(color::Black),
        color::Bg(color::Red),
        score
    )
    .unwrap();

    screen.flush().unwrap();
    thread::sleep(Duration::from_secs(2));
}

fn win(screen: &mut AlternateScreen<RawTerminal<Stdout>>, width: u16, height: u16) {
    write!(
        screen,
        "{}{}{}{} WIN ! ",
        termion::cursor::Goto(width / 2 - 3, height / 2),
        color::Fg(color::Black),
        color::Bg(color::LightGreen),
        termion::style::Bold
    )
    .unwrap();

    screen.flush().unwrap();
    thread::sleep(Duration::from_secs(2));
}
