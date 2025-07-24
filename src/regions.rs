use crate::holidays::Holiday;

/// The `Region` enum represents the different regions (mostly German states) for which holidays
/// can be calculated. It includes variations for regions where religious or minor
/// community-specific holidays differ.
///
/// Variants:
///
/// - `BadenWuerttemberg`: Represents the state of Baden-Württemberg.
///
/// - `Bayern`: Represents the state of Bavaria.
///   - *Important Notes*:
///     - The Augsburger Friedensfest only applies to Augsburg and is excluded by default.
///       It can, however, be included using [`Region::BayernAugsburg`].
///     - Mariä Himmelfahrt is included by default, as it applies to communities with
///       a catholic majority (the majority of communities in Bavaria).
///       Can be excluded using [`Region::BayernProtestant`].
///
/// - `BayernAugsburg`: Represents the region of Augsburg in Bavaria, where the
///   Augsburger Friedensfest applies.
///
/// - `BayernCatholic`: Represents Bavaria regions with a predominantly catholic population.
///
/// - `BayernProtestant`: Represents Bavaria regions with a predominantly protestant population.
///
/// - `Berlin`: Represents the city of Berlin.
///
/// - `Brandenburg`: Represents the state of Brandenburg.
///
/// - `Bremen`: Represents the city-state of Bremen.
///
/// - `Hamburg`: Represents the city-state of Hamburg.
///
/// - `Hessen`: Represents the state of Hesse (Hessen).
///
/// - `MecklenburgVorpommern`: Represents the state of Mecklenburg-Western Pomerania.
///
/// - `Niedersachsen`: Represents the state of Lower Saxony (Niedersachsen).
///
/// - `NordrheinWestfalen`: Represents the state of North Rhine-Westphalia.
///
/// - `RheinlandPfalz`: Represents the state of Rhineland-Palatinate (Rheinland-Pfalz).
///
/// - `Saarland`: Represents the state of Saarland.
///
/// - `Sachsen`: Represents the state of Saxony.
///   - *Important Notes*: Fronleichnam (Corpus Christi) applies to only a minority
///     of communities and is excluded by default. Use [`Region::SachsenCatholic`] to include it.
///
/// - `SachsenCatholic`: Represents catholic regions within Saxony where church-specific
///   holidays might differ.
///
/// - `SachenProtestant`: Represents protestant regions within Saxony.
///
/// - `SachsenAnhalt`: Represents the state of Saxony-Anhalt.
///
/// - `SchleswigHolstein`: Represents the state of Schleswig-Holstein.
///
/// - `Thueringen`: Represents the state of Thuringia.
///   - *Important Notes*: Fronleichnam (Corpus Christi) applies to only a minority
///     of communities and is excluded by default. It can be manually calculated using
///     [`Holiday::Fronleichnam`].
///
/// - `ThueringenCatholic`: Represents catholic regions within Thuringia where church-specific
///   holidays might differ.
///
/// - `ThueringenProtestant`: Represents protestant regions within Thuringia.
///
/// # Notes
/// Some regional holidays are either excluded by default or vary based on local religious
/// or cultural factors (e.g., catholic/protestant majority). Additional configurations may
/// need to be applied for such specific cases. Refer to the respective holiday or region variant
/// for more details.
#[allow(missing_docs)]
pub enum Region {
    BadenWuerttemberg,
    /// * The Augsburger Friedensfest only applies to Augsburg.
    ///   It is excluded but can be used with [`Region::BayernAugsburg`].
    /// * Mariä Himmelfahrt only applies to communities with a catholic majority.
    ///   Since this is the case in the majority of communities, it is included by default.
    Bayern,
    BayernAugsburg,
    BayernCatholic,
    BayernProtestant,
    Berlin,
    Brandenburg,
    Bremen,
    Hamburg,
    Hessen,
    MecklenburgVorpommern,
    Niedersachsen,
    NordrheinWestfalen,
    RheinlandPfalz,
    Saarland,
    /// Fronleichnam applies only to a minority of communities and has been excluded by default.
    /// It can be manually calculated via [`Holiday::Fronleichnam`].
    Sachsen,
    SachsenCatholic,
    SachenProtestant,
    SachsenAnhalt,
    SchleswigHolstein,
    /// Fronleichnam applies only to a minority of communities and has been excluded by default.
    /// It can be manually calculated via [`Holiday::Fronleichnam`].
    Thueringen,
    ThueringenCatholic,
    ThueringenProtestant,
}

