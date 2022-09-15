#[derive(Clone, Debug)]
pub struct Position3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Position3 { x, y, z }
    }
}

#[derive(Clone, Debug)]
pub struct Orientation3 {
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl Orientation3 {
    pub fn new(roll: f32, pitch: f32, yaw: f32) -> Self {
        Orientation3 { roll, pitch, yaw }
    }
}

#[derive(Clone, Debug)]
pub struct Velocity3 {
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
}

impl Velocity3 {
    pub fn new(vx: f32, vy: f32, vz: f32) -> Self {
        Velocity3 { vx, vy, vz }
    }
}

#[derive(Clone, Debug)]
pub struct Velocity2 {
    pub vx: f32,
    pub vy: f32,
}

impl Velocity2 {
    pub fn new(vx: f32, vy: f32) -> Self {
        Velocity2 { vx, vy }
    }
}
