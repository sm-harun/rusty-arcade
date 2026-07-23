use crate::{games::rendering::{Color, Renderable, Renderer }};

#[derive(PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq)]
pub enum GameState {
    Running,
    Paused,
    Over
}

#[derive(PartialEq, Clone)]
pub struct Position {
    x: u16,
    y: u16
}

impl Position {
    pub fn x(&self) -> &u16 {
        &self.x
    }

    pub fn y(&self) -> &u16 {
        &self.y
    }
}

pub struct Snake {
    position: Position,
    body: Vec<Position>,
    direction: Direction,
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
            direction: Direction::Right,
            body: vec![Position { x: x - 1, y }],
            max_x,
            max_y
        }
    }

    pub fn position_x(&self) -> &u16 {
        &self.position.x
    }

    pub fn position_y(&self) -> &u16 {
        &self.position.y
    }

    pub fn body(&self) -> &Vec<Position> {
        &self.body
    }

    fn move_snake(&mut self) {
        self.body.insert(0, self.position.clone());

        match self.direction {
            Direction::Up => if self.position.y != 0 { self.position.y -= 1 },
            Direction::Down => if self.position.y != self.max_y { self.position.y += 1 },
            Direction::Left => if self.position.x != 0 { self.position.x -= 1 },
            Direction::Right => if self.position.x != self.max_x { self.position.x += 1 }
        }

        print!("  X: {}", self.position.x);
        print!("  Y: {}", self.position.y);
    }

    fn change_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => if self.direction != Direction::Down { self.direction = Direction::Up },
            Direction::Down => if self.direction != Direction::Up { self.direction = Direction::Down },
            Direction::Right => if self.direction != Direction::Left { self.direction = Direction::Right },
            Direction::Left => if self.direction != Direction::Right { self.direction = Direction::Left },
        }
    }

    fn remove_last_cell(&mut self) {
        self.body.pop();
    }
}

pub struct Apple {
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

    pub fn position_x(&self) -> &u16 {
        &self.position.x
    }

    pub fn position_y(&self) -> &u16 {
        &self.position.y
    }

    fn regenrate(&mut self) {
        self.position.x = rand::random_range(0..=self.max_x);
        self.position.y = rand::random_range(0..=self.max_y);
    }
}

pub struct SnakeGame {
    snake: Snake,
    apple: Apple,
    score: u32,
    state: GameState,
    max_x: u16,
    max_y: u16
}

impl SnakeGame {
    pub fn new(max_x: u16, max_y: u16) -> SnakeGame {
        SnakeGame {
            snake: Snake::new(max_x / 2, max_y / 2, max_x, max_y),
            apple: Apple::new(max_x, max_y),
            score: 0,
            state: GameState::Over,
            max_x,
            max_y
        }
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn score(&self) -> &u32 {
        &self.score
    }

    pub fn snake(&self) -> &Snake {
        &self.snake
    }

    pub fn apple(&self) -> &Apple {
        &self.apple
    }

    pub fn start_game(&mut self) {
        self.snake = Snake::new(self.max_x / 2, self.max_y / 2, self.max_x, self.max_y);
        self.apple = Apple::new(self.max_x, self.max_y);
        self.score = 0;

        self.state = GameState::Running;
    }

    pub fn restart_game(&mut self) {
        if self.state == GameState::Over {
            self.start_game();
        }
    }

    pub fn toggle_pause(&mut self) {
        if self.state == GameState::Paused {
            self.state = GameState::Running;
        } else if self.state == GameState::Running {
            self.state = GameState::Paused;
        }
    }

    fn end_game(&mut self) {
        self.state = GameState::Over;
    }

    pub fn update_game(&mut self) {
        if self.state == GameState::Paused || self.state == GameState::Over { return (); }

        if self.snake.body.iter().any(|cell| *cell == self.snake.position) {
            self.end_game();
        }

        self.snake.move_snake();

        if self.apple.position == self.snake.position {
            self.apple.regenrate();
            self.score += 1;

            return ();
        } 

        self.snake.remove_last_cell();
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.snake.change_direction(direction);
    }
}

impl Renderable for SnakeGame {
    fn render(&self, renderer: &mut impl Renderer) {
        renderer.clear();

        if self.state == GameState::Over {
            let text = "
 ██████   █████  ███    ███ ███████     ███████ ██   ██ ████████ ██████ 
██       ██   ██ ████  ████ ██          ██   ██ ██   ██ ██       ██   ██
██   ███ ███████ ██ ████ ██ █████       ██   ██ ██   ██ ████████ ██████ 
██    ██ ██   ██ ██  ██  ██ ██          ██   ██  ██ ██  ██       ██   ██
 ██████  ██   ██ ██      ██ ███████     ███████   ███    ███████ ██   ██
            ";

            renderer.write_text(self.max_x / 2, self.max_y / 3, Color::White, &text.to_string());
            renderer.write_text(self.max_x / 2, self.max_y / 2, Color::White, &format!("Score: {}", self.score).to_string(),);

            renderer.flush();
            return ();
        }

        renderer.draw_cell(*self.apple.position_x(), *self.apple.position_y(), Color::Red);
        renderer.draw_cell(*self.snake.position_x(), *self.snake.position_y(), Color::Green);

        for cell in self.snake().body() {
            renderer.draw_cell(*cell.x(), *cell.y(), Color::Blue);
        }

        if self.state == GameState::Paused {
            let text = "
 ██████   █████  ███    ███ ███████     ██████   █████  ██    ██ ███████ ███████ ██████ 
██       ██   ██ ████  ████ ██          ██   ██ ██   ██ ██    ██ ██      ██      ██   ██
██   ███ ███████ ██ ████ ██ █████       ██████  ███████ ██    ██ ███████ █████   ██   ██
██    ██ ██   ██ ██  ██  ██ ██          ██      ██   ██ ██    ██      ██ ██      ██   ██
 ██████  ██   ██ ██      ██ ███████     ██      ██   ██  ██████  ███████ ███████ ██████
            ";

            renderer.write_text(self.max_x / 2, self.max_y / 2, Color::White, &text.to_string());
        }

        renderer.write_text(4, 2, Color::White, &format!("Score: {}", self.score).to_string());

        renderer.flush();
    }
}
