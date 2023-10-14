use crate::sql::time::Time;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub(crate) struct Agency {
    #[serde(rename = "agency_id")]
    pub(crate) id: String,
    #[serde(rename = "agency_name")]
    pub(crate) name: String,
    #[serde(rename = "agency_url")]
    pub(crate) url: String,
    #[serde(rename = "agency_timezone")]
    pub(crate) timezone: String,
    #[serde(rename = "agency_lang")]
    pub(crate) language: Option<String>,
    #[serde(rename = "agency_phone")]
    pub(crate) phone: Option<String>,
    #[serde(rename = "agency_fare_url")]
    pub(crate) fare_url: Option<String>,
    #[serde(rename = "agency_email")]
    pub(crate) email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Stop {
    #[serde(rename = "stop_id")]
    pub(crate) id: String,
    #[serde(rename = "stop_code")]
    pub(crate) code: Option<String>,
    #[serde(rename = "stop_name")]
    pub(crate) name: Option<String>,
    #[serde(rename = "tts_stop_name")]
    pub(crate) text_to_speech_name: Option<String>,
    #[serde(rename = "stop_desc")]
    pub(crate) description: Option<String>,
    #[serde(rename = "stop_lat")]
    pub(crate) latitude: Option<f32>,
    #[serde(rename = "stop_long")]
    pub(crate) longitude: Option<f32>,
    pub(crate) zone_id: Option<String>,
    #[serde(rename = "stop_url")]
    pub(crate) url: Option<String>,
    pub(crate) location_type: Option<u8>,
    pub(crate) parent_station: Option<String>,
    #[serde(rename = "stop_timezone")]
    pub(crate) timezone: Option<String>,
    pub(crate) wheelchair_boarding: Option<u8>,
    pub(crate) level_id: Option<String>,
    pub(crate) platform_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Route {
    #[serde(rename = "route_id")]
    pub(crate) id: String,
    #[serde(rename = "stop_timezone")]
    pub(crate) agency_id: Option<String>,
    #[serde(rename = "route_short_name")]
    pub(crate) short_name: Option<String>,
    #[serde(rename = "route_long_name")]
    pub(crate) long_name: Option<String>,
    #[serde(rename = "route_desc")]
    pub(crate) description: Option<String>,
    pub(crate) route_type: u8,
    #[serde(rename = "route_url")]
    pub(crate) url: Option<String>,
    #[serde(rename = "route_color")]
    pub(crate) color: Option<String>,
    #[serde(rename = "route_text_color")]
    pub(crate) text_color: Option<String>,
    #[serde(rename = "route_sort_order")]
    pub(crate) sort_order: Option<u32>,
    pub(crate) continuous_pickup: Option<u8>,
    pub(crate) continuous_drop_off: Option<u8>,
    pub(crate) network_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Trip {
    pub(crate) route_id: String,
    pub(crate) service_id: String,
    #[serde(rename = "trip_id")]
    pub(crate) id: String,
    #[serde(rename = "trip_headsign")]
    pub(crate) headsign: Option<String>,
    #[serde(rename = "trip_short_name")]
    pub(crate) short_name: Option<String>,
    pub(crate) direction_id: Option<u8>,
    pub(crate) block_id: Option<String>,
    pub(crate) shape_id: Option<String>,
    pub(crate) wheelchair_accessible: Option<u8>,
    pub(crate) bikes_allowed: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct StopTime {
    pub(crate) trip_id: String,
    pub(crate) arrival_time: Option<Time>,
    pub(crate) departure_time: Option<Time>,
    pub(crate) stop_id: String,
    pub(crate) stop_sequence: u32,
    pub(crate) stop_headsign: Option<String>,
    pub(crate) pickup_type: Option<u8>,
    pub(crate) drop_off_type: Option<u8>,
    pub(crate) continuous_pickup: Option<u8>,
    pub(crate) continuous_drop_off: Option<u8>,
    #[serde(rename = "shape_dist_travelled")]
    pub(crate) shape_distance_travelled: Option<f32>,
    pub(crate) timepoint: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Calendar {
    #[serde(rename = "service_id")]
    pub(crate) id: String,
    pub(crate) monday: bool,
    pub(crate) tuesday: bool,
    pub(crate) wednesday: bool,
    pub(crate) thursday: bool,
    pub(crate) friday: bool,
    pub(crate) saturday: bool,
    pub(crate) sunday: bool,
    pub(crate) start_date: String,
    pub(crate) end_date: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CalendarDate {
    pub(crate) service_id: String,
    pub(crate) date: String,
    pub(crate) exception_type: u8,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FareAttribute {
    pub(crate) fare_id: String,
    pub(crate) price: f32,
    pub(crate) currency_type: String,
    pub(crate) payment_method: u8,
    //TODO use enum values
    pub(crate) transfers: Option<u8>,
    pub(crate) agency_id: Option<String>,
    pub(crate) transfer_duration: Option<usize>,
}
