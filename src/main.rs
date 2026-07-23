use std::{
    io::{
        Stdout, Write, stdout
    },
    time::{
        Duration, Instant
    }
};

use crossterm::{
    QueueableCommand,
    cursor::{
        self, MoveTo
    },
    event::{
        Event, KeyCode, KeyEventKind, poll, read
    },
    execute,
    style::{
        self, Color, Print, ResetColor, SetBackgroundColor, Stylize
    },
    terminal::{
        self, Clear, ClearType, enable_raw_mode
    }
};

use crate::games::snake;

mod games;

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

    fn update(&mut self, game: &snake::SnakeGame) {
        if *game.state() == snake::GameState::Over {
            let text = "
                  ██████   █████  ███    ███ ███████     ███████ ██   ██ ████████ ██████  
                 ██       ██   ██ ████  ████ ██          ██   ██ ██   ██ ██       ██   ██ 
                 ██   ███ ███████ ██ ████ ██ █████       ██   ██ ██   ██ ████████ ██████  
                 ██    ██ ██   ██ ██  ██  ██ ██          ██   ██  ██ ██  ██       ██   ██ 
                  ██████  ██   ██ ██      ██ ███████     ███████   ███    ███████ ██   ██ 
            ";
            let text = text.replace('\n', "\r\n");

            self.write(&text.to_string(), self.max_x, 1);

            self.write(&format!("Score: {}", game.score()).to_string(), 16, 8);

            return ();
        }

        self.draw_cell(*game.apple().position_x(), *game.apple().position_y(), Color::Red);
        self.draw_cell(*game.snake().position_x(), *game.snake().position_y(), Color::Green);

        for cell in game.snake().body() {
            self.draw_cell(*cell.x(), *cell.y(), Color::Blue);
        }

        if *game.state() == snake::GameState::Paused {
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

    let mut game = snake::SnakeGame::new(cols / 2, rows);

    game.start_game();

    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    'game: loop {
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Char('w') => game.change_direction(snake::Direction::Up),
                        KeyCode::Char('a') => game.change_direction(snake::Direction::Left),
                        KeyCode::Char('s') => game.change_direction(snake::Direction::Down),
                        KeyCode::Char('d') => game.change_direction(snake::Direction::Right),
                        KeyCode::Enter => game.restart_game(),
                        KeyCode::Char(' ') =>  game.toggle_pause(),
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

            render.write(&format!("Score: {}", game.score()).to_string(), 4, 2);

            game.update();
            render.update(&game);

            render.flush();
        }

    }

    render.clear();
    render.flush();
}
