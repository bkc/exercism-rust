use std::fmt;
use std::str::FromStr;

const MINUTES_PER_DAY: i32 = 1_440;
const MINUTES_PER_HOUR: i32 = 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let non_normalized_time = (hours * MINUTES_PER_HOUR + minutes) % MINUTES_PER_DAY;
        Clock {
            minutes: if non_normalized_time < 0 {
                non_normalized_time + MINUTES_PER_DAY
            } else {
                non_normalized_time
            },
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let non_normalized_time = (self.minutes + minutes) % MINUTES_PER_DAY;
        Clock {
            minutes: if non_normalized_time < 0 {
                non_normalized_time + MINUTES_PER_DAY
            } else {
                non_normalized_time
            },
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minutes / MINUTES_PER_HOUR,
            self.minutes % MINUTES_PER_HOUR
        )
    }
}

#[derive(Debug, Clone)]
pub struct FormatError;

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "string format is not valid")
    }
}

impl FromStr for Clock {
    type Err = FormatError;

    fn from_str(item: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = item.split(':').collect();
        match parts.len() {
            2 => {
                let hour = parts[0].parse::<i32>().map_err(|_| FormatError)?;
                let minutes = parts[1].parse::<i32>().map_err(|_| FormatError)?;
                Ok(Clock::new(hour, minutes))
            }
            _ => Err(FormatError),
        }
    }
}

#[test]
fn test_fromstr() {
    assert_eq!(Clock::new(10, 37), Clock::from_str("10:37").unwrap());
}

#[test]
fn test_from_badstr() {
    assert!(Clock::from_str("10:37:5").is_err())
}
