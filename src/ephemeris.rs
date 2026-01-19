//! Ephemeris data structures and position calculations

use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use crate::config::AppConfig;
use crate::time::JulianDate;
use crate::{Error, Result};

/// 3D position vector
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    /// X coordinate (AU)
    pub x: f64,
    /// Y coordinate (AU)
    pub y: f64,
    /// Z coordinate (AU)
    pub z: f64,
}

impl Position {
    /// Create a new position
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Calculate distance from origin
    pub fn distance(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

/// Represents a celestial body in the ephemeris
#[derive(Debug, Clone)]
pub struct SpaceObject {
    /// Whether this object is active
    pub active: bool,
    /// Header data for this object
    header_data: Vec<i32>,
    /// Name of the object
    pub name: String,
    /// Length of coefficients for this object
    pub coefficient_length: i32,
}

impl SpaceObject {
    fn new(name: String, active: bool) -> Self {
        Self {
            active,
            header_data: Vec::new(),
            name,
            coefficient_length: 0,
        }
    }
}

/// Main ephemeris structure
pub struct Ephemeris {
    config: AppConfig,
    bodies: Vec<SpaceObject>,
    start_year: i32,
    end_year: i32,
    ncoeff: i32,
    emrat: f64,
    interval: i32,
    julian_start: f64,
    julian_end: f64,
}

impl Ephemeris {
    /// Create a new ephemeris instance
    ///
    /// # Arguments
    /// * `config_path` - Path to the config.toml file
    ///
    /// # Example
    /// ```ignore
    /// use rust_jpl::Ephemeris;
    /// let mut eph = Ephemeris::new("config.toml")?;
    /// # Ok::<(), rust_jpl::Error>(())
    /// ```
    pub fn new(config_path: &str) -> Result<Self> {
        let config = AppConfig::new(config_path)?;
        let mut eph = Self {
            config,
            bodies: Vec::new(),
            start_year: 0,
            end_year: 0,
            ncoeff: 0,
            emrat: 0.0,
            interval: 0,
            julian_start: 0.0,
            julian_end: 0.0,
        };

        eph.initialize()?;
        Ok(eph)
    }

    /// Initialize the ephemeris by reading configuration files
    fn initialize(&mut self) -> Result<()> {
        self.read_init_data()?;
        self.read_header()?;
        self.calculate_coefficient_lengths();
        Ok(())
    }

    /// Read initial data file
    fn read_init_data(&mut self) -> Result<()> {
        let path = &self.config.initial_data_dat;
        let mut indat = File::open(path)?;
        let mut buffer = String::new();
        indat.read_to_string(&mut buffer)?;
        let lines: Vec<&str> = buffer.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];
            i += 1;

            if line == "BODIES:" {
                while i < lines.len() {
                    let body_line = lines[i];
                    i += 1;
                    if body_line == "DATE:" {
                        break;
                    }
                    let mut parts = body_line.split_whitespace();
                    if let Some(name) = parts.next() {
                        let mut so = SpaceObject::new(name.to_string(), false);
                        if let Some(state_str) = parts.next() {
                            so.active = bool::from_str(state_str).unwrap_or(false);
                        }
                        self.bodies.push(so);
                    }
                }
            }

            if line == "DATE:" {
                if i < lines.len() {
                    let start_year_str = lines[i];
                    i += 1;
                    if start_year_str.starts_with("Start_year") {
                        self.start_year = start_year_str
                            .split_whitespace()
                            .last()
                            .and_then(|s| i32::from_str(s).ok())
                            .unwrap_or(0);
                    }
                }

                if i < lines.len() {
                    let end_year_str = lines[i];
                    i += 1;
                    if end_year_str.starts_with("End_year") {
                        self.end_year = end_year_str
                            .split_whitespace()
                            .last()
                            .and_then(|s| i32::from_str(s).ok())
                            .unwrap_or(0);
                    }
                }
            }
        }

        Ok(())
    }

