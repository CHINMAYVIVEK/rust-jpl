//! Time conversion example
//!
//! Demonstrates conversion between calendar dates and Julian dates

use rust_jpl::{CalendarDate, JulianDate};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Time Conversion Examples ===\n");

    // Example 1: Convert calendar date to Julian date
    println!("Example 1: Calendar to Julian Date");
    let cal = CalendarDate::new(2024, 1, 15, 12, 0, 0.0);
    let jd = cal.to_julian()?;
    println!(
        "  Calendar: {}-{:02}-{:02} {:02}:{:02}:{:02}",
        cal.year, cal.month, cal.day, cal.hour, cal.minute, cal.second as i32
    );
    println!("  Julian Date: {:.6}\n", jd.as_f64());

    // Example 2: Convert Julian date to calendar date
    println!("Example 2: Julian to Calendar Date");
    let jd2 = JulianDate::new(2460327.0); // January 15, 2024
    let cal2 = jd2.to_calendar();
    println!("  Julian Date: {:.6}", jd2.as_f64());
    println!(
        "  Calendar: {}-{:02}-{:02} {:02}:{:02}:{:02}\n",
        cal2.year, cal2.month, cal2.day, cal2.hour, cal2.minute, cal2.second as i32
    );

    // Example 3: Using from_calendar directly
    println!("Example 3: Direct conversion");
    let jd3 = JulianDate::from_calendar(2000, 1, 1, 12, 0, 0.0)?;
    println!("  January 1, 2000 12:00:00 UTC");
    println!("  Julian Date: {:.6}\n", jd3.as_f64());

    // Example 4: Historical dates
    println!("Example 4: Historical Dates");
    let dates = vec![
        (1969, 7, 20, 20, 17, 0.0, "Apollo 11 Moon Landing"),
        (1986, 1, 28, 11, 39, 0.0, "Space Shuttle Challenger"),
        (2021, 7, 20, 9, 13, 0.0, "Blue Origin NS-16"),
    ];

    for (year, month, day, hour, minute, second, event) in dates {
        let jd = JulianDate::from_calendar(year, month, day, hour, minute, second)?;
        println!("  {}: JD {:.6}", event, jd.as_f64());
    }

    Ok(())
}
