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
    platform_code TEXT,
    FOREIGN KEY(level_id) REFERENCES levels(id)
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
    FOREIGN KEY(route_id) REFERENCES routes(id),
    FOREIGN KEY(service_id) REFERENCES calendar(service_id),
    FOREIGN KEY(shape_id) REFERENCES shapes(id)
);

CREATE TABLE IF NOT EXISTS stop_times (
    trip_id TEXT NOT NULL,
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

CREATE TABLE IF NOT EXISTS calendar (
    service_id TEXT PRIMARY KEY NOT NULL,
    monday BOOLEAN NOT NULL,
    tuesday BOOLEAN NOT NULL,
    wednesday BOOLEAN NOT NULL,
    thursday BOOLEAN NOT NULL,
    friday BOOLEAN NOT NULL,
    saturday BOOLEAN NOT NULL,
    sunday BOOLEAN NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL
);

CREATE TABLE IF NOT EXISTS calendar_dates (
    service_id TEXT NOT NULL,
    date DATE NOT NULL,
    exception_type INTEGER NOT NULL,
    PRIMARY KEY (service_id, date),
    FOREIGN KEY (service_id) REFERENCES calendar(service_id)
);

CREATE TABLE IF NOT EXISTS fare_attributes (
    fare_id TEXT NOT NULL PRIMARY KEY,
    price REAL NOT NULL,
    currency_type TEXT NOT NULL,
    payment_method INTEGER NOT NULL,
    transfers INTEGER NOT NULL,
    agency_id TEXT,
    transfer_duration INTEGER,
    FOREIGN KEY (agency_id) REFERENCES agencies(id)
);

CREATE TABLE IF NOT EXISTS fare_rules (
    fare_id TEXT NOT NULL,
    route_id TEXT,
    origin_id TEXT,
    destination_id TEXT,
    contains_id TEXT,
    PRIMARY KEY (fare_id, route_id, origin_id, destination_id, contains_id),
    FOREIGN KEY (fare_id) REFERENCES fare_attributes(fare_id),
    FOREIGN KEY (route_id) REFERENCES routes(id),
    FOREIGN KEY (origin_id) REFERENCES stops(zone_id),
    FOREIGN KEY (destination_id) REFERENCES stops(zone_id),
    FOREIGN KEY (contains_id) REFERENCES stops(zone_id)
);

CREATE TABLE IF NOT EXISTS timeframes (
    group_id TEXT NOT NULL,
    start_time TIME,
    end_time TIME,
    service_id TEXT NOT NULL,
    PRIMARY KEY (group_id, start_time, end_time, service_id),
    FOREIGN KEY (service_id) REFERENCES calendar(service_id)
);

CREATE TABLE IF NOT EXISTS fare_media (
    id TEXT NOT NULL PRIMARY KEY,
    name TEXT,
    type INTEGER
);

CREATE TABLE IF NOT EXISTS fare_products (
    id TEXT NOT NULL,
    name TEXT,
    media_id TEXT,
    amount REAL NOT NULL,
    currency TEXT,
    PRIMARY KEY (id, media_id),
    FOREIGN KEY (media_id) REFERENCES fare_media(id)
);

CREATE TABLE IF NOT EXISTS fare_leg_rules (
    group_id TEXT,
    network_id TEXT,
    from_area_id TEXT,
    to_area_id TEXT,
    from_timeframe_group_id TEXT,
    to_timeframe_group_id TEXT,
    fare_product_id TEXT NOT NULL,
    PRIMARY KEY (network_id, from_area_id, to_area_id, from_timeframe_group_id, to_timeframe_group_id, fare_product_id),
    FOREIGN KEY (network_id) REFERENCES routes(network_id),
    FOREIGN KEY (from_area_id) REFERENCES areas(id),
    FOREIGN KEY (to_area_id) REFERENCES areas(id),
    FOREIGN KEY (from_timeframe_group_id) REFERENCES timeframes(group_id),
    FOREIGN KEY (to_timeframe_group_id) REFERENCES timeframes(group_id),
    FOREIGN KEY (fare_product_id) REFERENCES fare_products(id)
);

CREATE TABLE IF NOT EXISTS fare_transfer_rules (
    from_leg_group_id TEXT,
    to_leg_group_id TEXT,
    transfer_count INTEGER,
    duration_limit INTEGER,
    duration_limit_type INTEGER,
    fare_transfer_type INTEGER NOT NULL,
    fare_product_id TEXT,
    PRIMARY KEY (from_leg_group_id, to_leg_group_id, fare_product_id, transfer_count, duration_limit),
    FOREIGN KEY (from_leg_group_id) REFERENCES fare_leg_rules(group_id),
    FOREIGN KEY (to_leg_group_id) REFERENCES fare_leg_rules(group_id),
    FOREIGN KEY (fare_product_id) REFERENCES fare_products(id)
);

CREATE TABLE IF NOT EXISTS areas (
    id TEXT NOT NULL PRIMARY KEY,
    name TEXT
);

CREATE TABLE IF NOT EXISTS stop_areas (
    area_id TEXT NOT NULL,
    stop_id TEXT NOT NULL,
    PRIMARY KEY (area_id, stop_id),
    FOREIGN KEY (area_id) REFERENCES areas(id),
    FOREIGN KEY (stop_id) REFERENCES stops(id)
);

CREATE TABLE IF NOT EXISTS shapes (
    id TEXT NOT NULL,
    point_latitude REAL NOT NULL,
    point_longitude REAL NOT NULL,
    point_sequence INTEGER NOT NULL,
    distance_traveled REAL,
    PRIMARY KEY (id, point_sequence)
);

CREATE TABLE IF NOT EXISTS frequencies (
    trip_id TEXT NOT NULL,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    headway_seconds INTEGER NOT NULL,
    exact_times INTEGER,
    PRIMARY KEY (trip_id, start_time),
    FOREIGN KEY (trip_id) REFERENCES trips(id)
);

CREATE TABLE IF NOT EXISTS transfers (
    from_stop_id TEXT,
    to_stop_id TEXT,
    from_route_id TEXT,
    to_route_id TEXT,
    from_trip_id TEXT,
    to_trip_id TEXT,
    type INTEGER NOT NULL,
    minimum_transfer_time INTEGER NOT NULL,
    FOREIGN KEY (from_stop_id) REFERENCES stops(id),
    FOREIGN KEY (to_stop_id) REFERENCES stops(id),
    FOREIGN KEY (from_route_id) REFERENCES routes(id),
    FOREIGN KEY (to_route_id) REFERENCES routes(id),
    FOREIGN KEY (from_trip_id) REFERENCES trips(id),
    FOREIGN KEY (to_trip_id) REFERENCES trips(id)
);

CREATE TABLE IF NOT EXISTS pathways (
    id TEXT NOT NULL PRIMARY KEY,
    from_stop_id TEXT NOT NULL,
    to_stop_id TEXT NOT NULL,
    mode INTEGER NOT NULL,
    is_bidirectional BOOLEAN NOT NULL,
    length REAL,
    traversal_time INTEGER,
    stair_count INTEGER,
    maximum_slope REAL,
    minimum_width REAL,
    signposted_as TEXT,
    reversed_signposted_as TEXT,
    FOREIGN KEY (from_stop_id) REFERENCES stops(id),
    FOREIGN KEY (to_stop_id) REFERENCES stops(id)
);

CREATE TABLE IF NOT EXISTS levels (
    id TEXT NOT NULL PRIMARY KEY,
    "index" REAL NOT NULL,
    name TEXT
);

CREATE TABLE IF NOT EXISTS translations (
    table_name TEXT NOT NULL,
    field_name TEXT NOT NULL,
    language TEXT NOT NULL,
    translation TEXT NOT NULL,
    record_id TEXT,
    record_sub_id TEXT,
    field_value TEXT,
    PRIMARY KEY (table_name, field_name, language, record_id, record_sub_id, field_value)
);

CREATE TABLE IF NOT EXISTS feed_info (
    publisher_name TEXT NOT NULL,
    publisher_url TEXT NOT NULL,
    language TEXT NOT NULL,
    default_language TEXT,
    start_date DATE,
    end_date DATE,
    version TEXT,
    contact_email TEXT,
    contact_url TEXT
);

CREATE TABLE IF NOT EXISTS attributions (
    id TEXT PRIMARY KEY,
    agency_id TEXT,
    route_id TEXT,
    trip_id TEXT,
    organization_name TEXT NOT NULL,
    is_producer BOOLEAN,
    is_operator BOOLEAN,
    is_authority BOOLEAN,
    url TEXT,
    email TEXT,
    phone TEXT,
    FOREIGN KEY (agency_id) REFERENCES agencies(id),
    FOREIGN KEY (route_id) REFERENCES routes(id),
    FOREIGN KEY (trip_id) REFERENCES trips(id)
);

-- Create virtual table for stop full text search
-- 'trigram' is to search for parts of words
CREATE VIRTUAL TABLE IF NOT EXISTS stop_names USING fts5(id, name, tokenize = 'trigram');
-- Populate with stops data
INSERT INTO stop_names (id, name)
SELECT id, name FROM stops;

-- Create triggers to keep the virtual table and source table in sync
CREATE TRIGGER IF NOT EXISTS insert_stop_names
	AFTER INSERT ON stops
BEGIN
	INSERT INTO stop_names (id, name)
    VALUES (NEW.id, NEW.name);
END;

CREATE TRIGGER IF NOT EXISTS update_stop_names
	AFTER UPDATE ON stops
BEGIN
	UPDATE stops
    SET id = NEW.id, name = NEW.name
    WHERE id = NEW.id;
END;


CREATE TRIGGER IF NOT EXISTS delete_stop_names
    AFTER DELETE ON stops
BEGIN
    DELETE FROM stops
    WHERE id = OLD.id;
END;


-- Create index on names to make lookup by name faster
CREATE INDEX IF NOT EXISTS stops_name_index ON stops (name);


END;