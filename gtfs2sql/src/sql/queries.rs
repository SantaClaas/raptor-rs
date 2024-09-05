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

pub(super) const INSERT_CALENDAR_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO calendar
    VALUES (
        :service_id,
        :monday,
        :tuesday,
        :wednesday,
        :thursday,
        :friday,
        :saturday,
        :sunday,
        :start_date,
        :end_date);";

pub(super) const INSERT_CALENDAR_DATE_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO calendar_dates
    VALUES (
        :service_id,
        :date,
        :exception_type);";

pub(super) const INSERT_FARE_ATTRIBUTE_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO fare_attributes
    VALUES (
        :fare_id,
        :price,
        :currency_type,
        :payment_method,
        :transfers,
        :agency_id,
        :transfer_duration);";

pub(super) const INSERT_FARE_LEG_RULE_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO fare_leg_rules
    VALUES (
        :group_id,
        :network_id,
        :from_area_id,
        :to_area_id,
        :from_timeframe_group_id,
        :to_timeframe_group_id,
        :fare_product_id);";

pub(super) const INSERT_FARE_MEDIA_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO fare_media
    VALUES (
        :id,
        :name,
        :type);";

pub(super) const INSERT_FARE_PRODUCT_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO fare_products
    VALUES (
        :id,
        :name,
        :media_id,
        :amount,
        :currency);";
pub(super) const INSERT_FARE_RULE_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO fare_rules
    VALUES (
        :fare_id,
        :route_id,
        :origin_id,
        :destination_id,
        :contains_id);";

pub(super) const INSERT_FARE_TRANSFER_RULE_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO fare_transfer_rules
    VALUES (
        :from_leg_group_id,
        :to_leg_group_id,
        :transfer_count,
        :duration_limit,
        :duration_limit_type,
        :fare_transfer_type,
        :fare_product_id);";

pub(super) const INSERT_FEED_INFO_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO feed_info
    VALUES (
        :publisher_name,
        :publisher_url,
        :language,
        :default_language,
        :start_date,
        :end_date,
        :version,
        :contact_email,
        :contact_url);";

pub(super) const INSERT_FREQUENCY_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO frequencies
    VALUES (
        :trip_id,
        :start_time,
        :end_time,
        :headway_seconds,
        :exact_times);";

pub(super) const INSERT_LEVEL_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO levels
    VALUES (
        :id,
        :index,
        :name);";

pub(super) const INSERT_PATHWAY_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO pathways
    VALUES (
        :id,
        :from_stop_id,
        :to_stop_id,
        :mode,
        :is_bidirectional,
        :length,
        :traversal_time,
        :stair_count,
        :maximum_slope,
        :minimum_width,
        :signposted_as,
        :reversed_signposted_as);";

pub(super) const INSERT_SHAPE_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO shapes
    VALUES (
        :id,
        :point_latitude,
        :point_longitude,
        :point_sequence,
        :distance_traveled);";

pub(super) const INSERT_STOP_AREA_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO stop_areas VALUES (:area_id, :stop_id);";
pub(super) const INSERT_TIMEFRAME_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO timeframes VALUES (:group_id, :start_time, :end_time, :service_id);";
pub(super) const INSERT_TRANSFER_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO transfers
     VALUES (
         :from_stop_id,
         :to_stop_id,
         :from_route_id,
         :to_route_id,
         :from_trip_id,
         :to_trip_id,
         :type,
         :minimum_transfer_time);";

pub(super) const INSERT_TRANSLATION_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO translations
     VALUES (
         :table_nam,
         :field_name,
         :language,
         :translation,
         :record_id,
         :record_sub_id,
         :field_value);";

pub(super) const INSERT_AREA_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO areas VALUES (:id, :name);";

pub(super) const INSERT_ATTRIBUTION_QUERY: &str =
    /*language=sqlite*/
    "INSERT OR IGNORE INTO attributions
    VALUES (
        :id,
        :agency_id,
        :route_id,
        :trip_id,
        :organization_name,
        :is_producer,
        :is_operator,
        :is_authority,
        :url,
        :email,
        :phone);";
