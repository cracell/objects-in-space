extern crate graphics;
extern crate piston;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate shader_version;
extern crate window;

use sdl2_window::Sdl2Window as Window;
use opengl_graphics::Gl;
use shader_version::opengl::OpenGL::_3_2;

use std::cell::RefCell;
use piston::event::{
    self,
    RenderArgs,
    RenderEvent,
    UpdateArgs,
    UpdateEvent,
};

use graphics::{
    Context,
    Rectangle,
    RelativeTransform,
};

pub struct App {
    gl: Gl,        // OpenGL drawing backend
    rotation: f64, // Rotation for the square
    player: Player
}

pub struct Player {
    x: f64,
    y: f64,
    y_v: f64,
    x_v: f64 
}

impl App {
    fn render(&mut self, _: &mut Window, args: &RenderArgs) {
        // Set up a context to draw into.
        let context = &Context::abs(args.width as f64, args.height as f64);
        // Clear the screen
        graphics::clear([0.0,0.0,0.0,1.0], &mut self.gl);

        // Draw a box rotating around the middle of the screen.
        let center_context = &context.trans((args.width / 2) as f64, (args.height / 2 ) as f64)
            .rot_rad(self.rotation)
            .trans(-25.0, -25.0);
        self.render_ship(&center_context, args);
        Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw([self.player.x, self.player.y, 50.0, 50.0], center_context, &mut self.gl);
    }

    fn update(&mut self, _: &mut Window, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        // self.rotation += 2.0 * args.dt;
        self.player.y += self.player.y_v;
        self.player.x += self.player.x_v;
    }

    fn render_ship(&mut self, center_context: &Context, args: &RenderArgs) {
        Rectangle::new([1.0, 1.0, 1.0, 1.0]).draw([-250.0, -250.0, 500.0, 500.0], center_context, &mut self.gl);
    }
}

fn main() {
    // Create an SDL window.
    let window = Window::new(
      _3_2,
      window::WindowSettings {
          title: "Objects in Space".to_string(),
          size: [750, 750],
          fullscreen: false,
          exit_on_esc: true,
          samples: 4,
     }
          );

    let player = Player { x: 0.0, y: 0.0, y_v: 0.0, x_v: 0.0 };

    // Create a new game and run it.
    let mut app = App { gl: Gl::new(_3_2), rotation: 0.0, player: player };

    let window = RefCell::new(window);
    for e in event::events(&window) {
        use piston::event::{ PressEvent, ReleaseEvent };
        use piston::input::Button::Keyboard;
        use piston::input::keyboard::Key;
        

        if let Some(button) = e.press_args() {

            let velocity = 0.9;

            match button { 
                Keyboard(Key::Up) | Keyboard(Key::W) => app.player.y_v = -velocity,
                Keyboard(Key::Down) | Keyboard(Key::S) => app.player.y_v = velocity,
                Keyboard(Key::Left) | Keyboard(Key::A) => app.player.x_v = -velocity,
                Keyboard(Key::Right) | Keyboard(Key::D) => app.player.x_v = velocity,
                _ => () 
            }

        }
        if let Some(button) = e.release_args() {
            use piston::input::Button::Keyboard;
            use piston::input::keyboard::Key;

            if button == Keyboard(Key::Up) || button == Keyboard(Key::Down) || button ==
                Keyboard(Key::W) || button == Keyboard(Key::S) {
                app.player.y_v = 0.0
            } else if button == Keyboard(Key::Left) || button == Keyboard(Key::Right) || 
                button == Keyboard(Key::A) || button == Keyboard(Key::D) {
                app.player.x_v = 0.0
            }
        }
        if let Some(r) = e.render_args() {
            app.render(&mut *window.borrow_mut(), &r);
        }
        if let Some(u) = e.update_args() {
            app.update(&mut *window.borrow_mut(), &u);
        }
    }
}
