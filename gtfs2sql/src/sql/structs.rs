use crate::sql::time::Time;
use serde::Deserialize;

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
    #[serde(rename = "route_type")]
    pub(crate) r#type: u8,
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
    pub(crate) service_id: String,
    pub(crate) monday: u8,
    pub(crate) tuesday: u8,
    pub(crate) wednesday: u8,
    pub(crate) thursday: u8,
    pub(crate) friday: u8,
    pub(crate) saturday: u8,
    pub(crate) sunday: u8,
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

#[derive(Debug, Deserialize)]
pub(crate) struct FareRule {
    pub(crate) fare_id: String,
    pub(crate) route_id: Option<String>,
    pub(crate) origin_id: Option<String>,
    pub(crate) destination_id: Option<String>,
    pub(crate) contains_id: Option<String>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct Timeframe {
    #[serde(rename = "timeframe_group_id")]
    pub(crate) group_id: String,
    pub(crate) start_time: Option<Time>,
    pub(crate) end_time: Option<Time>,
    pub(crate) service_id: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FareMedia {
    #[serde(rename = "fare_media_id")]
    pub(crate) id: String,
    #[serde(rename = "fare_media_name")]
    pub(crate) name: Option<String>,
    #[serde(rename = "fare_media_type")]
    pub(crate) r#type: u8,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FareProduct {
    #[serde(rename = "fare_product_id")]
    pub(crate) id: String,
    #[serde(rename = "fare_product_name")]
    pub(crate) name: Option<String>,
    pub(crate) media_id: Option<String>,
    pub(crate) amount: f64,
    pub(crate) currency: String,
}
#[derive(Debug, Deserialize)]
pub(crate) struct FareLegRule {
    #[serde(rename = "leg_group_id")]
    pub(crate) group_id: Option<String>,
    #[serde(rename = "fare_product_name")]
    pub(crate) network_id: Option<String>,
    pub(crate) from_area_id: Option<String>,
    pub(crate) to_area_id: Option<String>,
    pub(crate) from_timeframe_group_id: Option<String>,
    pub(crate) to_timeframe_group_id: Option<String>,
    pub(crate) fare_product_id: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FareTransferRule {
    pub(crate) from_leg_group_id: Option<String>,
    pub(crate) to_leg_group_id: Option<String>,
    pub(crate) transfer_count: Option<usize>,
    pub(crate) duration_limit: Option<usize>,
    pub(crate) duration_limit_type: Option<u8>,
    pub(crate) fare_transfer_type: u8,
    pub(crate) fare_product_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Area {
    #[serde(rename = "area_id")]
    pub(crate) id: String,
    #[serde(rename = "area_name")]
    pub(crate) name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct StopArea {
    pub(crate) area_id: String,
    pub(crate) stop_id: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Shape {
    #[serde(rename = "shape_id")]
    pub(crate) id: String,
    #[serde(rename = "shape_pt_lat")]
    pub(crate) point_latitude: f32,
    #[serde(rename = "shape_pt_lon")]
    pub(crate) point_longitude: f32,
    #[serde(rename = "shape_pt_sequence")]
    pub(crate) point_sequence: usize,
    #[serde(rename = "shape_dist_traveled")]
    pub(crate) distance_traveled: Option<usize>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct Frequency {
    pub(crate) trip_id: String,
    pub(crate) start_time: Time,
    pub(crate) end_time: Time,
    #[serde(rename = "headway_secs")]
    pub(crate) headway_seconds: usize,
    pub(crate) exact_times: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Transfer {
    pub(crate) from_stop_id: Option<String>,
    pub(crate) to_stop_id: Option<String>,
    pub(crate) from_route_id: Option<String>,
    pub(crate) to_route_id: Option<String>,
    pub(crate) from_trip_id: Option<String>,
    pub(crate) to_trip_id: Option<String>,
    #[serde(rename = "transfer_type")]
    pub(crate) r#type: u8,
    #[serde(rename = "min_transfer_time")]
    pub(crate) minimum_transfer_time: Option<usize>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct Pathway {
    #[serde(rename = "pathway_id")]
    pub(crate) id: String,
    pub(crate) from_stop_id: String,
    pub(crate) to_stop_id: String,

    #[serde(rename = "pathway_mode")]
    pub(crate) mode: u8,
    pub(crate) is_bidirectional: u8,
    pub(crate) length: Option<usize>,
    pub(crate) traversal_time: Option<usize>,
    pub(crate) stair_count: Option<usize>,
    #[serde(rename = "max_slope")]
    pub(crate) maximum_slope: Option<f32>,
    #[serde(rename = "min_width")]
    pub(crate) minimum_width: Option<f32>,
    pub(crate) signposted_as: Option<String>,
    pub(crate) reversed_signposted_as: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Level {
    #[serde(rename = "level_id")]
    pub(crate) id: String,
    #[serde(rename = "level_index")]
    pub(crate) index: f32,
    #[serde(rename = "level_name")]
    pub(crate) name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Translation {
    pub(crate) table_name: String,
    pub(crate) field_name: String,
    pub(crate) language: String,
    pub(crate) translation: String,
    pub(crate) record_id: Option<String>,
    pub(crate) record_sub_id: Option<String>,
    pub(crate) field_value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FeedInfo {
    #[serde(rename = "feed_publisher_name")]
    pub(crate) publisher_name: String,
    #[serde(rename = "feed_publisher_url")]
    pub(crate) publisher_url: String,
    #[serde(rename = "feed_lang")]
    pub(crate) language: String,
    #[serde(rename = "default_lang")]
    pub(crate) default_language: Option<String>,
    #[serde(rename = "feed_start_date")]
    pub(crate) start_date: Option<String>,
    #[serde(rename = "feed_end_date")]
    pub(crate) end_date: Option<String>,
    #[serde(rename = "feed_version")]
    pub(crate) version: Option<String>,
    #[serde(rename = "feed_contact_email")]
    pub(crate) contact_email: Option<String>,
    #[serde(rename = "feed_contact_url")]
    pub(crate) contact_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Attribution {
    #[serde(rename = "attribution_id")]
    pub(crate) id: Option<String>,
    pub(crate) agency_id: Option<String>,
    pub(crate) route_id: Option<String>,
    pub(crate) trip_id: Option<String>,
    pub(crate) organization_name: String,
    pub(crate) is_producer: Option<u8>,
    pub(crate) is_operator: Option<u8>,
    pub(crate) is_authority: Option<u8>,
    #[serde(rename = "attribution_url")]
    pub(crate) url: Option<String>,
    #[serde(rename = "attribution_email")]
    pub(crate) email: Option<String>,
    #[serde(rename = "attribution_phone")]
    pub(crate) phone: Option<String>,
}
