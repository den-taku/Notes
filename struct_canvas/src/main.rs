use sdl2::pixels::Color;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use rand;

struct CanvasManager {
    event_pump: EventPump,
    canvas: WindowCanvas
}

impl CanvasManager {
    fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Screen", 512, 256)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");
        let canvas = window.into_canvas().build()
            .expect("could not make a canvas");
        let event_pump = sdl_context.event_pump().unwrap();
        CanvasManager {
            event_pump,
            canvas
        }
    }

    fn drawing_rectangle(&mut self, rect_width: &u32, rect_height: &u32) {
        let red: u8 = rand::random();
        let green: u8 = rand::random();
        let blue: u8 = rand::random();

        self.canvas.clear();

        let drawing_color = Color::RGB(red, green, blue);
        self.canvas.set_draw_color(drawing_color);

        let square_definition = Rect::new(0, 0, *rect_width, *rect_height);
        let _ = self.canvas.fill_rect(square_definition);

        self.canvas.present();
    }

    fn render(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
        self.canvas.present();
    }
}

fn main() {
    let mut canvas = CanvasManager::new();
    let mut i = 0;
    'running: loop {
        for event in canvas.event_pump.poll_iter() {
            match event {
                Event::Quit{..} => {
                    break 'running
                },
                _ => {}
            }
        }
        i = (i + 1) % 255;
        canvas.render(Color::RGB(i, 64, 255 - i));
        canvas.drawing_rectangle(&30, &30)
    }
}
