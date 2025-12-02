// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_YEAR_IN_SECONDS: f64 = 31_557_600.0; 

const ORBITAL_PERIOD_MERCURY: f64 = 0.2408467 * EARTH_YEAR_IN_SECONDS;
const ORBITAL_PERIOD_VENUS: f64 = 0.61519726 * EARTH_YEAR_IN_SECONDS;
const ORBITAL_PERIOD_EARTH: f64 = 1.0 * EARTH_YEAR_IN_SECONDS;
const ORBITAL_PERIOD_MARS: f64 = 1.8808158 * EARTH_YEAR_IN_SECONDS;
const ORBITAL_PERIOD_JUPITER: f64 = 11.862615 * EARTH_YEAR_IN_SECONDS;
const ORBITAL_PERIOD_SATURN: f64 = 29.447498 * EARTH_YEAR_IN_SECONDS;
const ORBITAL_PERIOD_URANUS: f64 = 84.016846 * EARTH_YEAR_IN_SECONDS;
const ORBITAL_PERIOD_NEPTUNE: f64 = 164.79132 * EARTH_YEAR_IN_SECONDS;

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s as f64)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
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
        d.0 / ORBITAL_PERIOD_MERCURY
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.0 / ORBITAL_PERIOD_VENUS
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.0 / ORBITAL_PERIOD_EARTH
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.0 / ORBITAL_PERIOD_MARS
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.0 / ORBITAL_PERIOD_JUPITER
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.0 / ORBITAL_PERIOD_SATURN
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.0 / ORBITAL_PERIOD_URANUS
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.0 / ORBITAL_PERIOD_NEPTUNE
    }
}
