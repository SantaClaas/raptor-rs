pub(super) const INSERT_AGENCY_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO agencies
    VALUES (
      :id,
      :name,
      :url,
      :timezone,
      :language,
      :phone,
      :fare_url,
      :email);";

pub(super) const INSERT_ROUTE_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO routes
    VALUES (
        :id,
        :agency_id,
        :short_name,
        :long_name,
        :description,
        :type,
        :url,
        :color,
        :text_color,
        :sort_order,
        :continuous_pickup,
        :continuous_drop_off,
        :network_id);";

pub(super) const INSERT_STOP_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO stops
    VALUES (
        :id,
        :code,
        :name,
        :text_to_speech_name,
        :description,
        :latitude,
        :longitude,
        :zone_id,
        :url,
        :location_type,
        :parent_station,
        :timezone,
        :wheelchair_boarding,
        :level_id,
        :platform_code);";

pub(super) const INSERT_STOP_TIME_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO stop_times
    VALUES (
        :trip_id,
        :arrival_time,
        :arrival_time_seconds,
        :departure_time,
        :departure_time_seconds,
        :stop_id,
        :stop_sequence,
        :stop_headsign,
        :pickup_type,
        :drop_off_type,
        :continuous_pickup,
        :continuous_drop_off,
        :shape_distance_traveled,
        :timepoint);";

pub(super) const INSERT_TRIP_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO trips
    VALUES (
        :id,
        :route_id,
        :service_id,
        :headsign,
        :short_name,
        :direction,
        :block_id,
        :shape_id,
        :wheelchair_accessible,
        :bikes_allowed);";
