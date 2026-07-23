pub enum Color {
    Red,
    Blue,
    Green,
    Black,
    White
}

pub trait Renderer {
    fn get_max_x(&self) -> u16;
    fn get_max_y(&self) -> u16;
    fn draw_cell(&mut self, x: u16, y: u16, color: Color);
    fn write_text(&mut self, x: u16, y: u16, color: Color, text: &str);
    fn clear(&mut self);
    fn flush(&mut self);
}

pub trait Renderable {
    fn render(&self, renderer: &mut impl Renderer);
}
