use std::{
    time::{
        Duration, Instant
    }
};

use crossterm::event::{ Event, KeyCode, KeyEventKind, poll, read };

use crate::{
    games::{rendering::{Renderable, Renderer}, snake}, renderer::CrossTermRenderer
};

mod renderer;
mod games;

fn main() {
    let mut renderer = CrossTermRenderer::new();
    let mut game = snake::SnakeGame::new(renderer.get_max_x() - 1, renderer.get_max_y() - 1);

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
            last_tick += tick_rate;

            game.update_game();
            game.render(&mut renderer);
        }
    }
}
