use std::{io::{Stdout, Write, stdout}, time::{Duration, Instant}};
use crossterm::{QueueableCommand, cursor::{self, MoveTo}, event::{Event, KeyCode, KeyEventKind, poll, read}, execute, style::{self, Color, Print, ResetColor, SetBackgroundColor, Stylize}, terminal::{self, Clear, ClearType, enable_raw_mode}};

enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq)]
enum GameState {
    Running,
    Paused,
    Over
}

#[derive(PartialEq, Clone)]
struct Position {
    x: u16,
    y: u16
}

struct Snake {
    position: Position,
    body: Vec<Position>,
    max_x: u16,
    max_y: u16
}

impl Snake {
    fn new(x: u16, y: u16, max_x: u16, max_y: u16) -> Snake {
        Snake {
            position: Position {
                x,
                y
            },
            body: vec![Position { x: x - 1, y }],
            max_x,
            max_y
        }
    }

    fn move_snake(&mut self, direction: &Direction) {
        self.body.insert(0, self.position.clone());

        match direction {
            Direction::Up => if self.position.y != 0 { self.position.y -= 1 },
            Direction::Down => if self.position.y != self.max_y { self.position.y += 1 },
            Direction::Left => if self.position.x != 0 { self.position.x -= 1 },
            Direction::Right => if self.position.x != self.max_x { self.position.x += 1 }
        }

        // print!("  X: {}", self.position.x);
        // print!("  Y: {}", self.position.y);

    }

    fn remove_last_cell(&mut self) {
        self.body.pop();
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

struct SnakeGame {
    snake: Snake,
    apple: Apple,
    score: u32,
    direction: Direction,
    state: GameState,
    max_x: u16,
    max_y: u16
}

impl SnakeGame {
    fn new(max_x: u16, max_y: u16) -> SnakeGame {
        SnakeGame {
            snake: Snake::new(max_x / 2, max_y / 2, max_x, max_y),
            apple: Apple::new(max_x, max_y),
            score: 0,
            direction: Direction::Right,
            state: GameState::Over,
            max_x,
            max_y
        }
    }

    fn start_game(&mut self) {
        self.snake = Snake::new(self.max_x / 2, self.max_y / 2, self.max_x, self.max_y);
        self.apple = Apple::new(self.max_x, self.max_y);
        self.score = 0;
        self.direction = Direction::Right;

        self.state = GameState::Running;
    }

    fn pause_game(&mut self) {
        self.state = GameState::Paused;
    }

    fn unpause_game(&mut self) {
        self.state = GameState::Running;
    }

    fn end_game(&mut self) {
        self.state = GameState::Over;
    }

    fn update(&mut self) {
        if self.state == GameState::Paused || self.state == GameState::Over { return (); }

        if self.snake.body.iter().any(|cell| *cell == self.snake.position) {
            self.end_game();
        }

        self.snake.move_snake(&self.direction);

        if self.apple.position == self.snake.position {
            self.apple.regenrate();
            self.score += 1;

            return ();
        } 

        self.snake.remove_last_cell();
    }
}

struct Renderer {
    stdout: Stdout,
    max_x: u16,
    max_y: u16
}

impl Renderer {
    fn new(max_x: u16, max_y: u16) -> Renderer {
        Renderer {
            stdout: stdout(),
            max_x,
            max_y
        }
    }

    fn draw_cell(&mut self, x: u16, y: u16, color: Color) {
        execute!(self.stdout, MoveTo(x * 2, y), SetBackgroundColor(color), Print("  "), ResetColor).unwrap();
    }

    fn update(&mut self, game: &SnakeGame) {
        if game.state == GameState::Over {
            let text = "
                  ██████   █████  ███    ███ ███████     ███████ ██   ██ ████████ ██████  
                 ██       ██   ██ ████  ████ ██          ██   ██ ██   ██ ██       ██   ██ 
                 ██   ███ ███████ ██ ████ ██ █████       ██   ██ ██   ██ ████████ ██████  
                 ██    ██ ██   ██ ██  ██  ██ ██          ██   ██  █   █  ██       ██   ██ 
                  ██████  ██   ██ ██      ██ ███████     ███████   ███    ███████ ██   ██ 
            ";
            let text = text.replace('\n', "\r\n");

            self.write(&text.to_string(), self.max_x, 1);

            self.write(&format!("Score: {}", game.score).to_string(), 16, 8);

            return ();
        }

        self.draw_cell(game.apple.position.x, game.apple.position.y, Color::Red);
        self.draw_cell(game.snake.position.x, game.snake.position.y, Color::Green);

        for cell in &game.snake.body {
            self.draw_cell(cell.x, cell.y, Color::Blue);
        }

        if game.state == GameState::Paused {
            let text = "
                  ██████   █████  ███    ███ ███████     ██████   █████  ██    ██ ███████ ███████ ██████ 
                 ██       ██   ██ ████  ████ ██          ██   ██ ██   ██ ██    ██ ██      ██      ██   ██
                 ██   ███ ███████ ██ ████ ██ █████       ██████  ███████ ██    ██ ███████ █████   ██   ██
                 ██    ██ ██   ██ ██  ██  ██ ██          ██      ██   ██ ██    ██      ██ ██      ██   ██
                  ██████  ██   ██ ██      ██ ███████     ██      ██   ██  ██████  ███████ ███████ ██████
            ";
            let text = text.replace('\n', "\r\n");

            self.write(&text.to_string(), self.max_x, 1);
        }

    }

    fn write(&mut self, text: &str, x: u16, y: u16) {
        self.stdout
          .queue(cursor::MoveTo(x, y)).unwrap()
          .queue(style::PrintStyledContent(text.white())).unwrap();
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

    let mut render = Renderer::new(cols / 2, rows);
    render.hide_cursor();

    let mut game = SnakeGame::new(cols / 2, rows);

    game.start_game();

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
                        KeyCode::Enter => {
                            if game.state == GameState::Over {
                                game.start_game();
                            }
                        },
                        KeyCode::Char(' ') =>  {
                            if game.state == GameState::Paused {
                                game.unpause_game();
                            } else if game.state == GameState::Running {
                                game.pause_game();
                            }
                        },
                        KeyCode::Esc => break 'game,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            render.clear();

            last_tick += tick_rate;

            render.write(&format!("Score: {}", game.score).to_string(), 4, 2);

            game.update();
            render.update(&game);

            render.flush();
        }

    }

    render.clear();
    render.flush();
}
