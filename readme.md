# Rust JPL Ephemeris Reader

The Rust JPL Ephemeris Reader is an advanced Rust library for parsing NASA JPL ephemeris data, providing precise planetary positions based on a given Julian date. Crafted in Rust for enhanced flexibility and efficiency, this tool harnesses the NASA JPL DE441 Library, offering a plethora of features crucial for diverse scientific and engineering applications.

## Features

| Feature                 | Description                                                     | Status  |
|-------------------------|-----------------------------------------------------------------|---------|
| Planetary Positions     | Accurate positions of celestial bodies for a given Julian date. | ✓       |
| Ephemeris Data          | Utilization of DE441 file for precise planetary information.    | ✓       |
| Scientific Applications | Essential for scientific and engineering calculations.          | ✓       |
| Flexibility             | Developed in Rust for flexibility and efficiency.               | ✓       |
| Solar System Dynamics   | Detailed data for understanding solar system dynamics.          | ✓       |
| Gravitational Effects   | Calculation of gravitational effects on celestial bodies.       | Planned |
| Time Conversions        | Conversion of time formats for diverse applications.            | ✓       |
| Multiple Ephemerides    | Support for various ephemerides for specialized needs.          | Planned |
| Celestial Phenomena     | Prediction and tracking of celestial events and phenomena.      | Planned |
| Lunar Ephemerides       | Specific data for accurate moon-related calculations.           | ✓       |

## Useful Implementations

| Implementation          | Description                                                    |
|-------------------------|----------------------------------------------------------------|
| Celestial Navigation    | Navigational aid for spacecraft based on accurate positions.   |
| Astronomy Applications  | Support for astronomical research and observation.             |
| Orbital Mechanics       | Critical for trajectory calculations and mission planning.     |
| Educational Purposes    | Ideal for teaching orbital dynamics and celestial mechanics.   |
| Simulation Software     | Integration into simulation tools for realistic scenarios.     |
| Satellite Tracking      | Tracking the positions of satellites for ground stations.      |
| Astrological Analysis   | Utilization in astrological software for celestial events.     |
| Space Mission Planning  | Assistance in planning trajectories for space missions.        |
| Deep Space Probes       | Positional data for deep space probes and exploration.         |

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-jpl = "0.0.1"
```

Or install from source:

```bash
git clone https://github.com/chinmayvivek/rust-jpl
cd rust-jpl
cargo build --release
```

## Quick Start

### Basic Usage

```rust
use rust_jpl::{Ephemeris, JulianDate};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize ephemeris with configuration
    let mut eph = Ephemeris::new("config.toml")?;
    
    // Convert calendar date to Julian date
    let jd = JulianDate::from_calendar(2024, 1, 15, 12, 0, 0.0)?;
    
    // Get planetary position
    let position = eph.get_position("Earth", jd)?;
    println!("Earth position: ({}, {}, {}) AU", 
             position.x, position.y, position.z);
    
    Ok(())
}
```

### Time Conversion

```rust
use rust_jpl::{CalendarDate, JulianDate};

// Convert calendar date to Julian date
let cal = CalendarDate::new(2024, 1, 15, 12, 0, 0.0);
let jd = cal.to_julian()?;
println!("Julian Date: {}", jd.as_f64());

// Convert Julian date back to calendar date
let cal2 = jd.to_calendar();
println!("Calendar: {}-{:02}-{:02}", cal2.year, cal2.month, cal2.day);
```

### Getting Planetary Positions

```rust
use rust_jpl::{Ephemeris, JulianDate};

let mut eph = Ephemeris::new("config.toml")?;
let jd = JulianDate::from_calendar(2024, 1, 15, 12, 0, 0.0)?;

// Get position of various celestial bodies
let sun_pos = eph.get_position("Sun", jd)?;
let earth_pos = eph.get_position("Earth", jd)?;
let mars_pos = eph.get_position("Mars", jd)?;

println!("Sun position: ({:.6}, {:.6}, {:.6}) AU", 
         sun_pos.x, sun_pos.y, sun_pos.z);
println!("Distance from origin: {:.6} AU", sun_pos.distance());
```

### Accessing Ephemeris Metadata

```rust
use rust_jpl::Ephemeris;

let mut eph = Ephemeris::new("config.toml")?;
let metadata = eph.get_metadata();

println!("Date Range: {} - {}", metadata.start_year, metadata.end_year);
println!("Julian Date Range: {} - {}", metadata.julian_start, metadata.julian_end);
println!("Interval: {} days", metadata.interval_days);
println!("Earth-Moon Mass Ratio: {}", metadata.earth_moon_ratio);

