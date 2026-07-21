use std::{io::{Stdout, Write, stdout}, time::Duration};
use crossterm::{QueueableCommand, cursor::{self, MoveTo}, event::{Event, KeyCode, poll, read}, execute, style::{Color, Print, ResetColor, SetBackgroundColor}, terminal::{self, Clear, ClearType, enable_raw_mode}};

enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq, Clone)]
struct Position {
    x: u16,
    y: u16
}

struct Snake {
    position: Position,
    body: Vec<Position>
}

impl Snake {
    fn new(x: u16, y: u16) -> Snake {
        Snake {
            position: Position {
                x,
                y
            },
            body: Vec::new()
        }
    }

    fn draw_snake(&self, stdout: &mut Stdout) {
        draw_cell(stdout, self.position.x, self.position.y);

        for cell in &self.body {
            draw_cell(stdout, cell.x, cell.y);
        }
    }

    fn move_snake(&mut self, direction: &Direction, eating: bool) {
        self.body.insert(0, self.position.clone());

        match direction {
            Direction::Up => self.position.y -= 1,
            Direction::Down => self.position.y += 1,
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1
        }

        if !eating {
            self.body.pop();
        }
    }
}

struct Apple {
    position: Position,
    max_x: u16,
    max_y: u16
}

impl Apple {
    fn new(max_x: u16, max_y: u16) -> Apple {
        Apple {
            position: Position {
                x:  rand::random_range(0..=max_x),
                y: rand::random_range(0..=max_y),
            },
            max_x,
            max_y
        }
    }

    fn draw(&self, stdout: &mut Stdout) {
        execute!(stdout, MoveTo(self.position.x * 2, self.position.y), SetBackgroundColor(Color::Red), Print("  "), ResetColor).unwrap();
    }
    
    fn regenrate(&mut self) {
        self.position.x = rand::random_range(0..=self.max_x);
        self.position.y = rand::random_range(0..=self.max_y);
    }
}

struct GameState {
    snake: Snake,
    apple: Apple,
    direction: Direction
}

impl GameState {
    fn new(max_x: u16, max_y: u16) -> GameState {
        return GameState {
            snake: Snake::new(max_x / 2, max_y / 2),
            apple: Apple::new(max_x, max_y),
            direction: Direction::Right
        };
    }

    fn update(&mut self, stdout: &mut Stdout) {
        self.apple.draw(stdout);
        self.snake.draw_snake(stdout);

        if self.apple.position == self.snake.position {
            self.apple.regenrate();
            self.snake.move_snake(&self.direction, true);
        } else {
            self.snake.move_snake(&self.direction, false);
        }
    }
}

fn main() {
    enable_raw_mode().unwrap();

    let (cols, rows) = terminal::size().unwrap();
    let mut stdout = stdout();
    stdout.queue(cursor::Hide).unwrap();

    let mut game = GameState::new(cols / 2, rows);

    loop {
        if poll(Duration::from_millis(100)).unwrap() {
            match read().unwrap() {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Char('w') => game.direction = Direction::Up,
                        KeyCode::Char('s') => game.direction = Direction::Down,
                        KeyCode::Char('a') => game.direction = Direction::Left,
                        KeyCode::Char('d') => game.direction = Direction::Right,
                        KeyCode::Esc => break,
                        _ => {}
                    }
                },
                _ => print!("Random Event")
            }
        }

        stdout.flush().unwrap();

        game.update(&mut stdout);

        clear_terminal(&mut stdout);
    }

    clear_terminal(&mut stdout);
    goto(&mut stdout, 0, 0);
    stdout.flush().unwrap();
}

fn clear_terminal(stdout: &mut Stdout) {
    stdout.queue(Clear(ClearType::All)).unwrap();
}

fn goto(stdout: &mut Stdout, x: u16, y: u16) {
    stdout.queue(cursor::MoveTo(x, y)).unwrap();
}

fn draw_cell(stdout: &mut Stdout, x: u16, y: u16) {
    execute!(stdout, MoveTo(x * 2, y), SetBackgroundColor(Color::Blue), Print("  "), ResetColor).unwrap();
}
