use sdl2::pixels::Color;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use rand;

struct CanvasManager {
    event_pump: EventPump,
    canvas1: WindowCanvas,
    canvas2: WindowCanvas
}

impl CanvasManager {
    fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem1 = sdl_context.video().unwrap();
        let video_subsystem2 = sdl_context.video().unwrap();
        let window1 = video_subsystem1.window("Screen", 512, 256)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");
        let window2 = video_subsystem2.window("Screen", 512, 256)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");
        let canvas1 = window1.into_canvas().build()
            .expect("could not make a canvas");
        let canvas2 = window2.into_canvas().build()
            .expect("could not make a canvas");
        let event_pump = sdl_context.event_pump().unwrap();
        CanvasManager {
            event_pump,
            canvas1,
            canvas2
        }
    }

    fn drawing_rectangle(&mut self, rect_width: &u32, rect_height: &u32) {
        let red: u8 = rand::random();
        let green: u8 = rand::random();
        let blue: u8 = rand::random();

        self.canvas1.clear();
        self.canvas2.clear();

        let drawing_color = Color::RGB(red, green, blue);
        self.canvas1.set_draw_color(drawing_color);
        self.canvas2.set_draw_color(drawing_color);

        let square_definition = Rect::new(0, 0, *rect_width, *rect_height);
        let _ = self.canvas1.fill_rect(square_definition);
        let _ = self.canvas2.fill_rect(square_definition);

        self.canvas1.present();
        self.canvas2.present();
    }

    fn render(&mut self, color: Color) {
        self.canvas1.set_draw_color(color);
        self.canvas2.set_draw_color(color);
        self.canvas1.clear();
        self.canvas2.clear();
        self.canvas1.present();
        self.canvas2.present();
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