// List all available celestial bodies
for body in eph.get_bodies() {
    println!("{}: {}", body.name, if body.active { "active" } else { "inactive" });
}
```

## Configuration

Copy `config.toml.example` to `config.toml` and update the paths to your ephemeris files:

```bash
cp config.toml.example config.toml
```

Then edit `config.toml` with your file paths:

```toml
[paths]
nasa_jpl_de441 = "assets/linux_m13000p17000.441.bsp"
header_441 = "assets/header.441"
initial_data_dat = "assets/Initial_data.dat"
```

### Setting Up Ephemeris Files

1. **Download NASA JPL DE441:**
   - Obtain the NASA JPL DE441 Library files from [NASA JPL FTP](https://ssd.jpl.nasa.gov/ftp/eph/planets/Linux/de441/).
   - Download `linux_m13000p17000.441` and `header.441`
   - Rename `linux_m13000p17000.441` to `linux_m13000p17000.441.bsp`
   - Place both files in your `assets/` directory

2. **Create Initial Data File:**
   - Create `assets/Initial_data.dat` with the following format:
   ```
   BODIES:
   
   Mercury				true
   Venus					true
   EarthMoon_barycenter	true
   Mars					true
   Jupiter					true
   Saturn					true
   Uranus					true
   Neptune					true
   Pluto					true
   Moon_geocentric		true
   Sun						true
   
   DATE:
   
   Start_year			1940
   End_year			2100
   ```

3. **Update config.toml:**
   - Set the correct paths to your ephemeris files

## Examples

The repository includes several examples:

### Run Examples

```bash
# Basic usage example
cargo run --example basic_usage

# Time conversion example
cargo run --example time_conversion

# Planetary positions example
cargo run --example planetary_positions
```

### Example: Multiple Dates

```rust
use rust_jpl::{Ephemeris, JulianDate};

let mut eph = Ephemeris::new("config.toml")?;

let dates = vec![
    (2024, 1, 1, 0, 0, 0.0),
    (2024, 6, 15, 12, 0, 0.0),
    (2024, 12, 31, 23, 59, 59.0),
];

for (year, month, day, hour, minute, second) in dates {
    let jd = JulianDate::from_calendar(year, month, day, hour, minute, second)?;
    let pos = eph.get_position("Earth", jd)?;
    println!("{}-{:02}-{:02}: ({:.6}, {:.6}, {:.6}) AU",
             year, month, day, pos.x, pos.y, pos.z);
}
```

## API Reference

### `Ephemeris`

Main structure for accessing ephemeris data.

#### Methods

- `new(config_path: &str) -> Result<Ephemeris, Error>` - Create a new ephemeris instance
- `get_position(body_name: &str, jd: JulianDate) -> Result<Position, Error>` - Get position of a celestial body
- `get_bodies() -> Vec<&SpaceObject>` - Get list of available celestial bodies
- `get_metadata() -> EphemerisMetadata` - Get ephemeris metadata
- `get_date_range() -> (f64, f64)` - Get valid Julian date range

### `JulianDate`

Represents a Julian date.

#### Methods

- `new(jd: f64) -> JulianDate` - Create from Julian day number
- `from_calendar(year, month, day, hour, minute, second) -> Result<JulianDate, Error>` - Convert from calendar date
- `to_calendar() -> CalendarDate` - Convert to calendar date
- `as_f64() -> f64` - Get Julian day number as f64

### `Position`

3D position vector in Astronomical Units (AU).

#### Fields

- `x: f64` - X coordinate
- `y: f64` - Y coordinate
- `z: f64` - Z coordinate

#### Methods

- `new(x, y, z) -> Position` - Create a new position
- `distance() -> f64` - Calculate distance from origin

## Supported Celestial Bodies

The library supports the following celestial bodies (as defined in your `Initial_data.dat`):

- Mercury
- Venus
- EarthMoon_barycenter (Earth-Moon barycenter)
- Mars
- Jupiter
- Saturn
- Uranus
- Neptune
- Pluto
- Moon_geocentric (Moon, geocentric)
- Sun

## Error Handling

The library uses a custom `Error` type that can represent:

- Configuration errors
- I/O errors
- Invalid date errors
- Ephemeris data errors

All functions return `Result<T, Error>` for proper error handling.

## Contributing

Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests. Your valuable input is highly appreciated.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

- NASA JPL for providing the DE441 ephemeris data
- The Rust community for excellent tooling and libraries
