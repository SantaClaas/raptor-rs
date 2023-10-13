pub(crate) struct Agency {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) timezone: String,
    pub(crate) language: Option<String>,
    pub(crate) phone: Option<String>,
    pub(crate) fare_url: Option<String>,
    pub(crate) email: Option<String>,
}

pub(crate) struct Route {
    pub(crate) id: String,
    pub(crate) agency_id: Option<String>,
    pub(crate) short_name: Option<String>,
    pub(crate) long_name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) route_type: u8,
    pub(crate) url: Option<String>,
    pub(crate) color: Option<String>,
    pub(crate) text_color: Option<String>,
    pub(crate) sort_order: Option<u32>,
    pub(crate) continuous_pickup: Option<u8>,
    pub(crate) continuous_drop_off: Option<u8>,
    pub(crate) network_id: Option<String>,
}

pub(crate) struct Stop {
    pub(crate) id: String,
    pub(crate) code: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) text_to_speech_name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) latitude: Option<String>,
    pub(crate) longitude: Option<String>,
    pub(crate) zone_id: Option<String>,
    pub(crate) url: Option<String>,
    pub(crate) location_type: Option<u8>,
    pub(crate) parent_station: Option<String>,
    pub(crate) timezone: Option<String>,
    pub(crate) wheelchair_boarding: Option<u8>,
    pub(crate) level_id: Option<String>,
    pub(crate) platform_code: Option<String>,
}

pub(crate) struct StopTime {
    pub(crate) trip_id: String,
    pub(crate) arrival_time: Option<String>,
    pub(crate) arrival_time_seconds: Option<u64>,
    pub(crate) departure_time: Option<String>,
    pub(crate) departure_time_seconds: Option<u64>,
    pub(crate) stop_id: String,
    pub(crate) stop_sequence: u32,
    pub(crate) stop_headsign: Option<String>,
    pub(crate) pickup_type: Option<u8>,
    pub(crate) drop_off_type: Option<u8>,
    pub(crate) continuous_pickup: Option<u8>,
    pub(crate) continuous_drop_off: Option<u8>,
    pub(crate) shape_distance_travelled: Option<f32>,
    pub(crate) timepoint: Option<u8>,
}

pub(crate) struct Trip {
    pub(crate) route_id: String,
    pub(crate) service_id: String,
    pub(crate) id: String,
    pub(crate) headsign: Option<String>,
    pub(crate) short_name: Option<String>,
    pub(crate) direction_id: Option<u8>,
    pub(crate) block_id: Option<String>,
    pub(crate) shape_id: Option<String>,
    pub(crate) wheelchair_accessible: Option<u8>,
    pub(crate) bikes_allowed: Option<u8>,
}
