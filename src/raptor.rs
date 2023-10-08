use crate::raptor::Time::{Finite, Infinite};
use std::cmp::{min, Ordering};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Add;

/// Represents a time stamp for various structures in RAPTOR.
/// The value represents a time after midnight for a day. It can be greater than 24h if a stop on a
/// trip is reached the next day after midnight
#[derive(Copy, Clone, Debug)]
pub(crate) enum Time {
    Finite(u64),
    Infinite,
}

impl Eq for Time {}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Infinite, Infinite) => Ordering::Equal,
            (Infinite, Finite(_)) => Ordering::Greater,
            (Finite(_), Infinite) => Ordering::Less,
            (Finite(value), Finite(other)) => value.cmp(other),
        }
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Infinite, Infinite) => true,
            (Infinite, Finite(_)) | (Finite(_), Infinite) => false,
            (Finite(value), Finite(other)) => value.eq(other),
        }
    }
}

impl Add for Time {
    type Output = Time;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Infinite, _) => Infinite,
            (_, Infinite) => Infinite,
            (Finite(self_value), Finite(other_value)) => {
                Finite(self_value + other_value)
            }
        }
    }
}

impl From<u64> for Time {
    fn from(value: u64) -> Self {
        Finite(value)
    }
}

impl Display for Time {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Infinite => write!(formatter, "Time Infinite"),
            Finite(seconds) => {
                // Idk how to do basic math
                let (seconds, minutes) = (seconds % 60, seconds / 60);
                let (seconds, minutes, hours) = (seconds, minutes % 60, minutes / 60 );
                write!(formatter, "Time {hours}:{minutes}:{seconds}")
            }
        }
    }
}

#[test]
fn format_time() {
    // Arrange
    // hours > 24 is valid GTFS time to signify trips that extend to the next day
    let hours = 69;
    let minutes = 4;
    let seconds = 20;

    let hours_in_seconds = hours * 60 * 60;
    let minutes_in_seconds = minutes * 60;

    let time = Time::Finite(hours_in_seconds + minutes_in_seconds + seconds);
    let expected = format!("Time {hours}:{minutes}:{seconds}");

    // Act
    let actual = format!("{}", time);
    // Assert
    assert_eq!(expected, actual);
}

/// The departure and arrival time of a trip at a stop
pub(crate) struct StopTime {
    pub(crate) departure_time: Time,
    pub(crate) arrival_time: Time,
}

/// A route or line in a transportation network. A route has multiple trips a day.
/// In contrast to GTFS data a route has always the same sequence of stops in its trips.
/// This means there is a separate route for every trip in GTFS where the sequence of stops or
/// direction is not the same
pub(crate) struct Route {
    /// Number of trips in a route. You can get the length of the block in StopTimes that represent
    /// all trips of this route by multiplying this with number_of_stops
    pub(crate) number_of_trips: usize,

    /// The number of stops per trip of this route. The number of trips is the same per trip.
    pub(crate) number_of_stops: usize,

    /// Pointer to the index that starts the block in the RouteStops array for the stops of this route
    pub(crate) route_stops_start_index: usize,

    /// Pointer to the index that starts the first block of StopTimes for the first trip
    pub(crate) stop_times_start_index: usize,
}

struct RoutesData {
    /// This array is divided into blocks, and the i-th block contains all trips corresponding
    /// to route ri. Within a block, trips are sorted by departure time (at the first stop).
    /// Each trip is just a sequence of stop times, represented by the corresponding arrival
    /// and departure times.
    stop_times: Vec<StopTime>,
    routes: Vec<Route>,
    /// The stops for routes where segments represent stops sequence for routes
    /// The first entries belong to routes[0] then the next to route[1] and so on...
    route_stops: Vec<usize>,
}

impl RoutesData {
    fn get_stop_times(&self, route: &Route) -> &[StopTime] {
        let start = route.stop_times_start_index;
        let end = start + (route.number_of_trips * route.number_of_stops);
        &self.stop_times[start..end]
    }

    fn get_route_stops(&self, route: &Route) -> &[usize] {
        let start = route.route_stops_start_index;
        let end = start + route.number_of_stops;
        &self.route_stops[start..end]
    }

