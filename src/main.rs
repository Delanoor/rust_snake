extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use std::collections::LinkedList;
use std::iter::FromIterator;

#[derive(Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        use graphics::*;

        let green: [f32; 4] = [0.0, 0.8, 0.0, 0.6];

        self.gl.draw(arg.viewport(), |_c, gl| {
            clear(green, gl);
        });

        self.snake.render(&mut self.gl, arg);
    }

    fn update(&mut self) {
        self.snake.update();
    }
}

struct Snake {
    body: LinkedList<(i32, i32)>,
    dir: Direction,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let size = 20;

        let squares: Vec<graphics::types::Rectangle> = self
            .body
            .iter()
            .map(|&(x, y)| {
                


                  rectangle::square((x * size) as f64, (y * size) as f64, size as f64)
            })
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares
                .into_iter()
                .for_each(|square| graphics::rectangle(red, square, transform, gl));
        });
    }

    fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

        match self.dir {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }

        self.body.push_front(new_head);
        self.body.pop_back().unwrap();
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.dir.clone();

        self.dir = match btn {
            &Button::Keyboard(Key::K) if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::J) if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::H) if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::L) if last_direction != Direction::Left => Direction::Right,
            _ => last_direction,
        };
    }
}

fn main() {
    let open_gl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Snake Game", [400, 400])
        .graphics_api(open_gl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(open_gl),
        snake: Snake {
            body: LinkedList::from_iter((vec![(0, 0), (0, 1)]).into_iter()),
            dir: Direction::Right,
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(u) = e.update_args() {
            game.update();
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.snake.pressed(&k.button);
            }
        }
    }
}
