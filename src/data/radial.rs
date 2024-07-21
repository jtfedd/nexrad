#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

#[cfg(feature = "uom")]
use uom::si::{angle::degree, f32::Angle};

/// A single radar ray composed of a series of gates. This represents a single azimuth angle and
/// elevation angle pair at a point in time and contains the Level II data (reflectivity, velocity,
/// and spectrum width) for each range gate in that ray. The range of the radar and gate interval
/// distance determines the resolution of the ray and the number of gates in the ray.
pub struct Radial {
    collection_timestamp: i64,

    azimuth_number: u16,
    azimuth_angle_degrees: f32,
    azimuth_spacing_degrees: f32,

    radial_status: RadialStatus,

    elevation_angle_degrees: f32,
}

impl Radial {
    /// The collection timestamp in milliseconds since midnight Jan 1, 1970 (epoch/UNIX timestamp).
    pub fn collection_timestamp(&self) -> i64 {
        self.collection_timestamp
    }

    /// The collection time for this radial and its data.
    #[cfg(feature = "chrono")]
    pub fn collection_time(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp_millis(self.collection_timestamp)
    }

    /// The index number for this radial's azimuth in the elevation sweep, ranging up to 720
    /// depending on the azimuthal resolution.
    pub fn azimuth_number(&self) -> u16 {
        self.azimuth_number
    }

    /// Azimuth angle this radial's data was collected at in degrees.
    pub fn azimuth_angle_degrees(&self) -> f32 {
        self.azimuth_angle_degrees
    }

    /// Azimuth angle this radial's data was collected at.
    #[cfg(feature = "uom")]
    pub fn azimuth(&self) -> Angle {
        Angle::new::<degree>(self.azimuth_angle_degrees)
    }

    /// Azimuthal distance between radials in the sweep in degrees.
    pub fn azimuth_spacing_degrees(&self) -> f32 {
        self.azimuth_spacing_degrees
    }

    /// Azimuthal distance between radials in the sweep.
    #[cfg(feature = "uom")]
    pub fn azimuth_spacing(&self) -> Angle {
        Angle::new::<degree>(self.azimuth_spacing_degrees)
    }

    /// The radial's position in the sequence of radials making up a scan.
    pub fn radial_status(&self) -> RadialStatus {
        self.radial_status
    }

    /// Elevation angle this radial's data was collected at in degrees.
    pub fn elevation_angle_degrees(&self) -> f32 {
        self.elevation_angle_degrees
    }

    /// Elevation angle this radial's data was collected at.
    pub fn elevation_angle(&self) -> Angle {
        Angle::new::<degree>(self.elevation_angle_degrees)
    }
}

/// Describe a radial's position within the sequence of radials comprising a scan.
#[derive(Clone, Copy)]
pub enum RadialStatus {
    ElevationStart,
    IntermediateRadialData,
    ElevationEnd,
    VolumeScanStart,
    VolumeScanEnd,
    /// Start of new elevation which is the last in the VCP.
    ElevationStartVCPFinal,
}
