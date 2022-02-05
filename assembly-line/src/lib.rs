const CARS_PER_HOUR: f64 = 221.0;
const MINUTES_PER_HOUR: u32 = 60;

fn success_rate_by_speed(speed: u8) -> f64 {
    if speed <= 4 {
        1.0
    } else if speed <= 8 {
        0.9
    } else {
        0.77
    }
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    success_rate_by_speed(speed) * CARS_PER_HOUR * (speed as f64)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) as u32) / MINUTES_PER_HOUR
}
