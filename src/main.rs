use std::{
    time::{
        Duration, Instant
    }
};

use crossterm::{
    event::{
        Event, KeyCode, KeyEventKind, poll, read
    },
    terminal
};

use crate::{
    games::{rendering::{Renderable, Renderer}, snake}, renderer::CrossTermRenderer
};

mod renderer;
mod games;

fn main() {
    terminal::enable_raw_mode().unwrap();

    let (cols, rows) = terminal::size().unwrap();

    let mut renderer = CrossTermRenderer::new(cols / 2, rows);
    renderer.hide_cursor();

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
            renderer.clear();

            last_tick += tick_rate;

            game.update_game();
            game.render(&mut renderer);
        }

    }

    renderer.clear();
    renderer.flush();

    renderer.show_cursor();
    terminal::disable_raw_mode().unwrap();
}
