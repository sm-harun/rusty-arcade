use std::{io::{Stdout, Write, stdout}, time::{Duration, Instant}};
use crossterm::{QueueableCommand, cursor::{self, MoveTo}, event::{Event, KeyCode, KeyEventKind, poll, read}, execute, style::{Color, Print, ResetColor, SetBackgroundColor}, terminal::{self, Clear, ClearType, enable_raw_mode}};

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
        GameState {
            snake: Snake::new(max_x / 2, max_y / 2),
            apple: Apple::new(max_x, max_y),
            direction: Direction::Right
        }
    }

    fn update(&mut self) {
        if self.apple.position == self.snake.position {
            self.apple.regenrate();
            self.snake.move_snake(&self.direction, true);
        } else {
            self.snake.move_snake(&self.direction, false);
        }
    }
}

struct Renderer {
    stdout: Stdout
}

impl Renderer {
    fn new() -> Renderer {
        Renderer {
            stdout: stdout()
        }
    }

    fn draw_cell(&mut self, x: u16, y: u16, color: Color) {
        execute!(self.stdout, MoveTo(x * 2, y), SetBackgroundColor(color), Print("  "), ResetColor).unwrap();
    }

    fn update(&mut self, game: &GameState) {
        self.draw_cell(game.apple.position.x, game.apple.position.y, Color::Red);
        self.draw_cell(game.snake.position.x, game.snake.position.y, Color::Green);

        for cell in &game.snake.body {
            self.draw_cell(cell.x, cell.y, Color::Blue);
        }
    }

    fn clear(&mut self) {
        self.stdout.queue(Clear(ClearType::All)).unwrap();
    }

    fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    fn hide_cursor(&mut self) {
        self.stdout.queue(cursor::Hide).unwrap();
    }
}

fn main() {
    enable_raw_mode().unwrap();

    let (cols, rows) = terminal::size().unwrap();

    let mut render = Renderer::new();
    render.hide_cursor();

    let mut game = GameState::new(cols / 2, rows);

    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    'game: loop {
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Char('w') => game.direction = Direction::Up,
                        KeyCode::Char('a') => game.direction = Direction::Left,
                        KeyCode::Char('s') => game.direction = Direction::Down,
                        KeyCode::Char('d') => game.direction = Direction::Right,
                        KeyCode::Esc => break 'game,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick += tick_rate;

            render.flush();


            game.update();

            render.update(&game);
            render.clear();

        }
    }

    render.clear();
    render.flush();
}

