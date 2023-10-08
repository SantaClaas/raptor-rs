use crate::raptor;
use crate::raptor::{raptor, Route, RoutesData, Stop, StopTime, StopsData, Time, Transfer};
use rusqlite::Error;
use rusqlite::{params, Connection};
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::convert::identity;
use std::hash::{Hash, Hasher};
use std::mem;
use std::time::Duration;

#[deprecated]
pub fn assemble_stops_data(
    connection: &Connection,
) -> Result<
    (
        Vec<String>,
        Vec<Stop>,
        Vec<Transfer>,
        HashMap<String, usize>,
    ),
    Error,
> {
    let mut statement = connection.prepare(
        "SELECT DISTINCT stop_id, route_id
        FROM stop_times
        JOIN trips
            ON stop_times.trip_id = trips.id
        JOIN routes
            ON trips.route_id = routes.id
        ORDER BY stop_id, route_id;",
    )?;
    let mut rows = statement.query([])?;

    let mut route_ids = Vec::new();
    let mut stops: Vec<Stop> = Vec::new();
    // For reverse lookup of stop indices when assembling route data
    let mut index_by_stop_id: HashMap<String, usize> = HashMap::new();

    let mut current_id: Option<String> = None;
    let mut transfers_index_start: usize = 0;
    let mut stop_routes_index_start: usize = 0;
    let mut transfers_count: usize = 0;
    let mut stop_routes_count: usize = 0;

    while let Some(row) = rows.next()? {
        let next_id: String = row.get("stop_id")?;

        current_id = match current_id {
            None => Some(next_id),
            Some(current_id) if current_id != next_id => {
                // Before we move on to the next stop, we complete the previous one
                let stop = Stop {
                    id: current_id.clone(),
                    stop_routes_count,
                    transfers_count,
                    stop_routes_index_start,
                    transfers_index_start,
                };

                let stop_index = stops.len();
                stops.push(stop);
                index_by_stop_id.insert(current_id, stop_index);

                // Advance start pointers
                //TODO support transfers
                transfers_index_start += transfers_count;
                stop_routes_index_start += stop_routes_count;

                // TODO test above does clone and this doesn't change above memory
                // Reset counters
                transfers_count = 0;
                stop_routes_count = 0;
                Some(next_id)
            }
            id => id,
        };

        let route_id: String = row.get("route_id")?;
        route_ids.push(route_id);
        stop_routes_count += 1;
        //TODO support transfers
    }

    //TODO transfers
    let transfers = Vec::new();

    // Then go through all routes in database and their stop times
    // Replace route_id
    Ok((route_ids, stops, transfers, index_by_stop_id))
}

/// Route id, direction and stop time count make a unique route
struct RouteId(String, bool, usize);

#[deprecated]
fn assemble_routes_data(
    connection: &Connection,
    stop_routes_ids: Vec<String>,
    index_by_stop_id: HashMap<String, usize>,
    stops: Vec<Stop>,
) -> Result<(Vec<usize>, Vec<Route>, Vec<StopTime>, Vec<usize>), Error> {
    let mut statement = connection.prepare(
        "SELECT
            trips.route_id,
            trips.direction,
            trips.id as trip_id,

            stop_times.stop_id,
            stop_times.stop_sequence,
            stop_times.arrival_time_seconds,
            stop_times.departure_time_seconds
        FROM trips
        JOIN stop_times
            ON trips.id = stop_times.trip_id
        ORDER BY trips.route_id, trips.id, trips.direction;",
    )?;

    let mut rows = statement.query([])?;

    // Only the partial id as we know the stop count only after iterating the trip
    let mut current_route_id: Option<(String, bool)> = None;
    let mut current_trip_id = None;
    let mut routes = Vec::new();
    let mut route_stops: Vec<usize> = Vec::new();
    // We need to replace the stop_routes route ids with stop_routes indices
    let mut stop_routes_indices: Vec<usize> = Vec::with_capacity(stop_routes_ids.len());

    let mut number_of_trips = 0;
    let mut number_of_stops = 0;
    let mut route_stops_start_index = 0;
    let mut stop_times_start_index = 0;

    while let Some(row) = rows.next()? {
        // Direction is technically optional
        let direction: Option<bool> = row.get("direction")?;
        let next_id: (String, bool) = (row.get("route_id")?, direction.unwrap_or_default());

        current_route_id = match current_route_id {
            None => Some(next_id),
            Some(current_id) if current_id != next_id => {
                // Assemble route before we move on to next route
                let route = Route {
                    number_of_trips,
                    number_of_stops,
                    route_stops_start_index,
                    stop_times_start_index,
                };

                routes.push(route);

                // Advance start pointers
                route_stops_start_index += number_of_stops;
                stop_times_start_index += number_of_trips * number_of_stops;

                // Reset counters
                number_of_trips = 0;
                number_of_stops = 0;

                Some(next_id)
            }
            id => id,
        };

        //TODO maybe load trips ordered by departure ascending and then find routes for trip
        // Iterate stop_times
        let next_trip_id: String = row.get("trip_id")?;
        current_trip_id = match current_trip_id {
            None => Some(next_trip_id),
            Some(current_trip_id) if current_trip_id != next_trip_id => Some(next_trip_id),
            id => id,
        }
    }
    todo!()
}
pub fn get_data(connection: &Connection) -> Result<(), Error> {
    let (route_ids, stops, transfers, index_by_stop_id) = assemble_stops_data(connection)?;
    let (stop_routes, routes, stop_times, route_stops) =
        assemble_routes_data(connection, route_ids, index_by_stop_id, stops)?;
    Ok(())
}

