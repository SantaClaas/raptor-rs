extern crate core;

use raptor::data::{
    assemble_raptor_data, get_routes, get_stops, GetRoutesReturn, GetStopsReturn, PartialStop,
};
use raptor::{raptor, RoutesData, Stop, StopsData, Time, Transfer};
use rusqlite::{named_params, Connection, Result, Statement};

fn main() {
    let connection = Connection::open("database.db").unwrap();

    let GetStopsReturn {
        transfers,
        stops,
        index_by_stop_id,
    } = get_stops(&connection).unwrap();

    let step_2_result = get_routes(
        &connection,
        //TODO remove debug clone clown
        index_by_stop_id.clone(),
    )
    .unwrap();

    let (routes_data, stops_data, trip_ids) = assemble_raptor_data(step_2_result, stops, transfers);

    let dream_source_stop_id = "1808";
    let dream_target_stop_id = "1811";
    let source_index = *index_by_stop_id.get(dream_source_stop_id).unwrap();
    let target_index = *index_by_stop_id.get(dream_target_stop_id).unwrap();
    let departure = Time::from(12 * 60 * 60);

    //TODO remove clown copy
    let stops = stops_data.stops.clone();
    let results = raptor(
        source_index,
        target_index,
        &departure,
        routes_data,
        stops_data,
    );

    let mut statement = connection
        .prepare("SELECT name FROM stops WHERE id = :id")
        .unwrap();

    let mut get_stop_name = |stop_index: usize| -> String {
        let stop_id = &stops[stop_index].id;
        statement
            .query_row(named_params! {":id": stop_id}, |row| {
                row.get::<_, String>("name")
            })
            .unwrap()
    };

    let from = get_stop_name(source_index);
    let to = get_stop_name(target_index);
    println!("From {from} to {to}");

    let mut round = 1;
    for result in results {
        println!("Round {round} reached stops...");
        for (stop_index, connection) in result {
            let stop_name = get_stop_name(stop_index);

            print!("\t{stop_name}:\t");
            match connection {
                raptor::Connection::Connection {
                    route,
                    trip_number,
                    boarded_at_stop,
                    exited_at_stop,
                } => {
                    let boarded = get_stop_name(boarded_at_stop);
                    let exited = get_stop_name(exited_at_stop);
                    println!("Route {route} Trip {trip_number} Connection {boarded} -> {exited}");
                }
                raptor::Connection::FootPath { .. } => {}
            }
        }
        println!();

        round += 1;
    }
}
