use std::io::{Stdout, Write, stdout};

use crossterm::{QueueableCommand, cursor::{self, MoveTo}, queue, style::{self, Color as CTCOLOR, Print, ResetColor, SetBackgroundColor, Stylize}, terminal::{Clear}};

use crate::games::{rendering::{Renderer, Color}};

pub struct CrossTermRenderer {
    stdout: Stdout,
    max_x: u16,
    max_y: u16
}

impl CrossTermRenderer {
    pub fn new(max_x: u16, max_y: u16) -> CrossTermRenderer {
        CrossTermRenderer {
            stdout: stdout(),
            max_x,
            max_y
        }
    }

    pub fn clear(&mut self) {
        self.stdout.queue(Clear(crossterm::terminal::ClearType::All)).unwrap();
    }

    pub fn hide_cursor(&mut self) {
        self.stdout.queue(cursor::Hide).unwrap();
    }

    pub fn show_cursor(&mut self) {
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
        let colored_text = match color {
            Color::Red => text.red(),
            Color::Blue => text.blue(),
            Color::Green => text.green(),
            Color::White => text.white(),
            Color::Black => text.black()
        };

        self.stdout
          .queue(cursor::MoveTo(x, y)).unwrap()
          .queue(style::PrintStyledContent(colored_text)).unwrap();

        queue!(self.stdout, MoveTo(x, y), style::PrintStyledContent(colored_text)).unwrap();
    }

    fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }
}
