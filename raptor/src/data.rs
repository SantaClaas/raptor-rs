use crate::{Route, RoutesData, Stop, StopTime, StopsData, Transfer};
use rusqlite::Connection;
use rusqlite::{Error};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::identity;

use std::mem;

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

pub struct PartialStop {
    id: String,
    transfers_count: usize,
    transfers_index_start: usize,
}

/// A quick type to bundle the return from loading tops
pub struct GetStopsReturn {
    pub transfers: Vec<Transfer>,
    pub stops: Vec<PartialStop>,
    /// The index of the stop representing the stop id in the stops vector
    pub index_by_stop_id: HashMap<String, usize>,
}

pub fn get_stops(connection: &Connection) -> Result<GetStopsReturn, Error> {
    //TODO transfers
    let mut statement = connection.prepare("SELECT id FROM stops;")?;

    let mut rows = statement.query([])?;

    // As we don't know the stop index of every target stop yet, we need to complete transfers later
    let partial_transfers: Vec<(&String, u64)> = Vec::new();
    let mut stops = Vec::new();
    let mut stop_index = 0;
    let mut current_stop_id: Option<String> = None;

    // For reverse lookup of stop indices when assembling route data
    let mut index_by_stop_id = HashMap::new();

    let mut transfers_index_start: usize = 0;
    let mut transfers_count: usize = 0;

    loop {
        match rows.next()? {
            None => {
                // Process last row if there was one (otherwise query result was empty)
                if let Some(last_id) = current_stop_id {
                    // Complete last stop
                    let last_stop = PartialStop {
                        id: last_id.clone(),
                        transfers_count,
                        transfers_index_start,
                    };

                    stops.push(last_stop);
                    index_by_stop_id.insert(last_id, stop_index);

                    // No need to advance stop index or pointers anymore
                }

                break;
            }
            Some(row) => {
                let new_stop_id = row.get::<_, String>("id")?;
                //let transfer_target_id =//TODO
                //let transfer_time=//TODO

                current_stop_id = match current_stop_id {
                    None => Some(new_stop_id),
                    Some(old_stop_id) if old_stop_id != new_stop_id => {
                        // Complete stop
                        let stop = PartialStop {
                            id: old_stop_id.clone(),
                            transfers_count,
                            transfers_index_start,
                        };

                        stops.push(stop);
                        index_by_stop_id.insert(old_stop_id, stop_index);
                        stop_index += 1;

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
        }
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
pub struct GetRoutesReturn {
    trips_by_stops: HashMap<Vec<usize>, Vec<Trip>>,
    trips_count: usize,
    stop_times_count: usize,
    route_stops_count: usize,
}

pub fn get_routes(
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
    loop {
        match rows.next()? {
            None => {
                // Is last row?
                if let Some(last_trip) = current_trip {
                    // Complete last trip
                    trips_count += 1;
                    let stop_sequence = mem::take(&mut current_stop_sequence);
                    route_stops_count += stop_sequence.len();

                    // Add trip to routes but insert it ordered by departure (impl Ord for Trip takes care of that)
                    let trips = trips_by_stops.entry(stop_sequence).or_default();

                    let position = trips.binary_search(&last_trip).unwrap_or_else(identity);
                    trips.insert(position, last_trip);
                }

                break;
            }
            Some(row) => {
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
                        let trips = trips_by_stops.entry(stop_sequence).or_default();

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
        }
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
pub fn assemble_raptor_data(
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
    for (mut stop_indices, trips_ordered) in trips_by_stops.into_iter() {
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
        let route_indices = route_indices_by_stop_index.remove(&stop_index);

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
