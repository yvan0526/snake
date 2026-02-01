use crate::snake::Snake;
use rand::prelude::IndexedRandom;
use std::io::Stdout;
use std::io::Write;
use termion::color;
use termion::event::Key;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;

#[derive(PartialEq)]
pub enum GameState {
    Running,
    GameOver,
    Win,
}

struct Size {
    width: u16,
    height: u16,
}

struct Apple {
    x: u16,
    y: u16,
}

pub struct Grid {
    size: Size,
    snake: Snake,
    apple: Apple,
    score: u16,
}

impl Grid {
    pub fn new(width: u16, height: u16) -> Self {
        let mut grid = Self {
            size: Size { width, height },
            snake: Snake::new(width / 2, height / 2, 2),
            apple: Apple { x: 1, y: 1 },
            score: 0,
        };

        grid.new_apple();
        grid
    }

    fn new_apple(&mut self) {
        let mut rng = rand::rng();
        let rand_apple_x: Vec<u16> = (1..self.size.width).collect();
        let rand_apple_y: Vec<u16> = (2..self.size.height).collect();

        let mut apple_x = *rand_apple_x.choose(&mut rng).unwrap();
        let mut apple_y = *rand_apple_y.choose(&mut rng).unwrap();

        while self.snake.is_on(apple_x, apple_y) {
            apple_x = *rand_apple_x.choose(&mut rng).unwrap();
            apple_y = *rand_apple_y.choose(&mut rng).unwrap();
        }

        self.apple = Apple {
            x: apple_x,
            y: apple_y,
        }
    }

    pub fn show_border(&self, screen: &mut AlternateScreen<RawTerminal<Stdout>>, difficulty: u8) {
        for i in 1..(self.size.width + 1) * 2 {
            write!(
                screen,
                "{}{} ",
                termion::cursor::Goto(i, 1),
                color::Bg(color::LightYellow)
            )
            .unwrap();
            write!(
                screen,
                "{}{} ",
                termion::cursor::Goto(i, self.size.height + 1),
                color::Bg(color::LightYellow)
            )
            .unwrap();
        }

        for i in 2..self.size.height {
            write!(
                screen,
                "{}{} ",
                termion::cursor::Goto(1, i),
                color::Bg(color::LightYellow)
            )
            .unwrap();
            write!(
                screen,
                "{}{} ",
                termion::cursor::Goto((self.size.width + 1) * 2, i),
                color::Bg(color::LightYellow)
            )
            .unwrap();
        }

        write!(
            screen,
            "{}{}{}Difficulty : {}",
            termion::cursor::Goto(2, self.size.height),
            color::Bg(color::LightYellow),
            color::Fg(color::Black),
            difficulty
        )
        .unwrap();

        screen.flush().unwrap();
    }

    pub fn show(&self, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
        write!(
            screen,
            "{}{}{}Score : {}",
            termion::cursor::Goto(2, 1),
            color::Bg(color::LightYellow),
            color::Fg(color::Black),
            self.score
        )
        .unwrap();

        write!(
            screen,
            "{}{}{}🬫🬛",
            termion::cursor::Goto(self.apple.x * 2, self.apple.y),
            color::Bg(color::Reset),
            color::Fg(color::Red)
        )
        .unwrap();

        self.snake.show(screen);

        screen.lock().flush().unwrap();
    }

    pub fn control(&mut self, key: Key) {
        match key {
            Key::Left => {
                self.snake.go_left();
            }
            Key::Up => {
                self.snake.go_up();
            }
            Key::Right => {
                self.snake.go_right();
            }
            Key::Down => {
                self.snake.go_down();
            }
            _ => {}
        }
    }

    pub fn update(&mut self) -> GameState {
        let died = !self
            .snake
            .update_position(self.size.width, self.size.height);

        if died {
            GameState::GameOver
        } else if (self.size.width - 2) * (self.size.height - 2) == self.score + 2 {
            GameState::Win
        } else {
            if self.snake.eat(self.apple.x, self.apple.y) {
                self.snake.grow();
                self.new_apple();
                self.score += 1;
            }

            GameState::Running
        }
    }

    pub fn snake_to_clean(&self) -> (u16, u16) {
        self.snake.to_clean()
    }

    pub fn clean_snake(
        &mut self,
        screen: &mut AlternateScreen<RawTerminal<Stdout>>,
        x: u16,
        y: u16,
    ) {
        write!(
            screen,
            "{}{}{}{}",
            termion::cursor::Goto(x * 2, y),
            color::Bg(color::Reset),
            color::Fg(color::Green),
            "  "
        )
        .unwrap();
    }

    pub fn score(&self) -> u16 {
        self.score
    }
}
