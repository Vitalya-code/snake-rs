extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

use rand::Rng;
use sdl2::video::Window;
use sdl2::Sdl;

static COLORS: &'static [Color] = &[
    Color::RGB(34, 40, 49),
    Color::RGB(57, 62, 70),
    Color::RGB(0, 173, 181),
    Color::RGB(238, 238, 238),
];

static CELL_SIZE: u16 = 50;

#[derive(Copy, Clone, PartialEq, Debug)]
struct Position {
    x: u16,
    y: u16,
}

struct SnakeSegment {
    position: Position,
}

#[derive(Copy, Clone)]
struct Borders {
    w: u16,
    h: u16,
}

struct Apple {
    position: Position,
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl Apple {
    fn new(borders: Borders) -> Apple {
        let mut apple = Apple {
            position: Position { x: 0, y: 0 },
        };

        apple.relocate(borders);

        return apple;
    }

    fn relocate(&mut self, borders: Borders) {
        let mut rng = rand::thread_rng();

        let mut x: u16 = rng.gen_range(0..borders.w / CELL_SIZE);
        let mut y: u16 = rng.gen_range(0..borders.h / CELL_SIZE);
        x = x * CELL_SIZE;
        y = y * CELL_SIZE;

        self.position = Position { x, y }
    }
}

struct Snake {
    length: u16,
    position: Position,
    segments: Vec<SnakeSegment>,
    direction: Direction,
    speed: u16,
    borders: Borders,
}

impl Snake {
    fn new(position: Position, borders: Borders) -> Snake {
        let position = position;
        let initial_segment = SnakeSegment { position };

        Snake {
            length: 0,
            position,
            segments: vec![initial_segment],
            direction: Direction::None,
            speed: CELL_SIZE.into(),
            borders,
        }
    }

    fn grow(&mut self, apple_old_position: Position) {
        self.length += 1;
        self.segments.push(SnakeSegment {
            position: Position {
                x: apple_old_position.x,
                y: apple_old_position.y,
            },
        });
    }

    fn change_direction(&mut self, new_direction: Direction) {
        if self.direction == Direction::Left && new_direction == Direction::Right {
            return;
        }
        if self.direction == Direction::Right && new_direction == Direction::Left {
            return;
        }
        if self.direction == Direction::Up && new_direction == Direction::Down {
            return;
        }
        if self.direction == Direction::Down && new_direction == Direction::Up {
            return;
        }
        self.direction = new_direction;
    }

    fn move_forward(&mut self) {
        if self.position.x <= self.borders.w || self.position.y <= self.borders.h {
            match self.direction {
                // The `saturating_sub` method is used to handle potential underflow situations
                // and ensure that the value remains within the valid range for a u16.
                Direction::Up => self.position.y = self.position.y.saturating_sub(self.speed),
                Direction::Down => self.position.y = self.position.y.saturating_add(self.speed),
                Direction::Right => self.position.x = self.position.x.saturating_add(self.speed),
                Direction::Left => self.position.x = self.position.x.saturating_sub(self.speed),
                Direction::None => (),
            }
        }

        // remove the first element
        self.segments.remove(0);

        // add last position to the end
        self.segments.push(SnakeSegment {
            position: Position {
                x: self.position.x,
                y: self.position.y,
            },
        });
    }
}

fn main() {
    // clear the window
    let sdl: Sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let window: Window = video_subsystem
        .window("Snake", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let borders = Borders { w: 800, h: 600 };

    game(window, sdl, borders)
}

fn game(window: Window, sdl: Sdl, borders: Borders) {
    let mut snake = Snake::new(Position { x: 0, y: 0 }, borders);
    let mut apple = Apple::new(borders);

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    'running: loop {
        // this shit is broken someone fix this section please

        for event in event_pump.poll_iter() {
            // check if its closing event

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => snake.change_direction(Direction::Up),

                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => snake.change_direction(Direction::Down),

                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => snake.change_direction(Direction::Right),

                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => snake.change_direction(Direction::Left),
                _ => {}
            }
        }

        // move snake forward
        snake.move_forward();

        // and then check apple collision
        if check_collision(snake.position, apple.position) == true {
            snake.grow(apple.position);
            apple.relocate(borders);
        } else {
            // and check snakes parts collision
            for (i, segment) in snake.segments.iter().enumerate() {
                if i != snake.length.try_into().unwrap() {
                    if snake.position == segment.position {
                        break 'running;
                    }
                }
            }
        }

        // Clear the screen
        canvas.set_draw_color(COLORS[0]);
        canvas.clear();

        // draw barriers
        // todo!();

        // draw food
        canvas.set_draw_color(COLORS[3]);
        canvas
            .fill_rect(Rect::new(
                apple.position.x.try_into().unwrap(),
                apple.position.y.try_into().unwrap(),
                CELL_SIZE.into(),
                CELL_SIZE.into(),
            ))
            .unwrap();

        // draw all parts of the snake
        for (i, segment) in snake.segments.iter().enumerate() {
            if i == snake.length.try_into().unwrap() {
                canvas.set_draw_color(COLORS[2]);
            } else {
                canvas.set_draw_color(COLORS[1]);
            };

            canvas
                .fill_rect(Rect::new(
                    segment.position.x.try_into().unwrap(),
                    segment.position.y.try_into().unwrap(),
                    CELL_SIZE.into(),
                    CELL_SIZE.into(),
                ))
                .unwrap();
        }

        // draw the score
        // todo!();

        canvas.present();

        ::std::thread::sleep(Duration::from_millis(200));
    }
}

fn check_collision(object1: Position, object2: Position) -> bool {
    // function that check collision between two object. True: collision detected, False: no collision
    object1.x == object2.x && object1.y == object2.y
}

// fn render(canvas: Canvas<Window>, snake: Snake, apple: Apple) {}
