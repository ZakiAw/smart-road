use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Smart Intersection", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        // Clear the screen with black
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw a white rectangle in the center
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let rect = Rect::new(300, 200, 200, 200); // x, y, width, height
        canvas.fill_rect(rect).unwrap();

        // Show the updated canvas
        canvas.present();

        // Event loop
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Cap frame rate to ~60 FPS
        std::thread::sleep(Duration::from_millis(16));
    }
}
