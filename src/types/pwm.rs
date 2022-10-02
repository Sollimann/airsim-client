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
        if front_right_pwm.is_sign_negative() || front_right_pwm > 1.0 {
            panic!("front_right_pwm outside of valid range 0.0 to 1.0")
        }

        if rear_left_pwm.is_sign_negative() || rear_left_pwm > 1.0 {
            panic!("rear_left_pwm outside of valid range 0.0 to 1.0")
        }

        if front_left_pwm.is_sign_negative() || front_left_pwm > 1.0 {
            panic!("front_left_pwm outside of valid range 0.0 to 1.0")
        }

        if rear_right_pwm.is_sign_negative() || rear_right_pwm > 1.0 {
            panic!("rear_right_pwm outside of valid range 0.0 to 1.0")
        }

        Self {
            front_right_pwm,
            rear_left_pwm,
            front_left_pwm,
            rear_right_pwm,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::PWM;

    #[test]
    #[should_panic]
    fn test_pwm_range() {
        PWM::new(-1.0, 0.1, 0.1, 0.1);
    }
}
