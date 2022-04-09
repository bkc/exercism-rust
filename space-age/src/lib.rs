#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

const SECONDS_PER_EARTH_YEAR: f64 = 31_557_600.0;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        unimplemented!(
            "convert a duration ({:?}) to the number of years on this planet for that duration",
            d,
        );
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        (d.seconds / SECONDS_PER_EARTH_YEAR) / 0.2408467
    }
}
impl Planet for Venus {}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / SECONDS_PER_EARTH_YEAR
    }
}
impl Planet for Mars {}
impl Planet for Jupiter {}
impl Planet for Saturn {}
impl Planet for Uranus {}
impl Planet for Neptune {}
