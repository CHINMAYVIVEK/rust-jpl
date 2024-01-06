use std::fs::File;
use std::io::Write;

#[derive(Default)]
struct Date {
    j_date: f64,
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    second: i32,
}

struct SpaceObject {
    object_state: bool,
    object_header_data: Vec<i32>,
    object_name: String,
    obj_file: File,
    object_length: i32,
    first_step: bool,
}

impl Default for SpaceObject {
    fn default() -> Self {
        SpaceObject {
            object_state: false,
            object_header_data: Vec::new(),
            object_name: String::new(),
            obj_file: File::create("filename.txt").expect("Unable to create file"),
            object_length: 0,
            first_step: false,
        }
    }
}

struct Ephemerides {
    date: Date,
    bodies: Vec<SpaceObject>,
    coef: Vec<Vec<f64>>,
    start_year: i32,
    end_year: i32,
    ncoeff: i32,
    emrat: f64,
    blocks_number: i32,
    interval: i32,
    julian_start: f64,
    julian_end: f64,
}

impl Default for Ephemerides {
    fn default() -> Self {
        let mut coef = Vec::with_capacity(230 * 9);
        for _ in 0..(230 * 9) {
            coef.push(vec![0.0; 1020]);
        }

        Ephemerides {
            date: Date::default(),
            bodies: Vec::new(),
            coef,
            start_year: 0,
            end_year: 0,
            ncoeff: 0,
            emrat: 0.0,
            blocks_number: 0,
            interval: 0,
            julian_start: 0.0,
            julian_end: 0.0,
        }
    }
}
