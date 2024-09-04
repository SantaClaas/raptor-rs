use std::fmt::{Display, Formatter};
use time::format_description::well_known::{iso8601, Iso8601};
use time::format_description::well_known::iso8601::TimePrecision;
use time::{error, PrimitiveDateTime};
use time::macros::format_description;

pub(self) const CONFIGURATION: iso8601::EncodedConfig = iso8601::Config::DEFAULT
    .set_time_precision(TimePrecision::Second {
        decimal_digits: None,
    })
    .encode();

pub(self) const FORMAT: Iso8601<CONFIGURATION> = Iso8601::<CONFIGURATION>;

time::serde::format_description!(date_time_local, PrimitiveDateTime, FORMAT);

/// A wrapper around [PrimitiveDateTime] to make handling of datetime-local strings from HTML inputs
/// easier. The date and time is presumed to be in the user's local timezone.
/// For more details see [Local date and time strings](https://developer.mozilla.org/en-US/docs/Web/HTML/Date_and_time_formats#local_date_and_time_strings)
#[derive(serde::Deserialize)]
#[repr(transparent)]
pub(crate) struct DateTimeLocal(#[serde(with = "date_time_local")] PrimitiveDateTime);

impl DateTimeLocal {
    pub(crate) fn format(&self) -> Result<String, error::Format> {
        let format = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
        self.0.format(format)
    }

    pub(crate) fn to_seconds(&self) -> u64 {
        let (hours, minutes, seconds) = self.0.as_hms();
        hours as u64 * 60 * 60 + minutes as u64 * 60 + seconds as u64
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct SearchConnectionRequest {
    pub(crate) start: Option<String>,
    pub(crate) end: Option<String>,
    pub(crate) departure: Option<DateTimeLocal>,
}