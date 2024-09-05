use std::hash::{Hash, Hasher};
use crate::Time;

/// A route or line in a transportation network. A route has multiple trips a day.
/// In contrast to GTFS data a route has always the same sequence of stops in its trips.
/// This means there is a separate route for every trip in GTFS where the sequence of stops or
/// direction is not the same
#[derive(Clone)]
pub struct Route {
    /// Number of trips in a route. You can get the length of the block in StopTimes that represent
    /// all trips of this route by multiplying this with number_of_stops
    pub number_of_trips: usize,

    /// The number of stops per trip of this route. The number of trips is the same per trip.
    pub number_of_stops: usize,

    /// Pointer to the index that starts the block in the RouteStops array for the stops of this route
    pub route_stops_start_index: usize,

    /// Pointer to the index that starts the first block of StopTimes for the first trip
    pub stop_times_start_index: usize,
}

/// The departure and arrival time of a trip at a stop
#[derive(Clone)]
pub struct StopTime {
    pub departure_time: Time,
    pub arrival_time: Time,
}
#[derive(Clone)]
pub struct RoutesData {
    /// This array is divided into blocks, and the i-th block contains all trips corresponding
    /// to route ri. Within a block, trips are sorted by departure time (at the first stop).
    /// Each trip is just a sequence of stop times, represented by the corresponding arrival
    /// and departure times.
    pub stop_times: Vec<StopTime>,
    pub routes: Vec<Route>,
    /// The stops for routes where segments represent stops sequence for routes
    /// The first entries belong to routes[0] then the next to route[1] and so on...
    pub route_stops: Vec<usize>,
}

impl RoutesData {
    fn get_stop_times(&self, route: &Route) -> &[StopTime] {
        let start = route.stop_times_start_index;
        let length = route.number_of_trips * route.number_of_stops;
        let end = start + length;
        &self.stop_times[start..end]
    }

    pub(crate) fn get_route_stops(&self, route: &Route) -> &[usize] {
        let start = route.route_stops_start_index;
        let end = start + route.number_of_stops;
        &self.route_stops[start..end]
    }

    /// Get the sequence for a stop on the given route
    /// Returns none if the stop is not on the route otherwise the sequence index of the stop on the
    /// route
    pub(crate) fn get_stop_sequence(&self, route: &Route, stop: &usize) -> Option<usize> {
        let route_stops = self.get_route_stops(route);
        route_stops
            .iter()
            .position(|route_stop| &route_stop == &stop)
    }

    /// Get the earliest trip departing from a stop along the route after some time
    /// returns the number of the trip in the route (index in sequence of trips for route) and the
    /// trip stop times
    pub(crate) fn get_earliest_departing_trip(
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
                let trip_end = trip_start + route.number_of_stops;
                let trip = &stop_times[trip_start..trip_end];
                return Some((trip_index, trip));
            }
        }

        return None;
    }
}

#[derive(Clone)]
pub struct Stop {
    pub id: String,
    pub transfers_index_start: usize,
    pub stop_routes_index_start: usize,
    pub transfers_count: usize,
    pub stop_routes_count: usize,
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
#[derive(Clone)]
pub struct Transfer {
    /// The target stop that can be reached by foot through this foot-path
    pub target: usize,
    /// Time it takes to reach the target stop by foot
    pub time: Time,
}

#[derive(Clone)]
pub struct StopsData {
    pub transfers: Vec<Transfer>,
    pub stops: Vec<Stop>,
    /// Not the routes themselves but the indices of in the route data
    pub stop_routes: Vec<usize>,
}

impl StopsData {
    fn get_routes_for(&self, stop: &Stop) -> &[usize] {
        let start = stop.stop_routes_index_start;
        let end = start + stop.stop_routes_count;
        &self.stop_routes[start..end]
    }

    pub(crate) fn get_routes(&self, stop: &usize) -> &[usize] {
        let stop = &self.stops[*stop];
        self.get_routes_for(stop)
    }
}