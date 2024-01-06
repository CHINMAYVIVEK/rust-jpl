# Rust JPL Ephemeris Reader

The Rust JPL Ephemeris Reader is a Rust project designed to read NASA JPL ephemeris data and provide planetary positions based on a given Julian date. NASA's Jet Propulsion Laboratory (JPL) releases ephemeris data containing accurate positions of celestial bodies, which is crucial for various scientific and engineering applications.

This project utilizes the Rust programming language to create a flexible and efficient tool for extracting planetary positions from the ephemeris file. The DE441 file is part of the Development Ephemeris (DE) series, and it provides accurate planetary positions for a wide range of dates.

## Ephemeris

1. **Download NASA JPL:**
   - Download NASA JPL DE441 Library `linux_m13000p17000.441` and `header.441` [https://ssd.jpl.nasa.gov/ftp/eph/planets/Linux/de441/](https://ssd.jpl.nasa.gov/ftp/eph/planets/Linux/de441/) and set the path of files in .env

   - Rename `linux_m13000p17000.441` to `linux_m13000p17000.441.bsp` and place in `assets/` directory 


## Configuration

1. Make a copy of `env` and rename it to `env`.

2. Set the path of NASA_JPL_DE441 in `.env`.

2. Set the path of HEADER_441 in `.env`.

## Contributing

Feel free to contribute by submitting issues, feature requests, or pull requests. Your input is highly appreciated.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
