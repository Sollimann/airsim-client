#[derive(Clone, Debug)]
pub struct PWM {
    /// PWM value for the front right motor (between 0.0 to 1.0)
    pub front_right_pwm: f32,
    /// PWM value for the rear left motor (between 0.0 to 1.0)
    pub rear_left_pwm: f32,
    /// PWM value for the front left motor (between 0.0 to 1.0)
    pub front_left_pwm: f32,
    /// PWM value for the rear right motor (between 0.0 to 1.0)
    pub rear_right_pwm: f32,
}

impl PWM {
    pub fn new(front_right_pwm: f32, rear_left_pwm: f32, front_left_pwm: f32, rear_right_pwm: f32) -> Self {
        Self {
            front_right_pwm,
            rear_left_pwm,
            front_left_pwm,
            rear_right_pwm,
        }
    }
}
