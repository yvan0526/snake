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
    Pause,
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
    has_wall: bool,
    to_clean: (u16, u16),
    difficulty: u8,
}

impl Grid {
    pub fn new(width: u16, height: u16, has_wall: bool, difficulty: u8) -> Self {
        // Serpent au milieu de la grille
        let snake = Snake::new(width / 2, height / 2, 2);

        let mut grid = Self {
            size: Size { width, height },
            apple: Apple { x: 1, y: 1 },
            has_wall,
            difficulty,
            to_clean: snake.to_clean(),
            snake,
        };

        // Nouvelle pomme à une position aléatoire
        grid.new_apple();
        grid
    }

    fn new_apple(&mut self) {
        let mut rng = rand::rng();

        let rand_apple_x: Vec<u16>;
        let rand_apple_y: Vec<u16>;
        if self.has_wall {
            rand_apple_x = (2..self.size.width - 1).collect();
            rand_apple_y = (3..self.size.height - 1).collect();
        } else {
            rand_apple_x = (1..self.size.width).collect();
            rand_apple_y = (2..self.size.height).collect();
        }

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

    pub fn show_border(&self, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
        for i in 1..(self.size.width + 1) * 2 {
            // Ligne du haut
            write!(
                screen,
                "{}{} ",
                termion::cursor::Goto(i, 1),
                color::Bg(color::LightYellow)
            )
            .unwrap();
            // Ligne du bas
            write!(
                screen,
                "{}{} ",
                termion::cursor::Goto(i, self.size.height + 1),
                color::Bg(color::LightYellow)
            )
            .unwrap();
        }

        for i in 2..self.size.height {
            // Colone de gauche
            write!(
                screen,
                "{}{} ",
                termion::cursor::Goto(1, i),
                color::Bg(color::LightYellow)
            )
            .unwrap();
            // Colone de droite
            write!(
                screen,
                "{}{} ",
                termion::cursor::Goto((self.size.width + 1) * 2, i),
                color::Bg(color::LightYellow)
            )
            .unwrap();
        }

        // Score
        write!(
            screen,
            "{}{}{}Score : ",
            termion::cursor::Goto(2, 1),
            color::Bg(color::LightYellow),
            color::Fg(color::Black)
        )
        .unwrap();

        // Score à faire
        let score_to_win = self.score_to_win();
        write!(
            screen,
            "{}{}{}/{}",
            termion::cursor::Goto(11 + Grid::get_num_length(score_to_win), 1),
            color::Bg(color::LightYellow),
            color::Fg(color::Black),
            score_to_win
        )
        .unwrap();

        // Difficulté
        write!(
            screen,
            "{}{}{}Difficulty : ",
            termion::cursor::Goto(2, self.size.height),
            color::Bg(color::LightYellow),
            color::Fg(color::Black)
        )
        .unwrap();

        screen.flush().unwrap();
    }

    pub fn show_wall(&self, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
        for i in 2..self.size.width * 2 + 1 {
            // Mur du haut
            write!(
                screen,
                "{}{} ",
                termion::cursor::Goto(i, 2),
                color::Bg(color::LightBlack)
            )
            .unwrap();
            // Mur du bas
            write!(
                screen,
                "{}{} ",
                termion::cursor::Goto(i, self.size.height - 1),
                color::Bg(color::LightBlack)
            )
            .unwrap();
        }

        for i in 3..self.size.height - 1 {
            // Mur de gauche
            write!(
                screen,
                "{}{}  ",
                termion::cursor::Goto(2, i),
                color::Bg(color::LightBlack)
            )
            .unwrap();
            // Mur de droite
            write!(
                screen,
                "{}{}   ",
                termion::cursor::Goto(self.size.width * 2 - 2, i),
                color::Bg(color::LightBlack)
            )
            .unwrap();
        }

        screen.flush().unwrap();
    }

    pub fn show(&self, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
        // Valeur du score
        write!(
            screen,
            "{}{}{}{}",
            termion::cursor::Goto(
                11 + Grid::get_num_length(self.score_to_win()) - Grid::get_num_length(self.score()),
                1
            ),
            color::Bg(color::LightYellow),
            color::Fg(color::Black),
            self.score()
        )
        .unwrap();

        // Valeur de la difficulté
        write!(
            screen,
            "{}{}{}{} ",
            termion::cursor::Goto(15, self.size.height),
            color::Bg(color::LightYellow),
            color::Fg(color::Black),
            self.difficulty()
        )
        .unwrap();

        // Pomme
        write!(
            screen,
            "{}{}{}🬫🬛",
            termion::cursor::Goto(self.apple.x * 2, self.apple.y),
            color::Bg(color::Reset),
            color::Fg(color::Red)
        )
        .unwrap();

        // Serpent
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
        // Bout de la queue à effacer
        self.to_clean = self.snake.to_clean();

        // Déplace le serpent et détermine s'il meurt
        let died = !self
            .snake
            .update_position(self.size.width, self.size.height, self.has_wall);

        if died {
            GameState::GameOver
        } else if self.score() == self.score_to_win() {
            GameState::Win
        } else {
            if self.snake.eat(self.apple.x, self.apple.y) {
                // Mange une pomme : fait grandir le serpent et fait apparaître une nouvelle pomme
                self.snake.grow();
                self.new_apple();
            }

            GameState::Running
        }
    }

    pub fn clean_snake(&mut self, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
        // Efface le bout de la queue du serpent
        write!(
            screen,
            "{}{}{}",
            termion::cursor::Goto(self.to_clean.0 * 2, self.to_clean.1),
            color::Bg(color::Reset),
            "  "
        )
        .unwrap();
    }

    fn score_to_win(&self) -> u16 {
        if self.has_wall {
            // Taille de la grille moins les bordures, les murs et la taille initiale du serpent
            return (self.size.width - 4) * (self.size.height - 4) - 2;
        }

        // Taille de la grille moins les bordures et la taille initiale du serpent
        (self.size.width - 2) * (self.size.height - 2) - 2
    }

    pub fn score(&self) -> u16 {
        // Score = nombre de pommes mangées : taille du serpent moint sa taille initiale
        self.snake.length() as u16 - 2
    }

    pub fn difficulty(&self) -> u8 {
        self.difficulty
    }

    pub fn difficulty_down(&mut self) {
        if self.difficulty > 1 {
            self.difficulty -= 1;
        }
    }

    pub fn difficulty_up(&mut self) {
        if self.difficulty < 20 {
            self.difficulty += 1;
        }
    }

    fn get_num_length(num: u16) -> u16 {
        let mut i = 1;
        let mut d = 10;
        while num / d > 0 {
            d *= 10;
            i += 1;
        }

        i
    }
}
