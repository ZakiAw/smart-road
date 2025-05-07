use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub enum Lane {
    Straight,
    Right,
    Left,
}

#[derive(Debug)]
pub struct Car {
    pub lane: Lane,
    pub speed: f32,
    pub position: (f32, f32),
    pub destination: (f32, f32),
    pub start_time: Instant,
    pub id: u32,
}

impl Car {
    pub fn new(lane: Lane, position: (f32, f32), destination: (f32, f32), speed: f32, id: u32) -> Self {
        Car {
            lane,
            position,
            destination,
            speed,
            start_time: Instant::now(),
            id,
        }
    }

    pub fn update_position(&mut self) {
        let dx = self.destination.0 - self.position.0;
        let dy = self.destination.1 - self.position.1;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > 0.1 {
            self.position.0 += self.speed * dx / distance;
            self.position.1 += self.speed * dy / distance;
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        let car_color = match self.lane {
            Lane::Straight => Color::RGB(0, 255, 0),
            Lane::Right => Color::RGB(0, 0, 255),
            Lane::Left => Color::RGB(255, 0, 0),
        };

        canvas.set_draw_color(car_color);
        canvas
            .fill_rect(Rect::new(
                self.position.0 as i32 - 10,
                self.position.1 as i32 - 10,
                20,
                20,
            ))
            .unwrap();
    }
}
