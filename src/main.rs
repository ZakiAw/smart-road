use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};

mod car;
use car::{Car, Lane};
use rand::Rng;

fn random_lane() -> Lane {
    let n = rand::thread_rng().gen_range(0..=2);
    match n {
        0 => Lane::Straight,
        1 => Lane::Right,
        _ => Lane::Left,
    }
}

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

    let mut cars: Vec<Car> = Vec::new();
    let mut car_id_counter = 0;

    const LANE_WIDTH: u32 = 60;
    const ROAD_WIDTH: u32 = LANE_WIDTH * 6;
    const SCREEN_WIDTH: u32 = 1600;
    const SCREEN_HEIGHT: u32 = 1200;

    let center_top: u32 = 420;
    let center_bottom: u32 = 750;
    let center_left: u32 = 600;
    let center_right: u32 = 960;

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw roads
        canvas.set_draw_color(Color::RGB(40, 40, 40));
        canvas
            .fill_rect(Rect::new(
                ((SCREEN_WIDTH - ROAD_WIDTH) / 2) as i32,
                0,
                ROAD_WIDTH,
                SCREEN_HEIGHT,
            ))
            .unwrap();
        canvas
            .fill_rect(Rect::new(
                0,
                ((SCREEN_HEIGHT - ROAD_WIDTH) / 2) as i32,
                SCREEN_WIDTH,
                ROAD_WIDTH,
            ))
            .unwrap();

        // Dashed lane dividers
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let dash_step = 60;
        for y in (0..SCREEN_HEIGHT).step_by(dash_step as usize) {
            for i in 1..=5 {
                if i == 3 {
                    continue;
                }
                let x = (SCREEN_WIDTH - ROAD_WIDTH) / 2 + i * LANE_WIDTH;
                if y < center_top || y > center_bottom {
                    canvas
                        .fill_rect(Rect::new(x as i32, y as i32, 2, 30))
                        .unwrap();
                }
            }
        }
        for x in (0..SCREEN_WIDTH).step_by(dash_step as usize) {
            for i in 1..=5 {
                if i == 3 {
                    continue;
                }
                let y = (SCREEN_HEIGHT - ROAD_WIDTH) / 2 + i * LANE_WIDTH;
                if x < center_left || x > center_right {
                    canvas
                        .fill_rect(Rect::new(x as i32, y as i32, 30, 2))
                        .unwrap();
                }
            }
        }

        // Solid center dividers
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let vertical_divider_x = (SCREEN_WIDTH / 2) as i32;
        canvas
            .fill_rect(Rect::new(vertical_divider_x - 1, 0, 2, SCREEN_HEIGHT))
            .unwrap();
        let horizontal_divider_y = (SCREEN_HEIGHT / 2) as i32;
        canvas
            .fill_rect(Rect::new(0, horizontal_divider_y - 1, SCREEN_WIDTH, 2))
            .unwrap();

        // Clear center box
        canvas.set_draw_color(Color::RGB(40, 40, 40));
        canvas
            .fill_rect(Rect::new(
                (center_left + 10) as i32,
                (center_top) as i32,
                center_right - center_left + 20,
                center_bottom - center_top + 25,
            ))
            .unwrap();

        // Draw cars
        for car in cars.iter_mut() {
            car.update_position();
            car.render(&mut canvas);
        }

        canvas.present();

        // Event handling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    let lane = random_lane();
                    let (position, waypoints) = match lane {
                        Lane::Straight => ((890.0, 1200.0), vec![(890.0, -20.0)]),
                        Lane::Left => (
                            (830.0, 1200.0),
                            vec![(830.0, 570.0), (-20.0, 570.0)], // turn UP
                        ),
                        Lane::Right => (
                            (950.0, 1200.0),
                            vec![(950.0, 750.0), (1620.0, 750.0)], // turn RIGHT
                        ),
                    };
                    cars.push(Car::new(lane, position, waypoints, 2.5, car_id_counter));
                    car_id_counter += 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    let lane = random_lane();
                    let (position, waypoints) = match lane {
                        Lane::Straight => ((710.0, 0.0), vec![(710.0, 1220.0)]),
                        Lane::Left => ((773.0, 0.0), vec![(773.0, 630.0), (1620.0, 630.0)]), // turn RIGHT
                        Lane::Right => ((650.0, 0.0), vec![(650.0, 450.0), (-20.0, 450.0)]), // turn LEFT
                    };
                    cars.push(Car::new(lane, position, waypoints, 2.5, car_id_counter));
                    car_id_counter += 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let lane = random_lane();
                    let (position, waypoints) = match lane {
                        Lane::Straight => ((1600.0, 510.0), vec![(-20.0, 510.0)]),
                        Lane::Left => ((1603.0, 570.0), vec![(773.0, 570.0), (773.0, 1220.0)]),  // turn UP
                        Lane::Right => ((1600.0, 450.0), vec![(950.0, 450.0), (950.0, -20.0)]), // turn DOWN
                    };
                    cars.push(Car::new(lane, position, waypoints, 2.5, car_id_counter));
                    
                    car_id_counter += 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let lane = random_lane();
                    let (position, waypoints) = match lane {
                        Lane::Straight => ((0.0, 690.0), vec![(1620.0, 690.0)]),
                        Lane::Left => ((0.0, 630.0), vec![(830.0, 630.0), (830.0, -20.0)]), // turn DOWN
                        Lane::Right => ((0.0, 750.0), vec![(650.0, 750.0), (650.0, 1220.0)]), // turn UP
                    };
                    cars.push(Car::new(lane, position, waypoints, 2.5, car_id_counter));
                    car_id_counter += 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    let lane = random_lane();
                    let (position, waypoints) = ((1620.0, 1000.0), vec![(-20.0, 170.0)]); 
                    cars.push(Car::new(lane, position, waypoints, 6.5, car_id_counter));
                    car_id_counter += 1;
                }


                _ => {}
            }
        }

        std::thread::sleep(Duration::from_millis(16));
    }
}
