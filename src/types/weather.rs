use rmp_rpc::Value;

#[derive(Debug, Clone, Copy)]
pub enum WeatherParameter {
    Rain,
    Roadwetness,
    Snow,
    RoadSnow,
    MapleLeaf,
    RoadLeaf,
    Dust,
    Fog,
    Enabled,
}

impl WeatherParameter {
    pub(crate) fn _as_msgpack(&self) -> Value {
        let val = match self {
            WeatherParameter::Rain => 0_i64,
            WeatherParameter::Roadwetness => 1_i64,
            WeatherParameter::Snow => 2_i64,
            WeatherParameter::RoadSnow => 3_i64,
            WeatherParameter::MapleLeaf => 4_i64,
            WeatherParameter::RoadLeaf => 5_i64,
            WeatherParameter::Dust => 6_i64,
            WeatherParameter::Fog => 7_i64,
            WeatherParameter::Enabled => 8_i64,
        };

        Value::Integer(val.into())
    }
}
