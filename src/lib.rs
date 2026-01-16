//! # Rust JPL Ephemeris Reader
//!
//! A Rust library for parsing NASA JPL ephemeris data, providing precise planetary positions
//! based on Julian dates. This library supports DE441 and other JPL ephemeris formats.
//!
//! ## Features
//!
//! - Planetary Positions: Accurate positions of celestial bodies for a given Julian date
//! - Ephemeris Data: Utilization of DE441 file for precise planetary information
//! - Time Conversions: Conversion between Julian dates and calendar dates
//! - Lunar Ephemerides: Specific data for accurate moon-related calculations
//! - Solar System Dynamics: Detailed data for understanding solar system dynamics
//!
//! ## Example
//!
//! ```no_run
//! use rust_jpl::{Ephemeris, JulianDate};
//!
//! // Initialize ephemeris with configuration
//! let eph = Ephemeris::new("config.toml")?;
//!
//! // Convert calendar date to Julian date
//! let jd = JulianDate::from_calendar(2024, 1, 15, 12, 0, 0.0)?;
//!
//! // Get planetary position
//! let position = eph.get_position("Earth", jd)?;
//! println!("Earth position: ({}, {}, {}) AU", position.x, position.y, position.z);
//! # Ok::<(), rust_jpl::Error>(())
//! ```

pub mod config;
pub mod ephemeris;
pub mod time;

pub use ephemeris::{Ephemeris, EphemerisMetadata, Position, SpaceObject};
pub use time::{CalendarDate, JulianDate};

/// Error types for the library
#[derive(Debug)]
pub enum Error {
    /// Configuration error
    Config(String),
    /// I/O error
    Io(std::io::Error),
    /// Invalid date/time error
    InvalidDate(String),
    /// Ephemeris data error
    Ephemeris(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Config(msg) => write!(f, "Configuration error: {}", msg),
            Error::Io(err) => write!(f, "I/O error: {}", err),
            Error::InvalidDate(msg) => write!(f, "Invalid date: {}", msg),
            Error::Ephemeris(msg) => write!(f, "Ephemeris error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
