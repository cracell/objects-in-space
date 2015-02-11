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
    y_v: bool
}

impl App {
    fn render(&mut self, _: &mut Window, args: &RenderArgs) {
        // Set up a context to draw into.
        let context = &Context::abs(args.width as f64, args.height as f64);
        // Clear the screen
        graphics::clear([0.0,1.0,0.0,1.0], &mut self.gl);

        // Draw a box rotating around the middle of the screen.
        let center_context = &context.trans((args.width / 2) as f64, (args.height / 2 ) as f64)
            .rot_rad(self.rotation)
            .trans(-25.0, -25.0);
        Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw([self.player.x, self.player.y, 100.0, 50.0], center_context, &mut self.gl);
        // Rectangle::new([1.0, 0.5, 0.0, 1.0]).draw([110.0, 0.0, 100.0, 50.0], center_context, &mut self.gl);
        // Rectangle::new([1.0, 0.5, 1.0, 0.5]).draw([50.0, 0.0, 70.0, 50.0], center_context, &mut self.gl);
    }

    fn update(&mut self, _: &mut Window, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        // self.rotation += 2.0 * args.dt;
        if self.player.y_v {
            self.player.y -= 0.1
        }
    }
}

fn main() {
    // Create an SDL window.
    // let window = Window::new(
      // _3_2,
      // window::WindowSettings::default()
          // );
    let window = Window::new(
      _3_2,
      window::WindowSettings {
          title: "Objects in Space".to_string(),
          size: [500, 500],
          fullscreen: false,
          exit_on_esc: true,
          samples: 4,
     }
          );

    let mut player = Player { x: 0.0, y: 0.0, y_v: false };

    // Create a new game and run it.
    let mut app = App { gl: Gl::new(_3_2), rotation: 0.0, player: player };

    let window = RefCell::new(window);
    for e in event::events(&window) {
        use piston::event::{ PressEvent, ReleaseEvent };

        if let Some(button) = e.press_args() {
            use piston::input::Button::Keyboard;
            use piston::input::keyboard::Key;

            if button == Keyboard(Key::Up) {
                app.player.y_v = true
            }

        }
        if let Some(button) = e.release_args() {
            use piston::input::Button::Keyboard;
            use piston::input::keyboard::Key;

            if button == Keyboard(Key::Up) {
                app.player.y_v = false 
            }

        }
        if let Some(r) = e.render_args() {
            app.render(&mut *window.borrow_mut(), &r);
        }
        if let Some(u) = e.update_args() {
            app.update(&mut *window.borrow_mut(), &u);
        }
    }

    // let window = RefCell::new(window);
    // for e in piston::events(&window) {
        // use piston::event::{ PressEvent };
        // println!("Hello, world.");

        // if let Some(button) = e.press_args() {
            // use piston::input::Button::Keyboard;
            // use piston::input::keyboard::Key;

            // if button == Keyboard(Key::G) {
                // app.player.y -= 10.0
            // }

        // }
    // }
}

// fn handleKey(key: Button, app: &mut APP) {
    // match key {
        // Keyboard(input::keyboard::Up) => { app.player.y -= 10.0 }
        // _ => {}
    // }
// }
