// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const DAY_IN_SECONDS: f64 = 86400.0;
const DAYS_EARTH_SIDERAL_YEAR: f64 = 365.25;
const EARTH_YEAR_SECONDS: f64 = DAY_IN_SECONDS * DAYS_EARTH_SIDERAL_YEAR;

#[derive(Debug)]
pub struct Duration {
    seconds: f64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

pub trait Planet {
    const LOCAL_YEAR: f64;
    fn years_during(d: &Duration) -> f64 {
        let age_as_earth_years = d.seconds / EARTH_YEAR_SECONDS;
        age_as_earth_years / Self::LOCAL_YEAR
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
    const LOCAL_YEAR: f64 = 0.2408467;
}
impl Planet for Venus {
    const LOCAL_YEAR: f64 = 0.61519726;
}
impl Planet for Earth {
    const LOCAL_YEAR: f64 = 1.0;
}
impl Planet for Mars {
    const LOCAL_YEAR: f64 = 1.8808158;
}
impl Planet for Jupiter {
    const LOCAL_YEAR: f64 = 11.862615;
}
impl Planet for Saturn {
    const LOCAL_YEAR: f64 = 29.447498;
}
impl Planet for Uranus {
    const LOCAL_YEAR: f64 = 84.016846;
}
impl Planet for Neptune {
    const LOCAL_YEAR: f64 = 164.79132;
}