impl Region {
    /// Checks if the given date is a holiday in a specific region.
    ///
    /// # Parameters
    /// - `date`: The date to check, represented as a `time::Date`.
    ///
    /// # Returns
    /// - `true` if the given `date` is a holiday, based on the list of holidays in the region
    /// - `false` otherwise.
    pub fn is_holiday(&self, date: time::Date) -> bool {
        let holidays = self.holidays();
        let year = date.year();

        for holiday in holidays {
            if let Some(holiday) = holiday.date(year) {
                if date == holiday {
                    return true;
                }
            }
        }

        false
    }

    /// Retrieves a list of holidays specific to a given region, including nationwide holidays.
    pub fn holidays(&self) -> Vec<Holiday> {
        let mut holidays = match self {
            Region::BadenWuerttemberg => Vec::from([
                Holiday::HeiligeDreiKoenige,
                Holiday::Fronleichnam,
                Holiday::Allerheiligen,
            ]),
            Region::Bayern => Vec::from([
                Holiday::HeiligeDreiKoenige,
                Holiday::Fronleichnam,
                Holiday::MariaeHimmelfahrt,
                Holiday::Allerheiligen,
            ]),
            Region::BayernAugsburg => Vec::from([
                Holiday::HeiligeDreiKoenige,
                Holiday::Fronleichnam,
                Holiday::AugsburgerFriedensfest,
                Holiday::MariaeHimmelfahrt,
                Holiday::Allerheiligen,
            ]),
            Region::BayernCatholic => Vec::from([
                Holiday::HeiligeDreiKoenige,
                Holiday::Fronleichnam,
                Holiday::MariaeHimmelfahrt,
                Holiday::Allerheiligen,
            ]),
            Region::BayernProtestant => Vec::from([
                Holiday::HeiligeDreiKoenige,
                Holiday::Fronleichnam,
                Holiday::MariaeHimmelfahrt,
            ]),
            Region::Berlin => Vec::from([Holiday::Frauentag]),
            Region::Brandenburg => Vec::from([Holiday::Reformationstag]),
            Region::Bremen => Vec::from([Holiday::Reformationstag]),
            Region::Hamburg => Vec::from([Holiday::Reformationstag]),
            Region::Hessen => Vec::from([Holiday::Fronleichnam]),
            Region::MecklenburgVorpommern => {
                Vec::from([Holiday::Frauentag, Holiday::Reformationstag])
            }
            Region::Niedersachsen => Vec::from([Holiday::Reformationstag]),
            Region::NordrheinWestfalen => {
                Vec::from([Holiday::Fronleichnam, Holiday::Allerheiligen])
            }
            Region::RheinlandPfalz => Vec::from([Holiday::Fronleichnam, Holiday::Allerheiligen]),
            Region::Saarland => Vec::from([
                Holiday::Fronleichnam,
                Holiday::MariaeHimmelfahrt,
                Holiday::Allerheiligen,
            ]),
            Region::Sachsen => Vec::from([Holiday::Reformationstag, Holiday::BussUndBettag]),
            Region::SachsenCatholic => Vec::from([
                Holiday::Reformationstag,
                Holiday::Fronleichnam,
                Holiday::BussUndBettag,
            ]),
            Region::SachenProtestant => {
                Vec::from([Holiday::Reformationstag, Holiday::BussUndBettag])
            }
            Region::SachsenAnhalt => {
                Vec::from([Holiday::HeiligeDreiKoenige, Holiday::Reformationstag])
            }
            Region::SchleswigHolstein => Vec::from([Holiday::Reformationstag]),
            Region::Thueringen => Vec::from([Holiday::Weltkindertag, Holiday::Reformationstag]),
            Region::ThueringenCatholic => {
                Vec::from([Holiday::Fronleichnam, Holiday::Reformationstag])
            }
            Region::ThueringenProtestant => Vec::from([Holiday::Reformationstag]),
        };

        holidays.extend_from_slice(NATION_WIDE_HOLIDAYS);

        holidays
    }
}

/// A constant array of holidays that are observed nationwide.
pub const NATION_WIDE_HOLIDAYS: &[Holiday] = &[
    Holiday::Neujahr,
    Holiday::Karfreitag,
    Holiday::Ostermontag,
    Holiday::TagDerArbeit,
    Holiday::ChristiHimmelfahrt,
    Holiday::Pfingstmontag,
    Holiday::TagDerDeutschenEinheit,
    Holiday::ErsterWeihnachtsfeiertag,
    Holiday::ZweiterWeihnachtsfeiertag,
];
