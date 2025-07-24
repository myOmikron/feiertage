//! # feiertage
//!
//! This crate provides functionality to calculate and verify German holidays.
//!
//! Its main capabilities include:
//! 1. **Holiday Detection**:
//!     - Determine if a specific date is a holiday in a given German region.
//!     - Supports all federal and regional holidays, including holidays specific to certain communities (e.g., Augsburg's Friedensfest).
//!
//! 2. **Region-Specific Calculation**:
//!     - The crate defines German regions ([`Region`]) to handle region-specific holidays.
//!     - These include Catholic and Protestant variations where applicable.
//!
//! 3. **Holiday List**:
//!     - Provides a flexible way to retrieve region-specific holidays or nationwide holidays.
//!
//!
//! # Example
//! ```
//! use feiertage::HolidayExt;
//! use feiertage::Region;
//!
//! let date = time::Date::from_calendar_date(2023, time::Month::December, 25).unwrap();
//!
//! // Returns true if the date is a holiday in the region of Bayern
//! assert!(date.is_holiday(Region::Bayern));
//! ```

#![warn(missing_docs)]

pub(crate) mod holidays;
pub(crate) mod regions;

pub use crate::holidays::*;
pub use crate::regions::*;

/// A trait that provides an extension method to check if a given date is a holiday
/// within a specified region.
///
/// # Example
/// ```
/// use feiertage::HolidayExt;
/// use feiertage::Region;
///
/// let date = time::Date::from_calendar_date(2023, time::Month::December, 25).unwrap();
///
/// // Returns true if the date is a holiday in the region of Bayern
/// assert!(date.is_holiday(Region::Bayern));
/// ```
pub trait HolidayExt {
    /// Checks if the given day is a holiday in the specified region.
    ///
    /// # Parameters
    /// - `region`: A `Region` enum that specifies the region to check for holidays.
    ///
    /// # Returns
    /// - `true` if the current day is a holiday in the specified region.
    /// - `false` if the current day is not a holiday in the specified region.
    ///
    /// # Example
    /// ```
    /// use feiertage::HolidayExt;
    /// use feiertage::Region;
    ///
    /// let date = time::Date::from_calendar_date(2023, time::Month::December, 25).unwrap();
    ///
    /// // Returns true if the date is a holiday in the region of Bayern
    /// assert!(date.is_holiday(Region::Bayern));
    /// ```
    fn is_holiday(&self, region: Region) -> bool;
}

impl HolidayExt for time::Date {
    fn is_holiday(&self, region: Region) -> bool {
        region.is_holiday(*self)
    }
}
