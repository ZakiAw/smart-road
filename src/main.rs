use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use std::time::{Duration, Instant};

mod car;
use car::{Car, Direction, Lane, Waypoint};
use rand::Rng;
mod spawn_cars;
use spawn_cars::spawn_car_from_key;

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

    let _image_context = sdl2::image::init(InitFlag::PNG).unwrap();
    let texture_creator = canvas.texture_creator();

    let car_textures: Vec<Texture> = vec![
        texture_creator.load_texture("assets/Car.png").unwrap(),
        texture_creator
            .load_texture("assets/Black_viper.png")
            .unwrap(),
        texture_creator.load_texture("assets/Police.png").unwrap(),
    ];

    let plane_textures: Vec<Texture> = vec![
        texture_creator.load_texture("assets/Blemheim.png").unwrap(),
        texture_creator.load_texture("assets/Hawker.png").unwrap(),
    ];

    let background_textures: Vec<Texture> = vec![
        texture_creator.load_texture("assets/left1.png").unwrap(),
        texture_creator.load_texture("assets/left2.png").unwrap(),
        texture_creator.load_texture("assets/right1.png").unwrap(),
        texture_creator.load_texture("assets/right2.png").unwrap(),
    ];

    let mut last_spawn_time = Instant::now();
    let cooldown = Duration::from_secs_f64(0.25);

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw roads
        canvas.set_draw_color(Color::RGB(23, 23, 23));
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

        // Dashed lane dividers - - - -
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

        // Solid center dividers +
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas
            .fill_rect(Rect::new(
                ((SCREEN_WIDTH / 2) as i32) - 1,
                0,
                2,
                SCREEN_HEIGHT,
            ))
            .unwrap();
        canvas
            .fill_rect(Rect::new(
                0,
                ((SCREEN_HEIGHT / 2) as i32) - 1,
                SCREEN_WIDTH,
                2,
            ))
            .unwrap();

        // Clear center box
        let center_x = SCREEN_WIDTH / 2;
        let center_y = SCREEN_HEIGHT / 2;

        let center_box_width = ROAD_WIDTH / 2 + 183;
        let center_box_height = ROAD_WIDTH / 2 + 183;

        let center_rect = Rect::new(
            (center_x - center_box_width / 2) as i32,
            (center_y - center_box_height / 2) as i32,
            center_box_width,
            center_box_height,
        );

        canvas.set_draw_color(Color::RGB(23, 23, 23));
        canvas.fill_rect(center_rect).unwrap();

        // mfar2 5tooot
        canvas.set_draw_color(Color::YELLOW);
        canvas.fill_rect(Rect::new(982, 420, 5, 179)).unwrap();

        canvas.set_draw_color(Color::YELLOW);
        canvas.fill_rect(Rect::new(613, 602, 5, 177)).unwrap();

        canvas.set_draw_color(Color::YELLOW);
        canvas.fill_rect(Rect::new(620, 414, 177, 5)).unwrap();

        canvas.set_draw_color(Color::YELLOW);
        canvas.fill_rect(Rect::new(802, 782, 177, 5)).unwrap();

        // canvas.fill_rect(Rect::new(820, 690, 5, 5)).unwrap();
        // canvas.fill_rect(Rect::new(890, 690, 5, 5)).unwrap();

        // canvas.fill_rect(Rect::new(710, 690, 5, 5)).unwrap();
        // canvas.fill_rect(Rect::new(770, 690, 5, 5)).unwrap();

        // canvas.fill_rect(Rect::new(773, 570, 5, 5)).unwrap();
        // canvas.fill_rect(Rect::new(830, 630, 5, 5)).unwrap();

        // canvas.fill_rect(Rect::new(773, 570, 5, 5)).unwrap();
        // canvas.fill_rect(Rect::new(830, 630, 5, 5)).unwrap();

        let image_positions = vec![(0, 0), (0, 780), (980, 0), (980, 780)];
        let (img_w, img_h) = (620, 420);
        for (i, texture) in background_textures.iter().enumerate() {
            let (x, y) = image_positions[i];
            canvas
                .copy(texture, None, Some(Rect::new(x, y, img_w, img_h)))
                .unwrap();
        }
        let a = cars.clone();
        for car in cars.iter_mut() {
            car.update_position(&a);
        }
        for car in cars.iter() {
            car.render(&mut canvas);
        }
        cars.retain(|car| !car.has_finished());

        canvas.present();

        let direction_keys = [Keycode::Left, Keycode::Right, Keycode::Up, Keycode::Down];
        for event in event_pump.poll_iter() {
            if let Event::KeyDown {
                keycode: Some(key), ..
            } = event
            {
                match key {
                    Keycode::Escape => {
                        break 'running;
                    }
                    Keycode::R => {
                        let random_key =
                            direction_keys[rand::thread_rng().gen_range(0..direction_keys.len())];
                        if last_spawn_time.elapsed() >= cooldown {
                            if let Some(car) =
                                spawn_car_from_key(random_key, &car_textures, car_id_counter)
                            {
                                cars.push(car);
                                car_id_counter += 1;
                                last_spawn_time = Instant::now();
                            }
                        }
                    }
                    Keycode::Up | Keycode::Down | Keycode::Left | Keycode::Right => {
                        if last_spawn_time.elapsed() >= cooldown {
                            if let Some(car) =
                                spawn_car_from_key(key, &car_textures, car_id_counter)
                            {
                                cars.push(car);
                                car_id_counter += 1;
                                last_spawn_time = Instant::now();
                            }
                        }
                    }
                    Keycode::P => {
                        let texture =
                            &plane_textures[rand::thread_rng().gen_range(0..plane_textures.len())];

                        let lane = Lane::Air;
                        let direction = Direction::East;
                        let position = (1620.0, 1000.0);

                        let waypoints = vec![Waypoint {
                            x: -20.0,
                            y: 170.0,
                            angle: None,
                        }];

                        cars.push(Car::new(
                            lane,
                            position,
                            waypoints,
                            4.0,
                            car_id_counter,
                            direction,
                            texture,
                            Some((120, 80)),
                        ));
                        car_id_counter += 1;
                    }
                    _ => {}
                }
            } else if let Event::Quit { .. } = event {
                break 'running;
            }
        }

        std::thread::sleep(Duration::from_millis(16));
    }
}
