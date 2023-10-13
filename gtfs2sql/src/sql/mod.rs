mod create_database_query;
mod queries;
mod structs;

use crate::sql::create_database_query::CREATE_DATABASE_QUERY;
use crate::sql::queries::*;
use crate::sql::structs::{Stop, StopTime, Trip};
pub(crate) use crate::sql::structs::{Agency, Route};
use rusqlite::{named_params, Connection};

pub(crate) fn create_database() -> Result<Connection, rusqlite::Error> {
    let connection = Connection::open("database.db")?;

    &connection.execute(CREATE_DATABASE_QUERY, [])?;

    // Preparing statements now as they will be run a lot of times
    connection.prepare_cached(INSERT_AGENCY_QUERY)?;
    connection.prepare_cached(INSERT_ROUTE_QUERY)?;
    connection.prepare_cached(INSERT_STOP_QUERY)?;
    connection.prepare_cached(INSERT_STOP_TIME_QUERY)?;
    connection.prepare_cached(INSERT_TRIP_QUERY)?;

    Ok(connection)
}

pub(crate) fn insert_agency(connection: &Connection, agency: Agency) -> Result<(), rusqlite::Error> {
    let mut statement = connection.prepare_cached(INSERT_AGENCY_QUERY)?;

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

pub(crate) fn insert_route(connection: &Connection, route: Route) -> Result<(), rusqlite::Error> {
    let mut statement = connection.prepare_cached(INSERT_ROUTE_QUERY)?;

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

fn insert_stop(connection: &Connection, stop: Stop) -> Result<(), rusqlite::Error> {
    let mut statement = connection.prepare_cached(INSERT_STOP_QUERY)?;

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

fn insert_stop_time(connection: &Connection, stop_time: StopTime) -> Result<(), rusqlite::Error> {
    let mut statement = connection.prepare_cached(INSERT_STOP_TIME_QUERY)?;

    statement.execute(named_params! {
        ":trip_id": stop_time.trip_id,
        ":arrival_time": stop_time.arrival_time,
        ":arrival_time_seconds": stop_time.arrival_time_seconds,
        ":departure_time": stop_time.departure_time,
        ":departure_time_seconds": stop_time.departure_time_seconds,
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

fn insert_trip(connection: &Connection, trip: Trip) -> Result<(), rusqlite::Error> {
    let mut statement = connection.prepare_cached(INSERT_TRIP_QUERY)?;

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



