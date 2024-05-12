use std::fmt::Debug;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use crate::model::messages::clutter_filter_map::elevation_segment::ElevationSegment;
use crate::model::messages::primitive_aliases::Integer2;
use crate::model::util::get_datetime;

/// Header information for a clutter filter map to be read directly from the Archive II file.
#[derive(Serialize, Deserialize)]
pub struct Header {
    /// The date the clutter filter map was generated represented as a count of days since 1 January
    /// 1970 00:00 GMT. It is also referred-to as a "modified Julian date" where it is the Julian
    /// date - 2440586.5.
    pub map_generation_date: Integer2,

    /// The time the clutter filter map was generated in milliseconds past midnight, GMT.
    pub map_generation_time: Integer2,

    /// The number of elevation segments defined in this clutter filter map. There may be 1 to 5,
    /// though there are typically 2. They will follow this header in order of increasing elevation.
    pub elevation_segment_count: Integer2,
}

impl Header {
    /// The date and time the clutter filter map was generated.
    pub fn date_time(&self) -> DateTime<Utc> {
        get_datetime(self.map_generation_date, Duration::milliseconds(self.map_generation_time as i64))
    }
}

impl Debug for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Header")
            .field("map_generation_date_time", &self.date_time())
            .field("elevation_segment_count", &self.elevation_segment_count)
            .finish()
    }
}

/// A clutter filter map describing elevations, azimuths, and ranges containing clutter to
/// filtered from radar products.
#[derive(Debug)]
pub struct Message {
    /// Decoded header information for this clutter filter map.
    pub header: Header,

    /// The elevation segments defined in this clutter filter map.
    pub elevation_segments: Vec<ElevationSegment>,
}

impl Message {
    /// Creates a new clutter filter map from the coded header.
    pub(crate) fn new(header: Header) -> Self {
        Self {
            elevation_segments: Vec::with_capacity(header.elevation_segment_count as usize),
            header,
        }
    }
}
