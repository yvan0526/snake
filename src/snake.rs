use std::io::Stdout;
use std::io::Write;
use termion::color;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;

#[derive(PartialEq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(PartialEq, Clone, Copy)]
struct Cell {
    x: u16,
    y: u16,
}

pub struct Snake {
    direction: Direction,
    body: Vec<Cell>,
    last_cell: Cell,
}

impl Snake {
    pub fn new(x: u16, y: u16, size: u16) -> Self {
        let mut body: Vec<Cell> = vec![Cell { x, y }];
        for i in 1..size {
            body.push(Cell { x: x - i, y });
        }

        Self {
            direction: Direction::Right,
            body,
            last_cell: Cell { x, y: y - size - 1 },
        }
    }

    fn go(&mut self, direction: Direction, opposite: Direction) {
        if opposite != self.direction {
            self.direction = direction;
        }
    }

    pub fn go_left(&mut self) {
        self.go(Direction::Left, Direction::Right);
    }

    pub fn go_up(&mut self) {
        self.go(Direction::Up, Direction::Down);
    }

    pub fn go_right(&mut self) {
        self.go(Direction::Right, Direction::Left);
    }

    pub fn go_down(&mut self) {
        self.go(Direction::Down, Direction::Up);
    }

    pub fn eat(&self, apple_x: u16, apple_y: u16) -> bool {
        self.body[0].x == apple_x && self.body[0].y == apple_y
    }

    pub fn grow(&mut self) {
        self.body.push(self.last_cell);
    }

    pub fn update_position(&mut self, grid_width: u16, grid_height: u16, grid_has_wall: bool) -> bool {
        // Position de la nouvelle tête
        let mut new_head = Cell {
            x: self.body[0].x,
            y: self.body[0].y,
        };

        // Déplacement de la nouvelle tête du serpent
        match self.direction {
            Direction::Left => {
                if new_head.x > 1 {
                    new_head.x -= 1;
                } else {
                    new_head.x = grid_width - 1;
                }
            }
            Direction::Up => {
                if new_head.y > 2 {
                    new_head.y -= 1;
                } else {
                    new_head.y = grid_height - 1;
                }
            }
            Direction::Right => {
                if new_head.x < grid_width - 1 {
                    new_head.x += 1;
                } else {
                    new_head.x = 1;
                }
            }
            Direction::Down => {
                if new_head.y < grid_height - 1 {
                    new_head.y += 1;
                } else {
                    new_head.y = 2;
                }
            }
        }

        // Game over si la grille a des murs et que la nouvelle tête en touche un
        if grid_has_wall
            && (new_head.x <= 1 || new_head.x >= grid_width - 1
            || new_head.y <= 2 || new_head.y >= grid_height - 1) {
            return false
        }

        // Game over si le nouvelle tête est sur une partie du corps sauf le bout de la queue à enlever
        for i in 1..self.body.len() - 1 {
            if self.body[i] == new_head {
                return false;
            }
        }

        // Retrait du bout de la queue
        self.last_cell = self.body.pop().unwrap();

        // Ajout de la nouvelle tête au début du serpent
        self.body.insert(0, new_head);

        true
    }

    pub fn show(&self, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
        for i in 0..self.body.len() {
            let body = if i == 0 {
                // Tête du serpent
                "🬴🬸"
            } else if i % 2 == 0 {
                // Corps 1
                "▓▓"
            } else {
                // Corps 2
                "▒▒"
            };

            // Affiche la partie du corps
            write!(
                screen,
                "{}{}{}",
                termion::cursor::Goto(self.body[i].x * 2, self.body[i].y),
                color::Fg(color::Green),
                body
            )
            .unwrap();
        }

        screen.flush().unwrap();
    }

    pub fn to_clean(&self) -> (u16, u16) {
        // Bout de la queue à effacer
        (self.body[self.body.len() - 1].x, self.body[self.body.len() - 1].y)
    }

    pub fn is_on(&self, x: u16, y: u16) -> bool {
        for i in 0..self.body.len() {
            if x == self.body[i].x && y == self.body[i].y {
                return true;
            }
        }

        false
    }

    pub fn length(&self) -> usize {
        self.body.len()
    }
}
