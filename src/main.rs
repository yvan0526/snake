mod grid;
mod snake;

use crate::grid::{GameState, Grid};
use clap::Parser;
use clap_num::number_range;
use std::io::{Stdout, Write, stdout};
use std::process::exit;
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
    #[arg(short, long, default_value_t = false)]
    wall: bool
}

fn main() {
    // Arguments du programme
    let args = Args::parse();

    // Taille du terminal
    let (width, height) = terminal_size().unwrap();

    // Taille minimale du terminal
    if width < 32 || height < 6 {
        eprintln!("Terminal minimal size: 32x6, current size: {width}x{height}");
        exit(1);
    }

    // Nouvelle portée pour l'écran alternatif
    {
        // Écran alternatif
        let mut screen = stdout()
            .into_raw_mode()
            .unwrap()
            .into_alternate_screen()
            .unwrap();

        // Initialise l'écran et affiche l'écran de démarrage
        init_screen(&mut screen);
        splash_screen(&mut screen, width, height, args.difficulty);

        // Lecteur des touches du clavier
        let mut keys = termion::async_stdin().keys();

        // Nouvelle grille de jeux de la taille du terminal
        let mut grid = Grid::new(width / 2, height, args.wall, args.difficulty);
        if args.wall {
            grid.show_wall(&mut screen);
        }
        grid.show_border(&mut screen);

        let mut game_state = GameState::Running;
        while game_state == GameState::Running || game_state == GameState::Pause {
            // Récupération de la dernière touche appuyée dans le buffer
            let mut input = None;
            while let Some(Ok(key)) = keys.next() {
                input = Some(key);
            }

            // Gestion des actions associées aux touches du clavier
            if let Some(key) = input {
                match key {
                    // Met fin à la partie
                    termion::event::Key::Esc => {
                        game_state = GameState::GameOver;
                    },
                    // Met le jeu en pause ou reprend la partie en pause
                    termion::event::Key::Char(' ') => {
                        if game_state == GameState::Pause {
                            game_state = GameState::Running;
                        } else {
                            game_state = GameState::Pause;
                        }
                    },
                    // Augmente la difficulté
                    termion::event::Key::PageDown => {
                        grid.difficulty_down();
                    },
                    // Diminue la difficulté
                    termion::event::Key::PageUp => {
                        grid.difficulty_up();
                    },
                    // Défini la direction du serpent
                    _ => grid.control(key),
                }
            }

            if game_state == GameState::Running {
                game_state = grid.update();
                grid.clean_snake(&mut screen);
            }

            // Affiche la grille
            grid.show(&mut screen);

            // Attend jusqu'à la prochaine frame
            thread::sleep(sleep_time(grid.difficulty()));
        }

        if game_state == GameState::GameOver {
            game_over_screen(&mut screen, width, height, grid.score());
        } else if game_state == GameState::Win {
            win_screen(&mut screen, width, height)
        }
        reset_screen(&mut screen);
    }

    exit(0);
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

fn game_over_screen(
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

fn win_screen(screen: &mut AlternateScreen<RawTerminal<Stdout>>, width: u16, height: u16) {
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

fn sleep_time(difficulty: u8) -> Duration {
    Duration::from_millis(200 - ((difficulty - 1) * 10) as u64)
}
