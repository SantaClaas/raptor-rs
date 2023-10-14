mod create_tables_queries;
mod queries;
mod structs;

mod time;

use crate::sql::create_tables_queries::{CREATE_AGENCIES_QUERY, CREATE_AREAS_QUERY, CREATE_ATTRIBUTIONS_QUERY, CREATE_CALENDAR_DATES_QUERY, CREATE_CALENDAR_QUERY, CREATE_DATABASE_QUERY, CREATE_FARE_ATTRIBUTES_QUERY, CREATE_FARE_LEG_RULES_QUERY, CREATE_FARE_MEDIA_QUERY, CREATE_FARE_PRODUCTS_QUERY, CREATE_FARE_RULES_QUERY, CREATE_FARE_TRANSFER_RULES_QUERY, CREATE_FEED_INFO_QUERY, CREATE_FREQUENCIES_QUERY, CREATE_LEVELS_QUERY, CREATE_PATHWAYS_QUERY, CREATE_ROUTES_QUERY, CREATE_SHAPES_QUERY, CREATE_STOP_AREAS_QUERY, CREATE_STOP_TIMES_QUERY, CREATE_STOPS_QUERY, CREATE_TIMEFRAMES_QUERY, CREATE_TRANSFERS_QUERY, CREATE_TRANSLATIONS_QUERY, CREATE_TRIPS_QUERY};
use crate::sql::queries::*;
pub(crate) use crate::sql::structs::{Agency, Route, Stop, StopTime, Trip};
pub(crate) use crate::sql::time::Time;
use rusqlite::types::ToSqlOutput;
use rusqlite::{named_params, Connection, ToSql};

pub(crate) fn create_database() -> Result<Connection, rusqlite::Error> {
    let connection = Connection::open("gtfs.db")?;

    &connection.execute(CREATE_AGENCIES_QUERY, [])?;
    &connection.execute(CREATE_STOPS_QUERY, [])?;
    &connection.execute(CREATE_ROUTES_QUERY, [])?;
    &connection.execute(CREATE_TRIPS_QUERY, [])?;
    &connection.execute(CREATE_STOP_TIMES_QUERY, [])?;

    &connection.execute(CREATE_CALENDAR_QUERY, [])?;
    &connection.execute(CREATE_CALENDAR_DATES_QUERY, [])?;

    &connection.execute(CREATE_FARE_ATTRIBUTES_QUERY, [])?;
    &connection.execute(CREATE_FARE_RULES_QUERY, [])?;
    &connection.execute(CREATE_TIMEFRAMES_QUERY, [])?;
    &connection.execute(CREATE_FARE_MEDIA_QUERY, [])?;
    &connection.execute(CREATE_FARE_PRODUCTS_QUERY, [])?;
    &connection.execute(CREATE_FARE_LEG_RULES_QUERY, [])?;
    &connection.execute(CREATE_FARE_TRANSFER_RULES_QUERY, [])?;
    &connection.execute(CREATE_AREAS_QUERY, [])?;
    &connection.execute(CREATE_STOP_AREAS_QUERY, [])?;
    &connection.execute(CREATE_SHAPES_QUERY, [])?;
    &connection.execute(CREATE_FREQUENCIES_QUERY, [])?;
    &connection.execute(CREATE_TRANSFERS_QUERY, [])?;
    &connection.execute(CREATE_PATHWAYS_QUERY, [])?;
    &connection.execute(CREATE_LEVELS_QUERY, [])?;
    &connection.execute(CREATE_TRANSLATIONS_QUERY, [])?;
    &connection.execute(CREATE_FEED_INFO_QUERY, [])?;
    &connection.execute(CREATE_ATTRIBUTIONS_QUERY, [])?;


    // Preparing statements now as they will be run a lot of times
    connection.prepare_cached(INSERT_AGENCY_QUERY)?;
    connection.prepare_cached(INSERT_ROUTE_QUERY)?;
    connection.prepare_cached(INSERT_STOP_QUERY)?;
    connection.prepare_cached(INSERT_STOP_TIME_QUERY)?;
    connection.prepare_cached(INSERT_TRIP_QUERY)?;

    Ok(connection)
}

