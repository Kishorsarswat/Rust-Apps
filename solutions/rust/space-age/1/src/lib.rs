// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const ORBITAL_PERIOD_MERCURY: f64 = 0.2408467;
const ORBITAL_PERIOD_VENUS: f64 = 0.61519726;
const ORBITAL_PERIOD_EARTH: f64 = 1.0;
const ORBITAL_PERIOD_MARS: f64 = 1.8808158;
const ORBITAL_PERIOD_JUPITER: f64 = 11.862615;
const ORBITAL_PERIOD_SATURN: f64 = 29.447498;
const ORBITAL_PERIOD_URANUS: f64 = 84.016846;
const ORBITAL_PERIOD_NEPTUNE: f64 = 164.79132;

const EARTH_YEAR_IN_SECONDS: f64 = 31_557_600.0; 

#[derive(Debug)]
pub struct Duration
{
    age: u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self{
            age: s
        }
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
        let year = EARTH_YEAR_IN_SECONDS * ORBITAL_PERIOD_MERCURY;
        d.age as f64 / year
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        let year = EARTH_YEAR_IN_SECONDS * ORBITAL_PERIOD_VENUS;
        d.age as f64 / year
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        let year = EARTH_YEAR_IN_SECONDS * ORBITAL_PERIOD_EARTH;
        d.age as f64 / year
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        let year = EARTH_YEAR_IN_SECONDS * ORBITAL_PERIOD_MARS;
        d.age as f64 / year
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        let year = EARTH_YEAR_IN_SECONDS * ORBITAL_PERIOD_JUPITER;
        d.age as f64 / year
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        let year = EARTH_YEAR_IN_SECONDS * ORBITAL_PERIOD_SATURN;
        d.age as f64 / year
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        let year = EARTH_YEAR_IN_SECONDS * ORBITAL_PERIOD_URANUS;
        d.age as f64 / year
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        let year = EARTH_YEAR_IN_SECONDS * ORBITAL_PERIOD_NEPTUNE;
        d.age as f64 / year
    }
}
