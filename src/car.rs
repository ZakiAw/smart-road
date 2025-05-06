#[derive(Debug)]
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
    pub start_time: std::time::Instant,
    pub id: u32,
}

