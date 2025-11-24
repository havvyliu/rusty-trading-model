use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::structs::Point;
use crate::structs::TimeRange;

#[derive(Deserialize, Serialize, Clone)]
pub struct TimeSeries {
    time_range_unit: TimeRange,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    data: Vec<Point>,
}

impl TimeSeries {
    pub fn new(time_range_unit: TimeRange, start: DateTime<Utc>, end: DateTime<Utc>, data: Vec<Point>) -> Self {
        Self {time_range_unit, start, end, data}
    }

    pub fn default() -> Self {
        TimeSeries::new(TimeRange::Day,
            Utc::now(),
            Utc::now(),
            vec![])
    }

    pub fn time_range_unit(self: &Self) -> &TimeRange {
        &self.time_range_unit
    }

    pub fn update_time_range_unit(self: &mut Self, unit: TimeRange) {
        self.time_range_unit = unit;
    }

    pub fn start(self: &Self) -> &DateTime<Utc> {
        &self.start
    }

    pub fn end(self: &Self) -> &DateTime<Utc> {
        &self.end
    }

    pub fn set_end(self: &mut Self, end: DateTime<Utc>) {
        self.end = end;
    }

    pub fn data(self: &mut Self) -> &mut Vec<Point> {
        &mut self.data
    }
}