struct Trip {
    id: String,
    stop_times: Vec<StopTime>,
}

impl Eq for Trip {}
// Implement ord for trip to sort them by departure of first stop
impl PartialEq<Self> for Trip {
    fn eq(&self, other: &Self) -> bool {
        match (self.stop_times.first(), other.stop_times.first()) {
            (Some(time), Some(other_time)) => time.departure_time.eq(&other_time.departure_time),
            (Some(_), None) | (None, Some(_)) => false,
            (None, None) => true,
        }
    }
}

impl PartialOrd<Self> for Trip {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.stop_times.first(), other.stop_times.first()) {
            (Some(time), Some(other_time)) => {
                time.departure_time.partial_cmp(&other_time.departure_time)
            }
            (Some(_), None) => Some(Ordering::Greater),
            (None, Some(_)) => Some(Ordering::Less),
            (None, None) => Some(Ordering::Equal),
        }
    }
}

impl Ord for Trip {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.stop_times.first(), other.stop_times.first()) {
            (Some(time), Some(other_time)) => time.departure_time.cmp(&other_time.departure_time),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => Ordering::Equal,
        }
    }
}

struct PartialStop {
    id: String,
    transfers_count: usize,
    transfers_index_start: usize,
}

/// A quick type to bundle the return from loading tops
struct GetStopsReturn {
    transfers: Vec<Transfer>,
    stops: Vec<PartialStop>,
    /// The index of the stop representing the stop id in the stops vector
    index_by_stop_id: HashMap<String, usize>,
}
fn get_stops(connection: &Connection) -> Result<GetStopsReturn, Error> {
    //TODO transfers
    let mut statement = connection.prepare("SELECT id FROM stops;")?;

    let rows = statement.query_map([], |row| row.get::<_, String>("id"))?;

    // As we don't know the stop index of every target stop yet, we need to complete transfers later
    let mut partial_transfers: Vec<(&String, u64)> = Vec::new();
    let mut stops = Vec::new();
    let mut stop_index = 0;
    let mut current_stop_id = None;

    // For reverse lookup of stop indices when assembling route data
    let mut index_by_stop_id = HashMap::new();

    let mut transfers_index_start: usize = 0;
    let mut transfers_count: usize = 0;

    for new_stop_id in rows {
        let new_stop_id = new_stop_id?;
        //let transfer_target_id =//TODO
        //let transfer_time=//TODO

        current_stop_id = match current_stop_id {
            None => Some(new_stop_id),
            Some(stop_id) if stop_id != new_stop_id => {
                // Complete stop
                let stop = PartialStop {
                    id: stop_id.clone(),
                    transfers_count,
                    transfers_index_start,
                };

                stops.push(stop);
                stop_index += 1;
                index_by_stop_id.insert(stop_id, stop_index);

                // Advance start pointers
                //TODO support transfers
                transfers_index_start += transfers_count;

                // Reset counters
                transfers_count = 0;

                Some(new_stop_id)
            }
            id => id,
        };

        // We complete transfers when we have all target stop indices. At this point we only have
        // transfer source stop index
        // partial_transfers.push((transfer_target_id, transfer_time));//TODO
        // transfers_count += 1; //TODO use
    }

    let mut transfers = Vec::with_capacity(partial_transfers.len());
    for (target_stop_id, transfer_time) in partial_transfers.into_iter() {
        // As we iterate all known stops this would only fail if the transfer references a non-existent stop
        let target_index = index_by_stop_id.get(target_stop_id).unwrap();

        transfers.push(Transfer {
            time: transfer_time.into(),
            target: *target_index,
        });
    }

    Ok(GetStopsReturn {
        transfers,
        stops,
        index_by_stop_id,
    })
}

