use time::Date;
use time::Duration;
use time::Month;

/// All reoccurring holidays in Germany.
/// This list contains both public and non-public holidays.
///
/// For public holidays use [`crate::regions::Region`] instead, since
/// public holidays differ from region to region.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum Holiday {
    Neujahr,
    HeiligeDreiKoenige,
    Frauentag,
    Karfreitag,
    Ostermontag,
    TagDerArbeit,
    TagDerBefreiung,
    ChristiHimmelfahrt,
    Pfingstsonntag,
    Pfingstmontag,
    Fronleichnam,
    AugsburgerFriedensfest,
    MariaeHimmelfahrt,
    Weltkindertag,
    TagDerDeutschenEinheit,
    Reformationstag,
    Allerheiligen,
    BussUndBettag,
    ErsterWeihnachtsfeiertag,
    ZweiterWeihnachtsfeiertag,
}

impl Holiday {
    /// Returns the date of the holiday for a given year, if applicable.
    ///
    /// This method calculates the specific `Date` that corresponds to the holiday
    /// for the provided year. Each holiday has a predefined date or a computation
    /// logic tied to it (e.g., some holidays are fixed on the calendar, while others
    /// are relative to Easter Sunday).
    ///
    /// # Parameters
    /// - `year`: The year for which the holiday date is to be calculated
    ///
    /// # Returns
    /// - `Option<Date>`:
    ///   - `Some(Date)`: If the holiday occurs in the specified year.
    ///   - `None`: If the calculation does not succeed, or the holiday is not defined for that year.
    pub fn date(&self, year: i32) -> Option<Date> {
        match self {
            Holiday::Neujahr => Date::from_calendar_date(year, Month::January, 1).ok(),
            Holiday::HeiligeDreiKoenige => Date::from_calendar_date(year, Month::January, 6).ok(),
            Holiday::Frauentag => Date::from_calendar_date(year, Month::March, 8).ok(),
            Holiday::Karfreitag => relative_to_easter_sunday(year, -2),
            Holiday::Ostermontag => relative_to_easter_sunday(year, 1),
            Holiday::TagDerArbeit => Date::from_calendar_date(year, Month::May, 1).ok(),
            Holiday::TagDerBefreiung => Date::from_calendar_date(year, Month::May, 8).ok(),
            Holiday::ChristiHimmelfahrt => relative_to_easter_sunday(year, 39),
            Holiday::Pfingstsonntag => relative_to_easter_sunday(year, 49),
            Holiday::Pfingstmontag => relative_to_easter_sunday(year, 50),
            Holiday::Fronleichnam => relative_to_easter_sunday(year, 60),
            Holiday::AugsburgerFriedensfest => {
                Date::from_calendar_date(year, Month::August, 8).ok()
            }
            Holiday::MariaeHimmelfahrt => Date::from_calendar_date(year, Month::August, 15).ok(),
            Holiday::Weltkindertag => Date::from_calendar_date(year, Month::September, 20).ok(),
            Holiday::TagDerDeutschenEinheit => {
                Date::from_calendar_date(year, Month::October, 3).ok()
            }
            Holiday::Reformationstag => Date::from_calendar_date(year, Month::October, 31).ok(),
            Holiday::Allerheiligen => Date::from_calendar_date(year, Month::November, 1).ok(),
            Holiday::BussUndBettag => bus_und_bettag(year),
            Holiday::ErsterWeihnachtsfeiertag => {
                Date::from_calendar_date(year, Month::December, 25).ok()
            }
            Holiday::ZweiterWeihnachtsfeiertag => {
                Date::from_calendar_date(year, Month::December, 26).ok()
            }
        }
    }
}

/// Calculate bus und bettag
fn bus_und_bettag(year: i32) -> Option<Date> {
    let reference_date = Date::from_calendar_date(year, Month::November, 23).ok()?;
    let weekday_ordinal = i64::from(reference_date.weekday().number_days_from_monday());
    let duration_to_previous_wednesday = if weekday_ordinal < 3 {
        Duration::days(-(weekday_ordinal + 5))
    } else {
        Duration::days(2 - weekday_ordinal)
    };
    Some(reference_date + duration_to_previous_wednesday)
}

/// Calculate a date relative to east sunday
fn relative_to_easter_sunday(year: i32, days_offset: i64) -> Option<Date> {
    let easter_sunday = computus_gregorian(year)?;
    Some(easter_sunday + Duration::days(days_offset))
}

/// Easter in the Gregorian calendar
///
/// Calculated using [computus](https://en.wikipedia.org/wiki/Date_of_Easter)
fn computus_gregorian(year: i32) -> Option<Date> {
    if !(1583..=9999).contains(&year) {
        return None;
    }
    let aa = year % 19;
    let bb = year / 100;
    let cc = year % 100;
    let dd = bb / 4;
    let ee = bb % 4;
    let ff = (bb + 8) / 25;
    let gg = (bb - ff + 1) / 3;
    let hh = (19 * aa + bb - dd - gg + 15) % 30;
    let ii = cc / 4;
    let kk = cc % 4;
    let ll = (32 + 2 * ee + 2 * ii - hh - kk) % 7;
    let mm = (aa + 11 * hh + 22 * ll) / 451;
    let month = (hh + ll - 7 * mm + 114) / 31;
    let day = (hh + ll - 7 * mm + 114) % 31 + 1;

    Date::from_calendar_date(year, Month::try_from(month as u8).ok()?, day as u8).ok()
}
