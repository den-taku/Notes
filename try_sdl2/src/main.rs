use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use std::time::Duration;
use rand;

fn render(canvas: &mut WindowCanvas, color: Color) {
    canvas.set_draw_color(color);
    canvas.clear();
    canvas.present();
}

fn display_rectangle(renderer: &mut WindowCanvas, canvas_width: &u32, canvas_height: &u32) {
    // let red: u8 = rand::random();
    // let green: u8 = rand::random();
    // let blue: u8 = rand::random();

    let red: u8 = 0;
    let green: u8 = 0;  
    let blue: u8 = 0;
    
    renderer.clear();
    let drawing_color = Color::RGB(red, green, blue);
    renderer.set_draw_color(drawing_color);

    let square_definition0 = Rect::new(0, 0, *canvas_width, *canvas_height);
    let square_definition1 = Rect::new(9, 10, *canvas_width, *canvas_height);
    let square_definition2 = Rect::new(0, 2, *canvas_width, *canvas_height);
    let square_definition3 = Rect::new(0, 3, *canvas_width, *canvas_height);
    let square_definition4 = Rect::new(0, 4, *canvas_width, *canvas_height);
    let square_definition5 = Rect::new(0, 5, *canvas_width, *canvas_height);
    let square_definition6 = Rect::new(0, 6, *canvas_width, *canvas_height);
    let square_definition7 = Rect::new(0, 7, *canvas_width, *canvas_height);
    let square_definition8 = Rect::new(30, 8, *canvas_width, *canvas_height);
    let square_definition9 = Rect::new(60, 190, *canvas_width, *canvas_height);
    let square_definition10 = Rect::new(30, 100, *canvas_width, *canvas_height);
    let square_definition11 = Rect::new(300, 110, *canvas_width, *canvas_height);
    let square_definition12 = Rect::new(500, 52, *canvas_width, *canvas_height);
    let square_definition13 = Rect::new(190, 63, *canvas_width, *canvas_height);
    let square_definition14 = Rect::new(200, 14, *canvas_width, *canvas_height);
    let _ = renderer.fill_rect(square_definition0);
    let _ = renderer.fill_rect(square_definition1);
    let _ = renderer.fill_rect(square_definition2);
    let _ = renderer.fill_rect(square_definition3);
    let _ = renderer.fill_rect(square_definition4);
    let _ = renderer.fill_rect(square_definition5);
    let _ = renderer.fill_rect(square_definition6);
    let _ = renderer.fill_rect(square_definition7);
    let _ = renderer.fill_rect(square_definition8);
    let _ = renderer.fill_rect(square_definition9);
    let _ = renderer.fill_rect(square_definition10);
    let _ = renderer.fill_rect(square_definition11);
    let _ = renderer.fill_rect(square_definition12);
    let _ = renderer.fill_rect(square_definition13);
    let _ = renderer.fill_rect(square_definition14);

    renderer.present();
}
 
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
 
    let window = video_subsystem.window("Screen", 512, 256)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
 
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
 
    let mut event_pump = sdl_context.event_pump()?;
    // let mut i = 0;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // Update
        // i = (i + 1) % 255;

        // Render
        render(&mut canvas, Color::RGB(255, 255, 255));
        display_rectangle(&mut canvas, &100, &10);

        // Try
        // let mut points = Vec::new();
        // for i in 0..30 {
        //     points.push(Point::new(i, i));
        // }
        // canvas.set_draw_color(Color::RGB(0, 0, 255));
        // canvas.fill_rect(Rect::new(10, 10, 1, 1));
        // canvas.draw_points(points.as_slice());
        // canvas.clear();
        // canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
