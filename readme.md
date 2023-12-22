# Rust JPL Ephemeris Reader

The Rust JPL Ephemeris Reader is a Rust project designed to read NASA JPL ephemeris data in the .bsp (Binary SPICE) format and provide planetary positions based on a given Julian date. NASA's Jet Propulsion Laboratory (JPL) releases ephemeris data containing accurate positions of celestial bodies, which is crucial for various scientific and engineering applications.

This project utilizes the Rust programming language to create a flexible and efficient tool for extracting planetary positions from the DE441.bsp ephemeris file. The DE441.bsp file is part of the Development Ephemeris (DE) series, and it provides accurate planetary positions for a wide range of dates.

## Ephemeris

1. **Downlod NASA JPL :**
    - [https://ssd.jpl.nasa.gov/ftp/eph/planets/bsp/](https://ssd.jpl.nasa.gov/ftp/eph/planets/bsp/)



## Example

```sh
# Assuming DE441.bsp is in the project directory
cargo run --release -- 2451545.0 4
```

Output:
```
Planet 4: (x, y, z)
```

## Contributing

Feel free to contribute by submitting issues, feature requests, or pull requests. Your input is highly appreciated.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.