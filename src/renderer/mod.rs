use std::io::{Stdout, Write, stdout};

use crossterm::{QueueableCommand, cursor::{self, MoveTo}, queue, style::{self, Color as CTCOLOR, Print, ResetColor, SetBackgroundColor, Stylize}, terminal};

use crate::games::{rendering::{Renderer, Color}};

pub struct CrossTermRenderer {
    stdout: Stdout,
    screen_x: u16,
    screen_y: u16
}

impl CrossTermRenderer {
    pub fn new() -> CrossTermRenderer {
        terminal::enable_raw_mode().unwrap();

        let mut new_stdout = stdout();
        new_stdout.queue(cursor::Hide).unwrap();


        let (cols, rows) = terminal::size().unwrap();

        CrossTermRenderer {
            stdout: new_stdout,
            screen_x: cols,
            screen_y: rows 
        }
    }
}

impl Drop for CrossTermRenderer {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();

        self.clear();
        self.flush();

        self.stdout.queue(cursor::Show).unwrap();
    }
}

impl Renderer for CrossTermRenderer {
    fn draw_cell(&mut self, x: u16, y: u16, color: Color) {
        let ct_color = match color {
            Color::Red => CTCOLOR::Red,
            Color::Blue => CTCOLOR::Blue,
            Color::Green => CTCOLOR::Green,
            Color::White => CTCOLOR::White,
            Color::Black => CTCOLOR::Black,
        };

        queue!(self.stdout, MoveTo(x * 2, y), SetBackgroundColor(ct_color), Print("  "), ResetColor).unwrap();
    }
    
    fn write_text(&mut self, x: u16, y: u16, color: Color, text: &str) {
        let lines: Vec<&str> = text.lines().collect();

        let width = lines.iter().map(|line| line.chars().count()).max().unwrap_or(0);
        let height = lines.len();

        let start_x = (x * 2).saturating_sub((width as u16) / 2);
        let start_y = y.saturating_sub((height as u16) / 2);

        for (i, line) in lines.iter().enumerate() {
            let colored_line = match color {
                Color::Red => line.red(),
                Color::Blue => line.blue(),
                Color::Green => line.green(),
                Color::White => line.white(),
                Color::Black => line.black(),
            };

            queue!(self.stdout, MoveTo(start_x, start_y + i as u16), style::PrintStyledContent(colored_line)).unwrap();
        }
    }

    fn clear(&mut self) {
        self.stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
    }

    fn get_max_x(&self) -> u16 {
        self.screen_x / 2
    }

    fn get_max_y(&self) -> u16 {
        self.screen_y
    }

    fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }
}
