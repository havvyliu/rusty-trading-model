
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Point {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u32,
}

impl Point {
    pub fn new(open: f64, high: f64, low: f64, close: f64, volume: u32) -> Self {
        Self {
            open,
            high,
            low,
            close,
            volume,
        }
    }

    pub fn blank() -> Self {
        Self {
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: 0.0,
            volume: 0,
        }
    }

    pub fn borrow(&self) -> &Self {
        self
    }
}
