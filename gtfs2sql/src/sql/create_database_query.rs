pub(super) const CREATE_DATABASE_QUERY: &str =
    /*language=sqlite*/
    "
          BEGIN;
          CREATE TABLE IF NOT EXISTS agencies (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            url TEXT NOT NULL,
            timezone TEXT NOT NULL,
            language TEXT,
            phone TEXT,
            fare_url TEXT,
            email TEXT
          );

          CREATE TABLE IF NOT EXISTS stops (
            id TEXT PRIMARY KEY,
            code TEXT,
            name TEXT,
            text_to_speech_name TEXT,
            description TEXT,
            latitude DECIMAL(7, 5),
            longitude DECIMAL(8, 5),
            zone_id TEXT,
            url TEXT,
            location_type INTEGER,
            parent_station TEXT,
            timezone TEXT,
            wheelchair_boarding INTEGER,
            level_id TEXT,
            platform_code TEXT--,
            --FOREIGN KEY(level_id) REFERENCES levels(id),
          );

          CREATE TABLE IF NOT EXISTS routes (
            id TEXT PRIMARY KEY,
            agency_id TEXT,
            short_name TEXT,
            long_name TEXT,
            description TEXT,
            type INTEGER NOT NULL,
            url TEXT,
            color TEXT,
            text_color TEXT,
            sort_order INTEGER,
            continuous_pickup INTEGER,
            continuous_drop_off INTEGER,
            network_id TEXT,
            FOREIGN KEY(agency_id) REFERENCES agencies(id)
          );

          CREATE TABLE IF NOT EXISTS trips (
            id TEXT PRIMARY KEY,
            route_id TEXT NOT NULL,
            service_id TEXT NOT NULL,
            headsign TEXT,
            short_name TEXT,
            direction BOOLEAN,
            block_id TEXT,
            shape_id TEXT,
            wheelchair_accessible INTEGER,
            bikes_allowed INTEGER,
            FOREIGN KEY(route_id) REFERENCES routes(id)--,
            --FOREIGN KEY(service_id) REFERENCES services(id),
            --FOREIGN KEY(shape_id) REFERENCES shapes(id)
          );

          CREATE TABLE IF NOT EXISTS stop_times (
            trip_id TEXT,
            arrival_time TEXT,
            arrival_time_seconds INTEGER,
            departure_time TEXT,
            departure_time_seconds INTEGER,
            stop_id TEXT,
            stop_sequence INTEGER,
            stop_headsign TEXT,
            pickup_type INTEGER,
            drop_off_type INTEGER,
            continuous_pickup INTEGER,
            continuous_drop_off INTEGER,
            shape_distance_traveled REAL,
            timepoint INTEGER,
            PRIMARY KEY (trip_id, stop_id, stop_sequence),
            FOREIGN KEY(trip_id) REFERENCES trips(id),
            FOREIGN KEY(stop_id) REFERENCES stops(id)
          );

          COMMIT;
        ";