/// Just a quick struct to bundle return values from get_routes
struct GetRoutesReturn {
    trips_by_stops: HashMap<Vec<usize>, Vec<Trip>>,
    trips_count: usize,
    stop_times_count: usize,
    route_stops_count: usize,
}

fn get_routes(
    connection: &Connection,
    index_by_stop_id: HashMap<String, usize>,
) -> Result<GetRoutesReturn, Error> {
    // We determine routes ourself by defining each trip with unique sequence of stops as a route
    let mut statement = connection.prepare(
        // We need the trip id to reconstruct the route and trip although the RAPTOR algorithm
        // does not care about it.
        // I assume stop departure, stop id and trip stop count get very close to uniquely
        // identifying a trip but are not guaranteed to not have collisions so we need to keep
        // track of the trip id.
        // We also need the trip id to group the stop ids as trips
        "SELECT
                trip_id,
                stop_id,
                arrival_time_seconds,
                departure_time_seconds
            FROM stop_times
            ORDER BY trip_id, departure_time_seconds",
    )?;
    let mut rows = statement.query([])?;

    // Trips by stop id sequence
    let mut trips_by_stops: HashMap<Vec<usize>, Vec<Trip>> = HashMap::new();

    let mut current_stop_sequence = Vec::new();
    let mut current_trip: Option<Trip> = None;
    // Counters to know allocation size for final data structure later
    let mut trips_count: usize = 0;
    let mut stop_times_count: usize = 0;
    let mut route_stops_count: usize = 0;
    while let Some(row) = rows.next()? {
        let next_trip_id: String = row.get("trip_id")?;
        let stop_id: String = row.get("stop_id")?;
        // Assume we have all stops that can be referenced or this would reference a non-existent
        // stop which is undefined behavior but would at least make this route unusable for end users
        let stop_index = index_by_stop_id.get(&stop_id).unwrap();
        let stop_time = StopTime {
            arrival_time: row.get::<_, u64>("departure_time_seconds")?.into(),
            departure_time: row.get::<_, u64>("arrival_time_seconds")?.into(),
        };

        current_trip = match current_trip {
            None => Some(Trip {
                id: next_trip_id,
                stop_times: Vec::from([stop_time]),
            }),
            Some(completed_trip) if completed_trip.id != next_trip_id => {
                // Complete current trip
                trips_count += 1;
                let stop_sequence = mem::take(&mut current_stop_sequence);
                route_stops_count += stop_sequence.len();

                // Add trip to routes but insert it ordered by departure (impl Ord for Trip takes care of that)
                let mut trips = trips_by_stops.entry(stop_sequence).or_default();

                // Trips that depart at the same time and have the same sequence of stops can be a
                // valid option for the user to choose from as the user might consider factors
                // unknown to us. Although this is very unlikely it is not impossible.
                // There could also be trips with the same departure time and sequence of stops
                // where one trip might arrive earlier because the train or bus is faster. (This too
                // seems unrealistic but is theoretically not impossible)
                // So get the position where it already exists or gets the position where it should
                // be inserted
                let position = trips
                    .binary_search(&completed_trip)
                    .unwrap_or_else(identity);
                trips.insert(position, completed_trip);

                // Continue with new trip moving forward
                Some(Trip {
                    id: next_trip_id,
                    stop_times: Vec::from([stop_time]),
                })
            }
            // Here we are still on the same trip
            Some(mut trip) => {
                trip.stop_times.push(stop_time);
                Some(trip)
            }
        };

        stop_times_count += 1;
        current_stop_sequence.push(*stop_index);
    }

    Ok(GetRoutesReturn {
        trips_by_stops,
        trips_count,
        stop_times_count,
        route_stops_count,
    })
}

