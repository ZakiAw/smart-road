use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;
mod car;
use car::Car;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Smart Intersection", 1600, 1200)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Constants
    const LANE_WIDTH: u32 = 60;
    const ROAD_WIDTH: u32 = LANE_WIDTH * 6; // 3 in + 3 out
    const SCREEN_WIDTH: u32 = 1600;
    const SCREEN_HEIGHT: u32 = 1200;

    // Intersection boundaries (center box to exclude dashed lines)
    let center_top: u32 = 420;
    let center_bottom: u32 = 750;
    let center_left: u32 = 600;
    let center_right: u32 = 960;

    'running: loop {
        // Clear the screen with black
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw vertical road
        canvas.set_draw_color(Color::RGB(40, 40, 40));
        canvas
            .fill_rect(Rect::new(
                ((SCREEN_WIDTH - ROAD_WIDTH) / 2) as i32,
                0,
                ROAD_WIDTH,
                SCREEN_HEIGHT,
            ))
            .unwrap();

        // Draw horizontal road
        canvas
            .fill_rect(Rect::new(
                0,
                ((SCREEN_HEIGHT - ROAD_WIDTH) / 2) as i32,
                SCREEN_WIDTH,
                ROAD_WIDTH,
            ))
            .unwrap();

        // Lane dividers (dashed white lines)
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let dash_step = 60;

        // Vertical dashed lines
        for y in (0..SCREEN_HEIGHT).step_by(dash_step as usize) {
            for i in 1..=5 {
                if i == 3 {
                    continue; // skip center lane
                }

                let x = (SCREEN_WIDTH - ROAD_WIDTH) / 2 + i * LANE_WIDTH;
                if y < center_top || y > center_bottom {
                    canvas.fill_rect(Rect::new(x as i32, y as i32, 2, 30)).unwrap();
                }
            }
        }

        // Horizontal dashed lines
        for x in (0..SCREEN_WIDTH).step_by(dash_step as usize) {
            for i in 1..=5 {
                if i == 3 {
                    continue;
                }

                let y = (SCREEN_HEIGHT - ROAD_WIDTH) / 2 + i * LANE_WIDTH;
                if x < center_left || x > center_right {
                    canvas.fill_rect(Rect::new(x as i32, y as i32, 30, 2)).unwrap();
                }
            }
        }

        // Solid center dividers
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        // Vertical center divider
        let vertical_divider_x = (SCREEN_WIDTH / 2) as i32;
        canvas
            .fill_rect(Rect::new(vertical_divider_x - 1, 0, 2, SCREEN_HEIGHT))
            .unwrap();

        // Horizontal center divider
        let horizontal_divider_y = (SCREEN_HEIGHT / 2) as i32;
        canvas
            .fill_rect(Rect::new(0, horizontal_divider_y - 1, SCREEN_WIDTH, 2))
            .unwrap();

        // Show everything
        canvas.present();

        // Handle events
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

        // Limit to ~60 FPS
        std::thread::sleep(Duration::from_millis(16));
    }
}
