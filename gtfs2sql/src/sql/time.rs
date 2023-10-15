use crate::sql::time::TimestampPart::{Hours, Minutes, Seconds};
use serde::de::{Error, Visitor};
use serde::{de, Deserialize, Deserializer};
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use std::u64;

/// Represents GTFS Time stored in seconds for simplicity.
/// (Not the HH:MM:SS format)
///
/// > Time in the HH:MM:SS format (H:MM:SS is also accepted).
/// > The time is measured from "noon minus 12h" of the service day (effectively midnight except for days on which daylight savings time changes occur). For times occurring after midnight on the service day, enter the time as a value greater than 24:00:00 in HH:MM:SS.
/// > Example: 14:30:00 for 2:30PM or 25:35:00 for 1:35AM on the next day.
/// >  [GTFS Reference](https://gtfs.org/schedule/reference/#field-types)
///
/// Note: This differentiates from Time for the RAPTOR algorithm as the RAPTOR has the concept of
/// infinite/finite time which GTFS does not
#[derive(Debug, Copy, Clone)]
pub(crate) struct Time(u64);

impl Time {
    pub(super) fn new(value: u64) -> Time {
        Time(value)
    }

    pub(super) fn total_seconds(self) -> u64 {
        self.0
    }
}

impl PartialEq<Self> for Time {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for Time {}

/// Represents a section in GTFS time stamp in the format of HH:MM:SS
#[derive(Debug)]
enum TimestampPart {
    Hours,
    Minutes,
    Seconds,
}

/// Describes which value of the timestamp was out of range. Hours can not be out of range
#[derive(Debug)]
enum TimeFraction {
    Minutes,
    Seconds,
}
#[derive(Debug)]
pub enum ParseTimeError {
    /// The provided string was not 7 or 8 in length.
    /// The value represents the provided invalid string length.
    InvalidLength(usize),

    /// Minutes or Seconds were not 2 characters long
    InvalidPartLength(TimeFraction, usize),
    /// The format was not HH:MM:SS or H:MM:SS.
    /// The value represents the part that was missing or at which parsing failed
    InvalidFormat(TimestampPart),
    /// Failed parsing either hours, minutes or seconds due to it being invalid
    ParseIntError(TimestampPart, ParseIntError),
    /// A parsed value was greater than 59 for either seconds or minutes.
    /// The first value indicates what was out of range and the second value is the value that is
    /// out of range.
    /// Hours can not be out of range as 24+ hours is valid to represent things that extend to the
    /// next day
    ValueOutOfRange(TimeFraction, u64),
}
impl FromStr for Time {
    type Err = ParseTimeError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.len() {
            7 | 8 => {
                let mut splits = string.split(':');
                let hours = splits.next().ok_or(ParseTimeError::InvalidFormat(Hours))?;
                let hours = u64::from_str(hours)
                    .map_err(|error| ParseTimeError::ParseIntError(Hours, error))?;
                let minutes = splits
                    .next()
                    .ok_or(ParseTimeError::InvalidFormat(Minutes))?;

                let length = minutes.len();
                if length != 2 {
                    return Err(ParseTimeError::InvalidPartLength(
                        TimeFraction::Minutes,
                        length,
                    ));
                }

                let minutes = u64::from_str(minutes)
                    .map_err(|error| ParseTimeError::ParseIntError(Minutes, error))?;
                if minutes > 59 {
                    return Err(ParseTimeError::ValueOutOfRange(
                        TimeFraction::Minutes,
                        minutes,
                    ));
                }

                let seconds = splits
                    .next()
                    .ok_or(ParseTimeError::InvalidFormat(Seconds))?;

                let length = seconds.len();
                if length != 2 {
                    return Err(ParseTimeError::InvalidPartLength(
                        TimeFraction::Seconds,
                        length,
                    ));
                }

                let seconds = u64::from_str(seconds)
                    .map_err(|error| ParseTimeError::ParseIntError(Seconds, error))?;

                if seconds > 59 {
                    return Err(ParseTimeError::ValueOutOfRange(
                        TimeFraction::Seconds,
                        seconds,
                    ));
                }

                let total_seconds = (hours * 60 * 60) + (minutes * 60) + seconds;

                Ok(Time(total_seconds))
            }
            other => Err(ParseTimeError::InvalidLength(other)),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::sql::time::{ParseTimeError, Time};
    use std::str::FromStr;

    #[test]
    fn equals() {
        // Arrange
        let a = Time(1);
        let b = Time(2);
        let c = Time(1);

        // Assert
        assert_ne!(a, b);
        assert_eq!(a, c);
    }

    #[test]
    fn time_from_string() {
        // Arrange
        let a = "12:34:56";
        let expected_a = Time((12 * 60 * 60) + (34 * 60) + 56);
        let b = "69:42:00";
        let expected_b = Time((69 * 60 * 60) + (42 * 60) + 00);

        // Act
        let actual_a = Time::from_str(a).unwrap();
        let actual_b = Time::from_str(b).unwrap();

        // Assert
        assert_eq!(expected_a, actual_a);
        assert_eq!(expected_b, actual_b);
    }

    #[test]
    fn can_not_parse_invalid_time() {
        // Arrange
        let test_values = vec![
            "-69:04:20",
            "6:94:20",
            "69:4:20",
            "694:20:0",
            "69:04:200",
            "lol:00:00",
            "12:lol:00",
            ":12:34",
            "12:34:",
            "12::34",
        ];

        // Act
        let results: Vec<Result<Time, ParseTimeError>> = test_values
            .iter()
            .map(|string| Time::from_str(string))
            .collect();

        // Assert
        assert!(results.iter().all(Result::is_err));
    }
}

impl Display for Time {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        // Idk how to do basic math
        let (seconds, minutes) = (self.0 % 60, self.0 / 60);
        let (seconds, minutes, hours) = (seconds, minutes % 60, minutes / 60);
        write!(formatter, "{hours}:{minutes}:{seconds}")
    }
}

// Deserialization specific code for serde
struct TimeVisitor;

impl<'de> Visitor<'de> for TimeVisitor {
    type Value = Time;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str(
            "a string in the format HH:MM:SS or H:MM:SS. Can be a value greater than 24:00:00",
        )
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        //TODO improve error message
        Time::from_str(value).map_err(|_| de::Error::custom("Invalid time value"))
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TimeVisitor)
    }
}
