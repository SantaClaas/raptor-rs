mod create_database_query;
mod queries;

use rusqlite::{Connection, Statement};
use crate::sql::create_database_query::CREATE_DATABASE_QUERY;
use crate::sql::queries::*;

struct Database<'conn> {
    connection: &'conn Connection,
    // insert_agency: Statement<'connection>,

}

//
// fn create_database<'conn>() -> Result<(&'conn Connection, &'conn Statement<'conn>), rusqlite::Error>  {
//     let connection = Connection::open("database.db")?;
//
//     &connection.execute(CREATE_DATABASE_QUERY,[])?;
//
//     // Preparing statements now as they will be run a lot of times
//     let insert_agency = { Connection::prepare(&connection, INSERT_AGENCY_QUERY)? };
//     // let insert_route = connection.prepare(INSERT_ROUTE_QUERY)?;
//     // let insert_stop = connection.prepare(INSERT_STOP_QUERY)?;
//     // let insert_stop_time = connection.prepare(INSERT_STOP_TIME_QUERY)?;
//     // let insert_trip_time = connection.prepare(INSERT_TRIP_QUERY)?;
//
//
//
//
//
//     Ok((&connection, &insert_agency))
// }
//