    /// Get the sequence for a stop on the given route
    /// Returns none if the stop is not on the route otherwise the sequence index of the stop on the
    /// route
    fn get_stop_sequence(&self, route: &Route, stop: &usize) -> Option<usize> {
        let route_stops = self.get_route_stops(route);
        route_stops
            .iter()
            .position(|route_stop| &route_stop == &stop)
    }

    /// Get the earliest trip departing from a stop along the route after some time
    /// returns the number of the trip in the route (index in sequence of trips for route) and the
    /// trip stop times
    fn get_earliest_departing_trip(
        &self,
        route: &Route,
        // The sequence of the stop on the route for which the next trip departing should be found
        from_stop_sequence: &usize,
        after: &Time,
    ) -> Option<(usize, &[StopTime])> {
        // Assume we get have the stop_sequence
        let stop_times = self.get_stop_times(route);
        for trip_index in 0..route.number_of_trips {
            let trip_start = trip_index * route.number_of_stops;
            let stop_time = &stop_times[trip_start + from_stop_sequence];
            if &stop_time.departure_time > after {
                let trip_end = trip_start + route.number_of_trips;
                let trip = &stop_times[trip_start..trip_end];
                return Some((trip_index, trip));
            }
        }

        return None;
    }
}

pub(crate) struct Stop {
    pub(crate) id: String,
    pub(crate) transfers_index_start: usize,
    pub(crate) stop_routes_index_start: usize,
    pub(crate) transfers_count: usize,
    pub(crate) stop_routes_count: usize,
}

impl Hash for Stop {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialEq<Self> for Stop {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for Stop {}

/// A transfer that leaves a stop and allows reaching another stop by foot path
pub(crate) struct Transfer {
    /// The target stop that can be reached by foot through this foot-path
    pub(crate) target: usize,
    /// Time it takes to reach the target stop by foot
    pub(crate) time: Time,
}

struct StopsData {
    transfers: Vec<Transfer>,
    stops: Vec<Stop>,
    /// Not the routes themselves but the indices of in the route data
    stop_routes: Vec<usize>,
}

impl StopsData {
    fn get_routes_for(&self, stop: &Stop) -> &[usize] {
        let start = stop.stop_routes_index_start;
        let end = start + stop.stop_routes_count;
        &self.stop_routes[start..end]
    }

