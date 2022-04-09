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
    fn years_during(d: &Duration) -> f64;
}

macro_rules! make_planets {
    ( $($name:ident,$orbital_period:literal);* ) => {
        $(
            pub struct $name;
            impl Planet for $name {
                fn years_during( d: &Duration) -> f64 {
                    (d.seconds / SECONDS_PER_EARTH_YEAR) / $orbital_period
                }
            }
        )*
    };
}

make_planets!(
    Mercury, 0.2408467;
    Venus, 0.61519726;
    Earth, 1.0;
    Mars, 1.8808158;
    Jupiter, 11.862615;
    Saturn, 29.447498;
    Uranus, 84.016846;
    Neptune, 164.79132
);
