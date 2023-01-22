use msgpack_rpc::Value;

#[derive(Debug, Clone, Copy)]
/// Struct to store values of PID gains. Used to transmit controller gain values while instantiating
pub struct PIDGains {
    /// Proportional gain
    pub kp: f32,
    /// Integrator gain
    pub ki: f32,
    /// Derivative gain
    pub kd: f32,
}

impl PIDGains {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Self { kp, ki, kd }
    }

    pub(crate) fn _to_msgpack(&self) -> Value {
        let gains = vec![Value::F32(self.kp), Value::F32(self.ki), Value::F32(self.kd)];
        Value::Array(gains)
    }
}

#[derive(Debug, Clone, Copy)]
/// Struct to contain controller gains used by angle rate and level PID controller
pub struct AngularControllerGains {
    /// kp, ki, kd for roll axis
    pub roll_gains: PIDGains,
    /// kp, ki, kd for pitch axis
    pub pitch_gains: PIDGains,
    /// kp, ki, kd for yaw axis
    pub yaw_gains: PIDGains,
}

impl AngularControllerGains {
    pub fn new(roll_gains: PIDGains, pitch_gains: PIDGains, yaw_gains: PIDGains) -> Self {
        Self {
            roll_gains,
            pitch_gains,
            yaw_gains,
        }
    }

    pub(crate) fn as_msgpack(&self, vehicle_name: &'static str) -> Vec<Value> {
        let kps = Value::Array(vec![
            Value::F32(self.roll_gains.kp),
            Value::F32(self.pitch_gains.kp),
            Value::F32(self.yaw_gains.kp),
        ]);

        let kis = Value::Array(vec![
            Value::F32(self.yaw_gains.ki),
            Value::F32(self.roll_gains.ki),
            Value::F32(self.pitch_gains.ki),
        ]);

        let kds = Value::Array(vec![
            Value::F32(self.roll_gains.kd),
            Value::F32(self.pitch_gains.kd),
            Value::F32(self.yaw_gains.kd),
        ]);

        vec![kps, kis, kds, Value::String(vehicle_name.into())]
    }
}

#[derive(Debug, Clone, Copy)]
/// Struct to contain controller gains used by velocity and Position PID controller
pub struct LinearControllerGains {
    /// kp, ki, kd for X axis
    pub x_gains: PIDGains,
    /// kp, ki, kd for Y axis
    pub y_gains: PIDGains,
    /// kp, ki, kd for Z axis
    pub z_gains: PIDGains,
}

impl LinearControllerGains {
    pub fn new(x_gains: PIDGains, y_gains: PIDGains, z_gains: PIDGains) -> Self {
        Self {
            x_gains,
            y_gains,
            z_gains,
        }
    }

    pub(crate) fn as_msgpack(&self, vehicle_name: &'static str) -> Vec<Value> {
        let kps = Value::Array(vec![
            Value::F32(self.x_gains.kp),
            Value::F32(self.y_gains.kp),
            Value::F32(self.z_gains.kp),
        ]);

        let kis = Value::Array(vec![
            Value::F32(self.x_gains.ki),
            Value::F32(self.y_gains.ki),
            Value::F32(self.z_gains.ki),
        ]);

        let kds = Value::Array(vec![
            Value::F32(self.x_gains.kd),
            Value::F32(self.y_gains.kd),
            Value::F32(self.z_gains.kd),
        ]);

        vec![kps, kis, kds, Value::String(vehicle_name.into())]
    }
}
