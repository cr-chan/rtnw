use std::ops::Add;

#[derive(Default, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn new_from_interval(a: Interval, b: Interval) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub fn cotains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn expand(&self, x: f64) -> Self {
        let padding = x * 0.5;

        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    pub const EMPTY: Self = Self {
        min: std::f64::INFINITY,
        max: std::f64::NEG_INFINITY,
    };

    pub const UNIVERSE: Self = Self {
        min: std::f64::NEG_INFINITY,
        max: std::f64::INFINITY,
    };
}

impl Add<f64> for Interval {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Interval {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}

impl Add<Interval> for f64 {
    type Output = Interval;

    fn add(self, rhs: Interval) -> Self::Output {
        Interval {
            min: self + rhs.min,
            max: self + rhs.max,
        }
    }
}
