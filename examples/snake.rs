use sfml::{
    graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable},
    system::{Clock, Time, Vector2f, Vector2i},
    window::{Event, Key, Style},
};

trait Simple: Default {
    fn load(&mut self, path: &str);
    fn exit(&self, path: &str);
    fn handle(&mut self, key: Key);
    fn update(&mut self, delta_time: Time);
    fn view(&self, canvas: &mut RenderWindow);
    fn run(title: &str) {
        if let Ok(mut window) = RenderWindow::new(
            (crate::SCREEN_WIDTH as u32, crate::SCREEN_HEIGHT as u32),
            title,
            Style::CLOSE,
            &Default::default(),
        ) {
            let path = format!("{}/.config/{title}", std::env::var("HOME").unwrap());
            let mut model = Self::default();
            model.load(&path);
            window.set_vertical_sync_enabled(true);
            let mut clock = Clock::start().unwrap();
            while window.is_open() {
                while let Some(event) = window.poll_event() {
                    match event {
                        Event::Closed => window.close(),
                        Event::KeyPressed { code, .. } => {
                            model.handle(code);
                        }
                        _ => {}
                    }
                }
                model.update(clock.restart());
                model.view(&mut window);
            };
            model.exit(&path);
        }
    }
}
// --- Constants ---
const SQUARE_SIZE: i32 = 20;

// --- Model: Structs and Enums for the game's state ---

/// Represents the four possible directions the snake can move.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// The main Model struct that holds the entire state of the game.
struct Model {
    snake: Vec<Vector2i>,
    direction: Direction,
    food: Vector2i,
    score: i32,
    game_over: bool,
    // SFML's `Time` struct is used for precise time-based movement.
    time_since_last_move: Time,
    move_interval: Time,
}

impl Default for Model {
    /// Creates a new, initial state for the game.
    fn default() -> Self {
        let mut model = Self {
            snake: Vec::new(),
            direction: Direction::Right,
            food: Vector2i::new(0, 0), // Will be placed properly in reset
            score: 0,
            game_over: false,
            time_since_last_move: Time::ZERO,
            move_interval: Time::milliseconds(120),
        };
        model.reset();
        model
    }
}

impl Simple for Model {
    /// Handles keyboard input to change the snake's direction.
    fn handle(&mut self, key: Key) {
        if self.game_over {
            if key == Key::Enter {
                self.reset();
            }
            return;
        }

        if let Some(dir) = match key {
            Key::Up | Key::W if self.direction != Direction::Down => Some(Direction::Up),
            Key::Down | Key::S if self.direction != Direction::Up => Some(Direction::Down),
            Key::Left | Key::A if self.direction != Direction::Right => Some(Direction::Left),
            Key::Right | Key::D if self.direction != Direction::Left => Some(Direction::Right),
            _ => None,
        } {
            self.direction = dir;
        }
    }
    /// Handles all game logic based on the passage of time.
    fn update(&mut self, delta_time: Time) {
        if self.game_over {
            return;
        }

        self.time_since_last_move += delta_time;

        // It's not time to move yet, so we exit the function.
        if self.time_since_last_move < self.move_interval {
            return;
        }

        // It is time to move, so reset the timer.
        self.time_since_last_move = Time::ZERO;

        // --- Snake Movement ---
        let mut new_head = self.snake[0];
        match self.direction {
            Direction::Up => new_head.y -= 1,
            Direction::Down => new_head.y += 1,
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x += 1,
        }

        // --- Collision Detection ---
        // 1. Wall collision
        if new_head.x < 0
            || new_head.x >= SCREEN_WIDTH as i32 / SQUARE_SIZE
            || new_head.y < 0
            || new_head.y >= SCREEN_HEIGHT as i32 / SQUARE_SIZE
        {
            self.game_over = true;
            return;
        }
        // 2. Self collision
        // We check from the 1st element because the head can't collide with itself.
        for segment in self.snake.iter().skip(1) {
            if new_head == *segment {
                self.game_over = true;
                return;
            }
        }
        // --- Update Snake Body ---
        self.snake.insert(0, new_head);
        if new_head == self.food {
            self.score += 1;
            self.place_food();
        } else {
            // If we didn't eat, remove the tail to simulate movement.
            self.snake.pop();
        }
    }

    fn view(&self, window: &mut RenderWindow) {
        window.clear(Color::rgb(253, 246, 227));
        let mut square = RectangleShape::new();
        square.set_size(Vector2f::new(SQUARE_SIZE as f32, SQUARE_SIZE as f32));

        for (i, segment) in self.snake.iter().enumerate() {
            square.set_position(Vector2f::new(
                (segment.x * SQUARE_SIZE) as f32,
                (segment.y * SQUARE_SIZE) as f32,
            ));
            square.set_fill_color(if i == 0 {
                Color::rgb(133, 153, 0)
            } else {
                Color::rgb(42, 161, 152)
            });
            window.draw(&square);
        }

        // Draw the food
        square.set_position(Vector2f::new(
            (self.food.x * SQUARE_SIZE) as f32,
            (self.food.y * SQUARE_SIZE) as f32,
        ));
        square.set_fill_color(Color::rgb(220, 50, 47));
        window.draw(&square);
        window.set_title(&format!(
            "Snake - Score: {} {}",
            self.score,
            if self.game_over {
                "GAME OVER - Press Enter"
            } else {
                ""
            }
        ));
        window.display();
    }
}

impl Model {
    /// Resets the game to its starting state.
    fn reset(&mut self) {
        self.snake.clear();
        self.snake.push(Vector2i::new(10, 10));
        self.snake.push(Vector2i::new(9, 10));
        self.snake.push(Vector2i::new(8, 10));
        self.direction = Direction::Right;
        self.score = 0;
        self.game_over = false;
        self.time_since_last_move = Time::ZERO;
        self.place_food();
    }

    /// Places the food in a random location not occupied by the snake.
    fn place_food(&mut self) {
        let mut rng = rand::rng();
        loop {
            let x = rng.random_range(0..(SCREEN_WIDTH as i32 / SQUARE_SIZE));
            let y = rng.random_range(0..(SCREEN_HEIGHT as i32 / SQUARE_SIZE));
            let new_pos = Vector2i::new(x, y);
            if !self.snake.contains(&new_pos) {
                self.food = new_pos;
                break;
            }
        }
    }
}

fn main() {
    Model::run();
}
