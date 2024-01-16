# Rust JPL Ephemeris Reader

The Rust JPL Ephemeris Reader is an advanced Rust project tailored for parsing NASA JPL ephemeris data, providing precise planetary positions based on a given Julian date. Crafted in Rust for enhanced flexibility and efficiency, this tool harnesses the NASA JPL DE441 Library, offering a plethora of features crucial for diverse scientific and engineering applications.

## Features

| Feature                 | Description                                                     | Status  |
|-------------------------|-----------------------------------------------------------------|---------|
| Planetary Positions     | Accurate positions of celestial bodies for a given Julian date. | Planned |
| Ephemeris Data          | Utilization of DE441 file for precise planetary information.    | Planned |
| Scientific Applications | Essential for scientific and engineering calculations.          | Planned |
| Flexibility             | Developed in Rust for flexibility and efficiency.               | ✓       |
| Solar System Dynamics   | Detailed data for understanding solar system dynamics.          | Planned |
| Gravitational Effects   | Calculation of gravitational effects on celestial bodies.       | Planned |
| Time Conversions        | Conversion of time formats for diverse applications.            | Planned |
| Multiple Ephemerides    | Support for various ephemerides for specialized needs.          | Planned |
| Celestial Phenomena     | Prediction and tracking of celestial events and phenomena.      | Planned |
| Lunar Ephemerides       | Specific data for accurate moon-related calculations.           | Planned |

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

## Ephemeris

1. **Download NASA JPL:**
   - Obtain the NASA JPL DE441 Library files `linux_m13000p17000.441` and `header.441` from [NASA JPL FTP](https://ssd.jpl.nasa.gov/ftp/eph/planets/Linux/de441/). Set the file paths in the `.env` configuration file.
   
   - Rename `linux_m13000p17000.441` to `linux_m13000p17000.441.bsp` and place it in the `assets/` directory.

## Configuration

1. Duplicate the `env` file and rename it to `.env`.

2. Set the paths for `NASA_JPL_DE441` and `HEADER_441` in the `.env` file.

## Contributing

Contributions are welcome through the submission of issues, feature requests, or pull requests. Your valuable input is highly appreciated. Feel free to contribute to make this tool even more robust and versatile.