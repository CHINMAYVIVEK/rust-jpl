# Rust JPL Ephemeris Reader

The Rust JPL Ephemeris Reader is a Rust project designed to read NASA JPL ephemeris data in the .bsp (Binary SPICE) format and provide planetary positions based on a given Julian date. NASA's Jet Propulsion Laboratory (JPL) releases ephemeris data containing accurate positions of celestial bodies, which is crucial for various scientific and engineering applications.

This project utilizes the Rust programming language to create a flexible and efficient tool for extracting planetary positions from the DE441.bsp ephemeris file. The DE441.bsp file is part of the Development Ephemeris (DE) series, and it provides accurate planetary positions for a wide range of dates.

## Features

- **Read Ephemeris Data:** The project is capable of reading ephemeris data from the DE441.bsp file, interpreting its specific binary format.

- **Calculate Planetary Positions:** Given a Julian date and a planet identifier, the project calculates and outputs the 3D position of the specified celestial body.

## Usage

1. **Installation:**
    - Clone the repository: `git clone https://github.com/CHINMAYVIVEK/rust-jpl.git`
    - Change into the project directory: `cd rust-jpl`

2. **Build and Run:**
    - Build the project: `cargo build --release`
    - Run the executable: `cargo run --release`

3. **Input Parameters:**
    - Provide the path to the DE441.bsp file in the `Cargo.toml` file or as a command-line argument.
    - Specify the Julian date and planet identifier to get the planetary position.

4. **Output:**
    - The project outputs the calculated 3D position of the specified celestial body.

## Example

```sh
# Assuming DE441.bsp is in the project directory
cargo run --release -- 2451545.0 4
```

Output:
```
Planet 4: (x, y, z)
```

## Dependencies

- **cheerio:** Used for reading and parsing SPICE kernels. [cheerio GitHub](https://github.com/DanielKeep/cheerio)
- **nalgebra:** Utilized for representing 3D vectors to store planetary positions. [nalgebra GitHub](https://github.com/dimforge/nalgebra)

## Contributing

Feel free to contribute by submitting issues, feature requests, or pull requests. Your input is highly appreciated.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.