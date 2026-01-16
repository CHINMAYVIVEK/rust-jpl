//! Time conversion utilities for Julian dates and calendar dates

use crate::Error;

/// Represents a Julian Date (JD)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct JulianDate {
    /// Julian day number
    pub jd: f64,
}

impl JulianDate {
    /// Create a Julian date from a Julian day number
    pub fn new(jd: f64) -> Self {
        Self { jd }
    }

    /// Convert calendar date to Julian date
    ///
    /// # Arguments
    /// * `year` - Year (e.g., 2024)
    /// * `month` - Month (1-12)
    /// * `day` - Day of month (1-31)
    /// * `hour` - Hour (0-23)
    /// * `minute` - Minute (0-59)
    /// * `second` - Second (0-59.999...)
    ///
    /// # Example
    /// ```
    /// use rust_jpl::JulianDate;
    /// let jd = JulianDate::from_calendar(2024, 1, 15, 12, 0, 0.0);
    /// ```
    pub fn from_calendar(
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        second: f64,
    ) -> Result<Self, Error> {
        if !(1..=12).contains(&month) {
            return Err(Error::InvalidDate(format!("Invalid month: {}", month)));
        }
        if !(1..=31).contains(&day) {
            return Err(Error::InvalidDate(format!("Invalid day: {}", day)));
        }
        if !(0..=23).contains(&hour) {
            return Err(Error::InvalidDate(format!("Invalid hour: {}", hour)));
        }
        if !(0..=59).contains(&minute) {
            return Err(Error::InvalidDate(format!("Invalid minute: {}", minute)));
        }
        if !(0.0..60.0).contains(&second) {
            return Err(Error::InvalidDate(format!("Invalid second: {}", second)));
        }

        let a = (14 - month) / 12;
        let y = year + 4800 - a;
        let m = month + 12 * a - 3;

        let jdn = day as f64 + (153 * m + 2) as f64 / 5.0 + 365.0 * y as f64 + (y / 4) as f64
            - (y / 100) as f64
            + (y / 400) as f64
            - 32045.0;

        let fraction = (hour as f64 + minute as f64 / 60.0 + second / 3600.0) / 24.0;

        Ok(Self {
            jd: jdn + fraction - 0.5,
        })
    }

    /// Convert Julian date to calendar date
    pub fn to_calendar(&self) -> CalendarDate {
        let j = (self.jd + 0.5).floor() as i64;
        let f = self.jd + 0.5 - j as f64;

        let j0 = j;
        let j1 = j0 + 68569;
        let j2 = 4 * j1 / 146097;
        let j3 = j1 - (146097 * j2 + 3) / 4;
        let j4 = 4000 * (j3 + 1) / 1461001;
        let j5 = j3 - 1461 * j4 / 4 + 31;
        let j6 = 80 * j5 / 2447;
        let j7 = j5 - 2447 * j6 / 80;
        let j8 = j6 / 11;
        let j9 = j6 + 2 - 12 * j8;
        let j10 = 100 * (j2 - 49) + j4 + j8;

        let year = j10 as i32;
        let month = j9 as i32;
        let day = j7 as i32;

        let total_seconds = f * 86400.0;
        let hour = (total_seconds / 3600.0) as i32;
        let minute = ((total_seconds % 3600.0) / 60.0) as i32;
        let second = total_seconds % 60.0;

        CalendarDate {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }

    /// Get the Julian day number
    pub fn as_f64(&self) -> f64 {
        self.jd
    }
}

/// Represents a calendar date
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CalendarDate {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: f64,
}

impl CalendarDate {
    /// Create a new calendar date
    pub fn new(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: f64) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }

    /// Convert to Julian date
    pub fn to_julian(&self) -> Result<JulianDate, Error> {
        JulianDate::from_calendar(
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_julian_date_conversion() {
        let jd = JulianDate::from_calendar(2024, 1, 15, 12, 0, 0.0).unwrap();
        let cal = jd.to_calendar();
        assert_eq!(cal.year, 2024);
        assert_eq!(cal.month, 1);
        assert_eq!(cal.day, 15);
    }
}
