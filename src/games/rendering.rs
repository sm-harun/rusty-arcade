pub enum Color {
    Red,
    Blue,
    Green,
    Black,
    White
}

pub trait Renderer {
    fn draw_cell(&mut self, x: u16, y: u16, color: Color);
    fn write_text(&mut self, x: u16, y: u16, color: Color, text: &str);
    fn flush(&mut self);
}

pub trait Renderable {
    fn render(&self, renderer: &mut impl Renderer);
}
