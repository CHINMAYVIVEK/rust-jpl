# Rust JPL Ephemeris Reader (`rust-jpl`)

`rust-jpl` is a Rust library that enables **integration of NASA Jet Propulsion Laboratory (JPL) ephemeris data** into Rust applications.  
It provides **high-precision planetary and lunar positions** based on a given **Julian date**, using official **NASA JPL DE ephemerides (DE441)**.

The library is designed for **scientific correctness, API clarity, and performance**, making it suitable for research, engineering, simulation, and educational use.

## 🧭 Use Cases

- Astronomy and astrophysics research
- Orbital mechanics and trajectory planning
- Space mission planning
- Celestial navigation
- Satellite and ground-station tracking
- Educational tools and simulations
- Scientific visualization software

---

## 🛠 Requirements

| Requirement     | Version               |
| --------------- | --------------------- |
| **Rust (MSRV)** | **1.70.0** or newer   |
| Cargo           | Comes with Rust       |
| Supported OS    | Linux, macOS, Windows |
| Architecture    | x86_64, aarch64       |

> The MSRV is documented and respected. Breaking MSRV changes will require a minor or major release.

---

## 📦 Installation

### From crates.io

```toml
[dependencies]
rust-jpl = "0.0.1-alpha"
```

---

## 🚀 Quick Start

### Basic Usage

```rust
use rust_jpl::{Ephemeris, JulianDate};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut eph = Ephemeris::new("config.toml")?;

    let jd = JulianDate::from_calendar(2024, 1, 15, 12, 0, 0.0)?;
    let position = eph.get_position("Earth", jd)?;

    println!(
        "Earth position: ({}, {}, {}) AU",
        position.x, position.y, position.z
    );

    Ok(())
}
```

---

## ⏱ Time Conversion

```rust
use rust_jpl::{CalendarDate, JulianDate};

let cal = CalendarDate::new(2024, 1, 15, 12, 0, 0.0);
let jd = cal.to_julian()?;
println!("Julian Date: {}", jd.as_f64());

let cal2 = jd.to_calendar();
println!("Calendar: {}-{:02}-{:02}", cal2.year, cal2.month, cal2.day);
```

---

## 🪐 Planetary Positions

```rust
use rust_jpl::{Ephemeris, JulianDate};

let mut eph = Ephemeris::new("config.toml")?;
let jd = JulianDate::from_calendar(2024, 1, 15, 12, 0, 0.0)?;

let sun = eph.get_position("Sun", jd)?;
let earth = eph.get_position("Earth", jd)?;
let mars = eph.get_position("Mars", jd)?;

println!("Sun: ({:.6}, {:.6}, {:.6}) AU", sun.x, sun.y, sun.z);
println!("Distance from origin: {:.6} AU", sun.distance());
```

---

## 📊 Ephemeris Metadata

```rust
use rust_jpl::Ephemeris;

let mut eph = Ephemeris::new("config.toml")?;
let metadata = eph.get_metadata();

println!("Date Range: {} - {}", metadata.start_year, metadata.end_year);
println!("Julian Range: {} - {}", metadata.julian_start, metadata.julian_end);
println!("Interval: {} days", metadata.interval_days);
println!("Earth–Moon Mass Ratio: {}", metadata.earth_moon_ratio);

for body in eph.get_bodies() {
    println!("{}: {}", body.name, if body.active { "active" } else { "inactive" });
}
```

---

## ⚙️ Configuration

Copy the example configuration file:

```bash
cp config.toml.example config.toml
```

### Example `config.toml`

```toml
[paths]
nasa_jpl_de441 = "assets/linux_m13000p17000.441.bsp"
header_441 = "assets/header.441"
initial_data_dat = "assets/Initial_data.dat"
```

---

## 📥 Ephemeris File Setup

### 1. Download NASA JPL DE441

- Source: NASA JPL Solar System Dynamics
- Required files:
  - `linux_m13000p17000.441`
  - `header.441`

Rename:

```
linux_m13000p17000.441 → linux_m13000p17000.441.bsp
```

Place files in the `assets/` directory.

---

### 2. Create `Initial_data.dat`

```text
BODIES:

Mercury                 true
Venus                   true
EarthMoon_barycenter     true
Mars                    true
Jupiter                 true
Saturn                  true
Uranus                  true
Neptune                 true
Pluto                   true
Moon_geocentric          true
Sun                     true

DATE:

Start_year              1940
End_year                2100
```

---

## 🧪 Examples

```bash
cargo run --example basic_usage
cargo run --example time_conversion
cargo run --example planetary_positions
```

---