    /// Read header file
    fn read_header(&mut self) -> Result<()> {
        let path = &self.config.header_441;
        let mut header = File::open(path)?;
        let mut buffer = String::new();
        header.read_to_string(&mut buffer)?;
        let lines: Vec<&str> = buffer.lines().collect();
        let mut number_emrat = 0;

        for line in lines {
            let mut parts = line.split_whitespace();
            if let Some(key) = parts.next() {
                match key {
                    "NCOEFF=" => {
                        self.ncoeff = parts
                            .next()
                            .and_then(|s| i32::from_str(s).ok())
                            .unwrap_or(0);
                    }
                    "GROUP" => {
                        if let Some(group_number_str) = parts.next() {
                            match group_number_str {
                                "1030" => {
                                    if let Some(start_str) = parts.next() {
                                        self.julian_start = f64::from_str(start_str).unwrap_or(0.0);
                                    }
                                    if let Some(end_str) = parts.next() {
                                        self.julian_end = f64::from_str(end_str).unwrap_or(0.0);
                                    }
                                    if let Some(interval_str) = parts.next() {
                                        self.interval = i32::from_str(interval_str).unwrap_or(0);
                                    }
                                }
                                "1040" => {
                                    if let Some(size_str) = parts.next() {
                                        let size = i32::from_str(size_str).unwrap_or(0);
                                        for i in 0..size {
                                            if let Some(tmp) = parts.next() {
                                                if tmp == "EMRAT" {
                                                    number_emrat = i;
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                                "1041" => {
                                    for _ in 0..number_emrat {
                                        parts.next();
                                    }
                                    if let Some(emrat_str) = parts.next() {
                                        let mut emrat = emrat_str.to_string();
                                        let len = emrat.len();
                                        emrat.truncate(len - 4);
                                        emrat.push('E');
                                        self.emrat = f64::from_str(&emrat).unwrap_or(0.0);
                                    }
                                }
                                "1050" => {
                                    for _ in 0..3 {
                                        for body in &mut self.bodies {
                                            if let Some(buf_str) = parts.next() {
                                                let buf = i32::from_str(buf_str).unwrap_or(0);
                                                body.header_data.push(buf);
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    /// Calculate coefficient lengths for each body
    fn calculate_coefficient_lengths(&mut self) {
        let len = self.bodies.len();
        for i in 0..len - 1 {
            if !self.bodies[i].header_data.is_empty() && !self.bodies[i + 1].header_data.is_empty()
            {
                self.bodies[i].coefficient_length =
                    self.bodies[i + 1].header_data[0] - self.bodies[i].header_data[0];
            }
        }
        if !self.bodies.is_empty() && !self.bodies[len - 1].header_data.is_empty() {
            self.bodies[len - 1].coefficient_length =
                self.ncoeff - self.bodies[len - 1].header_data[0];
        }
    }

    /// Get the position of a celestial body at a given Julian date
    ///
    /// # Arguments
    /// * `body_name` - Name of the celestial body (e.g., "Earth", "Moon", "Sun", "Mars")
    /// * `jd` - Julian date
    ///
    /// # Returns
    /// Position in AU (Astronomical Units)
    ///
    /// # Example
    /// ```ignore
    /// use rust_jpl::{Ephemeris, JulianDate};
    /// let mut eph = Ephemeris::new("config.toml")?;
    /// let jd = JulianDate::from_calendar(2024, 1, 15, 12, 0, 0.0)?;
    /// let position = eph.get_position("Earth", jd)?;
    /// # Ok::<(), rust_jpl::Error>(())
    /// ```
    pub fn get_position(&self, body_name: &str, jd: JulianDate) -> Result<Position> {
        // Validate Julian date is within range
        if jd.jd < self.julian_start || jd.jd > self.julian_end {
            return Err(Error::Ephemeris(format!(
                "Julian date {} is outside valid range [{}, {}]",
                jd.jd, self.julian_start, self.julian_end
            )));
        }

        // Find the body
        let body = self
            .bodies
            .iter()
            .find(|b| {
                b.name.eq_ignore_ascii_case(body_name)
                    || b.name.replace("_", "").eq_ignore_ascii_case(body_name)
            })
            .ok_or_else(|| {
                Error::Ephemeris(format!(
                    "Body '{}' not found. Available bodies: {}",
                    body_name,
                    self.bodies
                        .iter()
                        .map(|b| b.name.clone())
                        .collect::<Vec<_>>()
                        .join(", ")
                ))
            })?;

        if !body.active {
            return Err(Error::Ephemeris(format!(
                "Body '{}' is not active in this ephemeris",
                body_name
            )));
        }

        // For now, return a placeholder position
        // In a full implementation, this would read the binary ephemeris file
        // and interpolate the Chebyshev coefficients
        Ok(Position::new(0.0, 0.0, 0.0))
    }

    /// Get all available celestial bodies
    pub fn get_bodies(&self) -> Vec<&SpaceObject> {
        self.bodies.iter().collect()
    }

    /// Get the valid date range for this ephemeris
    pub fn get_date_range(&self) -> (f64, f64) {
        (self.julian_start, self.julian_end)
    }

    /// Get ephemeris metadata
    pub fn get_metadata(&self) -> EphemerisMetadata {
        EphemerisMetadata {
            start_year: self.start_year,
            end_year: self.end_year,
            julian_start: self.julian_start,
            julian_end: self.julian_end,
            interval_days: self.interval as f64,
            earth_moon_ratio: self.emrat,
            number_of_coefficients: self.ncoeff,
        }
    }
}

/// Metadata about the ephemeris
#[derive(Debug, Clone)]
pub struct EphemerisMetadata {
    pub start_year: i32,
    pub end_year: i32,
    pub julian_start: f64,
    pub julian_end: f64,
    pub interval_days: f64,
    pub earth_moon_ratio: f64,
    pub number_of_coefficients: i32,
}
