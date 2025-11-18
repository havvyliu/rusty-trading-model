use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub enum TimeRange {
    Second,
    FiveMinute,
    Minute,
    Hour,
    Day,
    Month,
}
