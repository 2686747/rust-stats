use domain::bar::Bar;
use std::hash::Hash;
use std::hash::Hasher;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Position {
    pub bar: Bar,
    pub volume: f32,
}

impl Position {
    pub fn ticker(&self) -> &str {
        &self.bar.ticker
    }
    pub fn new(bar: Bar, volume: f32) -> Self {
        Position { bar, volume }
    }
    pub fn update(&self, bar: Bar) -> Self {
        Position{bar: bar, volume: self.volume}
    }
    pub fn amount(&self) -> f32 {
        self.bar.price() * self.volume
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.ticker() == other.ticker()
    }
}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ticker().hash(state);
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        self.amount().partial_cmp(&other.amount()).unwrap()
    }

    fn max(self, other: Self) -> Self where Self: Sized {
        if self.amount() > other.amount() { self } else { other }
    }

    fn min(self, other: Self) -> Self where Self: Sized {
        if self.amount() <= other.amount() { self } else { other }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        self.amount().partial_cmp(&other.amount())
    }
}

impl Eq for Position {}

