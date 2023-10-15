mod sql;

use crate::sql::{Insert, Route, Stop, StopTime, Trip};
use csv::Reader;
use serde::Deserialize;
use sql::Agency;
use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::{stdout, Read, Write};
use std::path::Path;
use std::str::FromStr;
use std::time::SystemTime;
use zip::read::ZipFile;
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

#[derive(Debug)]
enum Error {
    Csv(csv::Error),
    Sql(rusqlite::Error),
}

impl From<csv::Error> for Error {
    fn from(value: csv::Error) -> Self {
        Error::Csv(value)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(value: rusqlite::Error) -> Self {
        Error::Sql(value)
    }
}

/// This function allows inserting entries read from a CSV into an insertee like an SQLite
/// database. To be able to insert the entries the struct that should be inserted needs to be
/// deserializable and the insertee needs to be able to insert that struct (the Insert<T> trait
/// needs to be implemented for the struct T)
///
/// # Arguments
///
/// * `reader`:
/// * `inserter`:
///
/// returns: Result<(), Error>
///
/// # Examples
///
/// ```
///
/// ```
fn insert_csv<T: for<'a> Deserialize<'a>, TInsert: Insert<T>>(
    reader: &mut Reader<ZipFile>,
    insertee: &TInsert,
) -> Result<(), Error> {
    let mut count = 0f64;
    let now = SystemTime::now();

    for result in reader.deserialize() {
        let item: T = result?;
        insertee.insert(item)?;
        count += 1.0;
        let count_per_second = count / now.elapsed().unwrap().as_secs_f64();
        print!("\rCompleted {count} {count_per_second:.2}\t entries/s");
        stdout().flush().unwrap();
    }

    println!();

    Ok(())
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

        match file_name {
            "agency.txt" => insert_csv::<Agency, _>(&mut reader, &connection).unwrap(),
            "stops.txt" => insert_csv::<Stop, _>(&mut reader, &connection).unwrap(),
            "routes.txt" => insert_csv::<Route, _>(&mut reader, &connection).unwrap(),
            "trips.txt" => insert_csv::<Trip, _>(&mut reader, &connection).unwrap(),
            "stop_times.txt" => insert_csv::<StopTime, _>(&mut reader, &connection).unwrap(),
            _ => (),
        }
    }

    for file_name in CONDITIONALLY_REQUIRED_FILES {
        println!("Reading {file_name}");
        let file = archive
            .by_name(file_name)
            .expect("Required file is missing");
        let mut reader = Reader::from_reader(file);
        match file_name {
            "calendar.txt" => todo!(),
            "calendar_dates.txt" => todo!(),
            _ => (),
        }
    }
}