/// Assembles the data from the previous two steps of getting stops and route data into the final
/// structs required by the RAPTOR algorithm
///
/// # Arguments
///
/// * `GetRoutesReturn {trips_by_stops, trips_count, stop_times_count, route_stops_count}`:
/// * `partial_stops`:
/// * `transfers`:
///
/// returns: (RoutesData, StopsData, Vec<String, Global>)
///
/// # Examples
///
/// ```
///
/// ```
fn assemble_raptor_data(
    GetRoutesReturn {
        trips_by_stops,
        trips_count,
        stop_times_count,
        route_stops_count,
    }: GetRoutesReturn,
    partial_stops: Vec<PartialStop>,
    transfers: Vec<Transfer>,
) -> (RoutesData, StopsData, Vec<String>) {
    // Final assembly RoutesData

    // Trip ids where the index refers to the number of the block that represents a trip in
    // stop_times. Not relevant for RAPTOR but needed to reconstruct journey
    let mut trip_ids: Vec<String> = Vec::with_capacity(trips_count);

    // Arrays as described in RAPTOR paper Appendix A Data Structures
    let mut stop_times: Vec<StopTime> = Vec::with_capacity(stop_times_count);
    let mut routes: Vec<Route> = Vec::with_capacity(trips_by_stops.len());
    // Route stops contains not the stops but the indices of stops in the stops data structure
    let mut route_stops: Vec<usize> = Vec::with_capacity(route_stops_count);

    // Pointers to the start of each route segment
    let mut route_stops_start_index = 0;
    let mut stop_times_start_index = 0;

    // For later final assembly StopsData
    let mut route_index = 0;
    let mut route_indices_by_stop_index: HashMap<usize, Vec<usize>> = HashMap::new();
    // To know allocation size later
    let mut stop_routes_count = 0;

    // Go through each route
    for (mut stop_indices, mut trips_ordered) in trips_by_stops.into_iter() {
        let number_of_stops = stop_indices.len();

        let length = stop_indices.len();
        stop_routes_count += length;
        // Need to find out what routes arrive at what stop later for StopsData
        for index in 0..length {
            let stop_index = stop_indices[index];
            // Add for StopsData construction later
            let routes = route_indices_by_stop_index
                .entry(stop_index.clone())
                .or_default();
            routes.push(route_index);
        }

        // Route Stops
        route_stops.append(&mut stop_indices);

        let number_of_trips = trips_ordered.len();
        // Stop Times
        for Trip {
            stop_times: mut trip_stop_times,
            id,
        } in trips_ordered.into_iter()
        {
            stop_times.append(&mut trip_stop_times);

            trip_ids.push(id);
        }

        // Complete route
        routes.push(Route {
            number_of_trips,
            number_of_stops,
            route_stops_start_index,
            stop_times_start_index,
        });
        route_index += 1;

        // Advance pointers
        route_stops_start_index += number_of_stops;
        stop_times_start_index += number_of_trips * number_of_stops;
    }

    let routes_data = RoutesData {
        stop_times,
        routes,
        route_stops,
    };

    // Final assembly StopsData
    let mut stop_routes = Vec::with_capacity(stop_routes_count);
    let stops_length = partial_stops.len();
    let mut stops = Vec::with_capacity(stops_length);
    let mut stop_index: usize = 0;

    let mut stop_routes_index_start = 0;

    for PartialStop {
        id,
        transfers_count,
        transfers_index_start,
    } in partial_stops.into_iter()
    {
        // There indeed exist stops where no one stops in GTFS
        // Maybe they are stations that group stops but the ones I have encountered so far aren't
        let mut route_indices = route_indices_by_stop_index.remove(&stop_index);

        let stop_routes_count = match route_indices {
            Some(mut route_indices) => {
                let count = route_indices.len();
                stop_routes.append(&mut route_indices);
                count
            }
            None => 0,
        };

        // Complete stop
        stops.push(Stop {
            id,
            transfers_index_start,
            stop_routes_index_start,
            transfers_count,
            stop_routes_count,
        });

        stop_index += 1;
        // Advance pointer
        stop_routes_index_start += stop_routes_count;
    }

    let stops_data = StopsData {
        transfers,
        stops,
        stop_routes,
    };

    (routes_data, stops_data, trip_ids)
}
#[test]
fn how_fast() {
    let connection = Connection::open("database.db").unwrap();

    let GetStopsReturn {
        transfers,
        stops,
        index_by_stop_id,
    } = get_stops(&connection).unwrap();

    //TODO remove debug clone clown
    let step_2_result = get_routes(&connection, index_by_stop_id.clone()).unwrap();

    let (routes_data, stops_data, trip_ids) = assemble_raptor_data(step_2_result, stops, transfers);

    let dream_source_stop_id = "1808";
    let dream_target_stop_id = "1811";
    let source_index = index_by_stop_id.get(dream_source_stop_id).unwrap();
    let target_index = index_by_stop_id.get(dream_target_stop_id).unwrap();
    let departure = Time::from(12 * 60 * 60);

    //TODO remove clown copy
    let stops = stops_data.stops. clone();
    let results = raptor(
        *source_index,
        *target_index,
        &departure,
        routes_data,
        stops_data,
    );

    let mut statement = connection
        .prepare("SELECT name FROM stops WHERE id = :id")
        .unwrap();

    let mut round = 1;
    for result in results {
        println!("Round {round} visited...");
        for (stop_index, _) in result {
            let stop_id = &stops[stop_index].id;
            let stop_name = statement.query_row(params![":id", stop_id], |row| row.get::<_, String>("name")).unwrap();

            print!("{stop_name} ")
        }
        println!();

        round += 1;
    }
}