pub(crate) trait Insert<T> {
    fn insert(&self, item: T) -> rusqlite::Result<()>;
}

impl Insert<Agency> for Connection {
    fn insert(&self, agency: Agency) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_AGENCY_QUERY)?;

        statement.execute(named_params! {
            ":id": agency.id,
            ":name": agency.name,
            ":url": agency.url,
            ":timezone": agency.timezone,
            ":language": agency.language,
            ":phone": agency.phone,
            ":fare_url": agency.fare_url,
            ":email": agency.email
        })?;

        Ok(())
    }
}

impl Insert<Route> for Connection {
    fn insert(&self, route: Route) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_ROUTE_QUERY)?;

        statement.execute(named_params! {
           ":id": route.id,
           ":agency_id": route.agency_id,
           ":short_name": route.short_name,
           ":long_name": route.long_name,
           ":description": route.description,
           ":type": route.route_type,
           ":url": route.url,
           ":color": route.color,
           ":text_color": route.text_color,
           ":sort_order": route.sort_order,
           ":continuous_pickup": route.continuous_pickup,
           ":continuous_drop_off": route.continuous_drop_off,
           ":network_id": route.network_id,
        })?;

        Ok(())
    }
}

impl Insert<Stop> for Connection {
    fn insert(&self, stop: Stop) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_STOP_QUERY)?;

        statement.execute(named_params! {
            ":id": stop.id,
            ":code": stop.code,
            ":name": stop.name,
            ":text_to_speech_name": stop.text_to_speech_name,
            ":description": stop.description,
            ":latitude": stop.latitude,
            ":longitude": stop.longitude,
            ":zone_id": stop.zone_id,
            ":url": stop.url,
            ":location_type": stop.location_type,
            ":parent_station": stop.parent_station,
            ":timezone": stop.timezone,
            ":wheelchair_boarding": stop.wheelchair_boarding,
            ":level_id": stop.level_id,
            ":platform_code": stop.platform_code,
        })?;

        Ok(())
    }
}

impl ToSql for Time {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}
impl Insert<StopTime> for Connection {
    fn insert(&self, stop_time: StopTime) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_STOP_TIME_QUERY)?;

        statement.execute(named_params! {
            ":trip_id": stop_time.trip_id,
            ":arrival_time": stop_time.arrival_time,
            ":arrival_time_seconds": stop_time.arrival_time.map(Time::total_seconds),
            ":departure_time": stop_time.departure_time,
            ":departure_time_seconds": stop_time.departure_time.map(Time::total_seconds),
            ":stop_id": stop_time.stop_id,
            ":stop_sequence": stop_time.stop_sequence,
            ":stop_headsign": stop_time.stop_headsign,
            ":pickup_type": stop_time.pickup_type,
            ":drop_off_type": stop_time.drop_off_type,
            ":continuous_pickup": stop_time.continuous_pickup,
            ":continuous_drop_off": stop_time.continuous_drop_off,
            ":shape_distance_traveled": stop_time.shape_distance_travelled,
            ":timepoint": stop_time.timepoint,
        })?;

        Ok(())
    }
}

impl Insert<Trip> for Connection {
    fn insert(&self, trip: Trip) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_TRIP_QUERY)?;

        statement.execute(named_params! {
            ":id": trip.id,
            ":route_id": trip.route_id,
            ":service_id": trip.service_id,
            ":headsign": trip.headsign,
            ":short_name": trip.short_name,
            ":direction": trip.direction_id,
            ":block_id": trip.block_id,
            ":shape_id": trip.shape_id,
            ":wheelchair_accessible": trip.wheelchair_accessible,
            ":bikes_allowed": trip.bikes_allowed,
        })?;

        Ok(())
    }
}
