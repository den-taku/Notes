#[macro_use]
extern crate conrod;

use conrod::backend::glium::glium::{self, Surface};

const WIDTH: i32 = 400;
const HEIGHT: i32 = 640;

widget_ids!(struct Ids { text });

fn main() {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Hello conrod")
        .with_dimensions(WIDTH, HEIGHT);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    let ids = Ids::new(ui.widget_id_generator());

    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let mut renderer = conrod::backend::gluim::Renderer::new(&display).unwrap();

    'main: loop {
        // Render the `Ui` and then display if on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 1.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unrwap();
        }
    }
}
