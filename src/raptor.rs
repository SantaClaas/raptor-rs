use std::cmp::{min, Ordering};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::iter::Map;
use std::slice::Chunks;

/// Represents a time stamp for various structures in RAPTOR.
/// The value represents a time after midnight for a day. It can be greater than 24h if a stop on a
/// trip is reached the next day after midnight
#[derive(Debug)]
struct Time(u64);

impl Eq for Time {}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

struct NumberOfTrips(u32);

struct StopId(String);

impl PartialEq<Self> for StopId {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for StopId {}

impl Hash for StopId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

struct Trip {}

/// The time a trip stops at a stop
struct StopTime {
    departure_time: Time,
    arrival_time: Time,
}

/// A route or line in a transportation network. A route has multiple trips a day.
/// In contrast to GTFS data a route has always the same sequence of stops in its trips.
/// This means there is a separate route for every trip in GTFS where the sequence of stops or
/// direction is not the same
struct Route {
    number_of_trips: usize,

    /// The number of stops per trip of this route. The number of trips is the same per trip.
    number_of_stops: usize,

    /// Pointer to the RouteStops list representing the index where the first stop for the route
    /// starts
    route_stops_start_index: usize,

    /// Pointer to the StopTimes list representing the trips that operate on the route
    stop_times_start_index: usize,
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
    //TODO Test getting trip
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

    fn get_trips(&self, route: &Route) -> Chunks<StopTime> {
        // Access stop times of all trips on route
        let stop_times = self.get_stop_times(route);
        stop_times.chunks(route.number_of_stops)
    }

    /// Get the earliest trip departing from a stop along the route after some time
    fn get_earliest_departing_trip(
        &self,
        from_stop_id: &usize,
        route: &Route,
        after: &Time,
    ) -> Option<&[StopTime]> {
        // Only the iterator is mutable
        let mut trips = self.get_trips(route);
        self.get_stop_sequence(route, &from_stop_id)
            .and_then(|from_stop_sequence| {
                trips.find(|trip| &trip[from_stop_sequence].departure_time > after)
            })
    }
}

struct Label {
    trips: NumberOfTrips,
    earliest_known_arrival: Time,
}

// /// A route that serves a stop
// struct StopRoute {
//     id: String,
// }
//
// impl Hash for StopRoute {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.id.hash(state)
//     }
// }
//
// impl PartialEq<Self> for StopRoute {
//     fn eq(&self, other: &Self) -> bool {
//         self.id.eq(&other.id)
//     }
// }
//
// impl Eq for StopRoute {}

/// The index of a route in the route data
struct RouteIndex(usize);

struct Stop {
    id: StopId,
    transfers_index_start: usize,
    stop_routes_index_start: usize,
    transfers_count: usize,
    stop_routes_count: usize,
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
struct Transfer {}

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

    fn get_transfers(&self, stop: &Stop) -> &[Transfer] {
        let start = stop.transfers_index_start;
        let end = start + stop.transfers_count;
        &self.transfers[start..end]
    }
}

fn raptor(source: usize, target: usize, departure: &Time, routes: RoutesData, stops: StopsData) {
    let k = 0usize;

    let mut labels_by_stop = HashMap::from([(source, HashMap::from([(k, departure)]))]);

    let mut earliest_arrivals = HashMap::from([(k, departure)]);
    let mut marked_stops = HashSet::from([&source]);
    let mut queue: HashMap<&usize, &usize> = HashMap::new();

    while marked_stops.len() > 0usize {
        // Accumulate routes serving marked stops from previous round
        let routes_at_stop: HashMap<&usize, &[usize]> = marked_stops
            .iter()
            .map(|stop| (*stop, stops.get_routes(stop)))
            .collect();

        queue.clear();
        for p in marked_stops.iter() {
            let routes_serving_p = routes_at_stop[p];

            for route in routes_serving_p {
                match queue.get(route) {
                    None => {
                        &queue.insert(route, p);
                    }
                    Some(p_other) => {
                        let route_value = &routes.routes[*route];
                        let sequence = &routes.get_stop_sequence(route_value, p).unwrap();
                        let sequence_other =
                            &routes.get_stop_sequence(route_value, p_other).unwrap();

                        // If p comes before p' (p_other) replace p' with p
                        if sequence < sequence_other {
                            &queue.insert(route, p);
                        }
                    }
                }
            }
        }

        marked_stops.clear();

        for (route, p) in &queue {
            // Go through each stop of route starting with p
            let route = &routes.routes[**route];
            let stops = routes.get_route_stops(route);
            let mut current_trip: Option<&[StopTime]> = None;

            let start_sequence = stops.iter().position(|stop| &stop == p).unwrap();
            for sequence in start_sequence..stops.len() {
                let stop = &stops[sequence];

                if let Some(trip) = current_trip {
                    // Earliest known arrival at stop for any route and trip (for local pruning?)
                    let earliest_arrival = earliest_arrivals.get(&stop);
                    // Earliest arrival at target stop for journey. Used for target pruning.
                    // (We don't need to look at stops that arrive after the target arrival if we
                    // have one)
                    let earliest_arrival_target = earliest_arrivals.get(&target);

                    // Arrival time for the current stop on the current trip for the current route
                    let arrival_time = &trip[sequence].arrival_time;
                    // Can label be improved

                    if Some(&arrival_time) < min(earliest_arrival, earliest_arrival_target) {
                        let time_by_round = labels_by_stop.entry(*stop).or_default();
                        time_by_round.insert(k, arrival_time);
                        earliest_arrivals.insert(*stop, arrival_time);
                        //TODO arr trip to reconstruct journey
                        marked_stops.insert(&stop);
                    }
                }

                // If we have None for a time value, it can be treated as infinite. No arrival time -> Infinite time to arrive
                // Can we catch an earlier trip?
                let previous_arrival = labels_by_stop
                    .get(stop)
                    .and_then(|time_by_round| time_by_round.get(&(k - 1)));

                // Pseudo code example code uses departure but this is probably a typo as text uses
                // arrival which seems to make more sense too
                let arrival_time = current_trip.map(|trip| &trip[sequence].arrival_time);

                // Using is_some as let Some(time) chained with comparison is currently unstable
                // And it would need to be wrapped in Some again for comparison

                //TODO simplify this
                match (previous_arrival, arrival_time) {
                    (Some(previous), None /* "Infinite" */) => {
                        current_trip = routes.get_earliest_departing_trip(stop, route, previous);
                    }
                    (Some(previous), Some(arrival)) if previous <= &arrival => {
                        current_trip = routes.get_earliest_departing_trip(stop, route, previous);
                    }
                    (None /* "Infinite" */, Some(_)) | (Some(_), _) | (None, None) => {}
                }
            }
        }
    }
}

#[test]
fn huh() {
    assert_eq!(Time(3), Time(3))
}