    fn get_routes(&self, stop: &usize) -> &[usize] {
        let stop = &self.stops[*stop];
        self.get_routes_for(stop)
    }
}

/// A connection between two stops
enum Connection {
    /// By using a trip with on a route with the respective transportation
    Connection {
        route: usize,
        trip_number: usize,
        boarded_at_stop: usize,
        exited_at_stop: usize,
    },
    /// By walking from a source stop (index in stops data structure) and the connected transfer (index)
    FootPath { source: usize, transfer: usize },
}

pub fn raptor(
    source: usize,
    target: usize,
    departure: &Time,
    routes: RoutesData,
    stops: StopsData,
) -> Vec<HashMap<usize, Connection>> {
    let mut k = 0usize;

    // For each round the best arrival by stop. Index is amount of transfers or k - 1
    let mut labels_by_round = vec![HashMap::from([(source, departure.clone())])];
    // The best arrival time for any stop without caring about the round
    let mut best_by_stop = HashMap::from([(source, departure)]);
    // Connections to reconstruct journey
    let mut connections_by_round = Vec::new();

    let mut marked_stops = HashSet::from([&source]);
    let mut queue: HashMap<&usize, &usize> = HashMap::new();

    while !marked_stops.is_empty() {
        k += 1;
        let last_round_labels = &labels_by_round[(k - 1)];
        let mut current_round_labels: HashMap<usize, Time> = HashMap::new();
        // Best connection for current round by the stop the connection reaches
        // For journey reconstruction
        let mut connection_by_stop: HashMap<usize, Connection> = HashMap::new();

        // Accumulate routes serving marked stops from previous round
        let routes_at_stop: HashMap<&usize, &[usize]> = marked_stops
            .iter()
            .map(|stop| (*stop, stops.get_routes(stop)))
            .collect();

        queue.clear();
        for p in &marked_stops {
            let routes_serving_p = routes_at_stop[p];

            for route in routes_serving_p {
                if let Some(p_other) = queue.get(route) {
                    let route_value = &routes.routes[*route];
                    let sequence = &routes.get_stop_sequence(route_value, p).unwrap();
                    let sequence_other = &routes.get_stop_sequence(route_value, p_other).unwrap();

                    // If p comes before p' (p_other) replace p' with p
                    if sequence < sequence_other {
                        &queue.insert(route, p);
                    }
                    continue;
                }

                let _ = &queue.insert(route, p);
            }
        }

        marked_stops.clear();

        for (route_index, p) in &queue {
            // Go through each stop of route starting with p
            let route = &routes.routes[**route_index];
            let stops = routes.get_route_stops(route);
            let mut current_trip: Option<(usize, &[StopTime], &usize)> = None;

            // Traverse stops in route starting with marked stop
            let start_sequence = stops.iter().position(|stop| &stop == p).unwrap();
            for stop_sequence in start_sequence..stops.len() {
                // Stop (index) of the stop in the trip we traverse
                let trip_stop = &stops[stop_sequence];

                if let Some((trip_number, trip_times, boarded_at_stop)) = current_trip {
                    // Earliest known arrival at stop for any route and trip (for local pruning?)
                    let earliest_arrival = best_by_stop.get(&trip_stop).unwrap_or(&&Infinite);
                    // Earliest arrival at target stop for journey. Used for target pruning.
                    // (We don't need to look at stops that arrive after the target arrival if we
                    // have one)
                    let earliest_arrival_target = best_by_stop.get(&target).unwrap_or(&&Infinite);
                    // Arrival time for the current stop on the current trip for the current route
                    let arrival_time = &trip_times[stop_sequence].arrival_time;
                    // Can label be improved

                    //TODO consider minimum time it takes to transfer between lines/routes/trips
                    //TODO check if we can drop off at stop
                    if &arrival_time < min(earliest_arrival, earliest_arrival_target) {
                        current_round_labels.insert(*trip_stop, *arrival_time);
                        best_by_stop.insert(*trip_stop, arrival_time);
                        // Save connection to reconstruct journey
                        let connection = Connection::Connection {
                            route: **route_index,
                            trip_number,
                            boarded_at_stop: *boarded_at_stop,
                            exited_at_stop: *trip_stop,
                        };
                        connection_by_stop.insert(*trip_stop, connection);
                        // Mark as improved
                        marked_stops.insert(&trip_stop);
                    }
                }

                // Can we catch an earlier trip?
                let previous_arrival = last_round_labels.get(trip_stop).unwrap_or(&&Infinite);

                // Pseudo code example code uses departure but this is probably a typo as text uses
                // arrival which makes more sense to my understanding of the algorithm
                let arrival_time = &current_trip
                    .map(|(_, trip, _)| &trip[stop_sequence].arrival_time)
                    .unwrap_or(&Infinite);

                if previous_arrival <= arrival_time {
                    current_trip = routes
                        .get_earliest_departing_trip(route, &stop_sequence, previous_arrival)
                        .map(|(trip_number, trip_times)| (trip_number, trip_times, trip_stop));
                }
            }
        }

        // Can not change marked stops while iterating so we save them here temporarily
        let mut new_marks = HashSet::new();
        // Look at foot-paths
        for p in &marked_stops {
            let stop = &stops.stops[**p];
            let start = stop.transfers_index_start;

            let arrival_at_p = current_round_labels.get(*p).cloned().unwrap_or(Infinite);

            for transfer_index in 0..stop.transfers_count {
                let transfer = &stops.transfers[start + transfer_index];
                let arrival_by_foot = arrival_at_p + transfer.time;

                let current_arrival_target = current_round_labels
                    .get(&transfer.target)
                    .cloned()
                    .unwrap_or(Infinite);

                if arrival_by_foot < current_arrival_target {
                    // Improved arrival time by walking
                    current_round_labels.insert(transfer.target, arrival_by_foot);
                    // Add footpath to connections
                    let connection = Connection::FootPath {
                        source: **p,
                        transfer: transfer_index,
                    };
                    connection_by_stop.insert(transfer.target, connection);
                    // Mark stop as improved
                    new_marks.insert(&transfer.target);
                    // marked_stops.insert(&transfer.target);
                }
            }
        }

        // Add collected improved stops
        marked_stops.extend(new_marks);

        labels_by_round.push(current_round_labels);
        connections_by_round.push(connection_by_stop);
    }

    return connections_by_round;
}

#[test]
fn huh() {
    assert_eq!(Finite(3), Finite(3))
}

//TODO Benchmark passing time as reference (Arc/Ref or &) vs copying/cloning time values...If that even matters at all
