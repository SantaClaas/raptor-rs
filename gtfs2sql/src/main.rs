mod sql;

use crate::sql::Route;
use csv::{Reader, StringRecord};
use sql::Agency;
use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use zip::ZipArchive;

const REQUIRED_FILES: [&'static str; 5] = [
    "agency.txt",
    "stops.txt",
    "routes.txt",
    "trips.txt",
    "stop_times.txt",
];

const CONDITIONALLY_REQUIRED_FILES: [&'static str; 2] = ["calendar.txt", "calendar_dates.txt"];

const OPTIONAL_FILES: [&'static str; 17] = [
    "fare_attributes.txt",
    "fare_rules.txt",
    "timeframes.txt",
    "fare_media.txt",
    "fare_products.txt",
    "fare_leg_rules.txt",
    "fare_transfer_rules.txt",
    "areas.txt",
    "stop_areas.txt",
    "shapes.txt",
    "frequencies.txt",
    "transfers.txt",
    "pathways.txt",
    "levels.txt",
    "translations.txt",
    "feed_info.txt",
    "attributions.txt",
];

trait MapInto<T> {
    fn map_into<U>(self) -> Option<U>
    where
        U: From<T>;
}

impl<T> MapInto<T> for Option<T> {
    fn map_into<U>(self) -> Option<U>
    where
        U: From<T>,
    {
        self.map(U::from)
    }
}

fn main() {
    let connection = sql::create_database().unwrap();

    let file_path = env::args()
        .nth(1)
        .expect("Expected file path to GTFS zip file to be passed");

    let file_path = Path::new(&*file_path);
    let file = File::open(file_path).expect("Could not open file");

    let mut archive = ZipArchive::new(file).expect("Could read file as zip");

    for file_name in REQUIRED_FILES {
        println!("Reading {file_name}");
        let file = archive
            .by_name(file_name)
            .expect("Required file is missing");
        let mut reader = Reader::from_reader(file);

        let headers = reader
            .headers()
            .expect("Expected file to have headers. Can not read file without headers.")
            .clone();

        let headers = Vec::from_iter(&headers);

        let get_by_name = |record: &StringRecord, header_name: &str| -> Option<String> {
            // Linear search is probably easier than binary as there aren't that many headers
            headers
                .iter()
                .position(|header| *header == header_name)
                .and_then(|position| record.get(position))
                .map(ToString::to_string)
        };

        match file_name {
            "agency.txt" => {
                let mut count = 0usize;
                for result in reader.records() {
                    let record = result.expect("Could not read record from agencies");
                    let get_required = |name| {
                        get_by_name(&record, name)
                            .expect(&*format!("Expected agency to have field {}", name))
                    };
                    let get = |name| get_by_name(&record, name);

                    let agency = Agency {
                        id: get_required("agency_id"),
                        name: get_required("agency_name"),
                        url: get_required("agency_url"),
                        timezone: get_required("agency_timezone"),
                        language: get("agency_lang"),
                        phone: get("agency_phone"),
                        fare_url: get("agency_fare_url"),
                        email: get("agency_email"),
                    };

                    sql::insert_agency(&connection, agency).unwrap();

                    count += 1;
                    println!("Completed {count}");
                }
            }
            "routes.txt" => {
                let mut count = 0usize;
                for result in reader.records() {
                    let record = result.expect("Could not read record from agencies");
                    let get_required = |name| {
                        get_by_name(&record, name)
                            .expect(&*format!("Expected agency to have field {}", name))
                    };
                    let get = |name| get_by_name(&record, name);

                    let route = Route {
                        id: get_required("route_id"),
                        agency_id: get("agency_id"),
                        short_name: get("route_short_name"),
                        long_name: get("route_long_name"),
                        description: get("route_desc"),
                        route_type: get_required("route_type").parse().unwrap(),
                        url: get("route_url"),
                        color: get("route_color"),
                        text_color: get("route_text_color"),
                        sort_order: get("route_sort_order")
                            .map(|order| u32::from_str(order.as_str()).unwrap()),
                        continuous_pickup: get("continuous_pickup")
                            .map(|order| u8::from_str(order.as_str()).unwrap()),
                        continuous_drop_off: get("continuous_drop_off")
                            .map(|order| u8::from_str(order.as_str()).unwrap()),
                        network_id: get("network_id"),
                    };

                    sql::insert_route(&connection, route).unwrap();

                    count += 1;
                    println!("Completed {count}");
                }
            }
            _ => todo!(),
        }
    }
}
