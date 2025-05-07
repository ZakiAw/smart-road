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
    Air,
}

#[derive(Debug)]
pub struct Car {
    pub id: usize,
    pub position: (f64, f64),
    pub speed: f64,
    pub waypoints: Vec<(f64, f64)>,
    pub lane: Lane,
}


impl Car {
    pub fn new(lane: Lane, start: (f64, f64), waypoints: Vec<(f64, f64)>, speed: f64, id: usize) -> Self {
        Car {
            id,
            position: start,
            speed,
            waypoints,
            lane,
        }
    }


    pub fn update_position(&mut self) {
        if let Some(&(target_x, target_y)) = self.waypoints.first() {
            let dx = target_x - self.position.0;
            let dy = target_y - self.position.1;
            let distance = (dx * dx + dy * dy).sqrt();
    
            if distance < self.speed {
                self.position = (target_x, target_y);
                self.waypoints.remove(0); // Go to next waypoint
            } else {
                self.position.0 += self.speed * dx / distance;
                self.position.1 += self.speed * dy / distance;
            }
        }
    }
    

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        let car_color = match self.lane {
            Lane::Straight => Color::RGB(0, 255, 0),
            Lane::Right => Color::RGB(0, 0, 255),
            Lane::Left => Color::RGB(255, 0, 0),
            Lane::Air => Color::RGB(255, 255, 255),
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
