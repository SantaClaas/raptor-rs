mod queries;
mod structs;

mod time;

use crate::sql::queries::*;
pub(crate) use crate::sql::structs::{
    Agency, Area, Attribution, Calendar, CalendarDate, FareAttribute, FareLegRule, FareMedia,
    FareProduct, FareRule, FareTransferRule, FeedInfo, Frequency, Level, Pathway, Route, Shape,
    Stop, StopArea, StopTime, Timeframe, Transfer, Translation, Trip,
};
pub(crate) use crate::sql::time::Time;
use rusqlite::types::ToSqlOutput;
use rusqlite::{named_params, Connection, ToSql};

const CREATE_TABLES_QUERY: &str = include_str!("create_tables.sql");

pub(crate) fn create_database() -> Result<Connection, rusqlite::Error> {
    let connection = Connection::open("gtfs.db")?;

    connection.execute_batch(CREATE_TABLES_QUERY)?;
    Ok(connection)
}

pub(crate) trait Insert<T> {
    fn insert(&self, item: T) -> rusqlite::Result<()>;
}

impl Insert<Agency> for Connection {
    fn insert(&self, agency: Agency) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_AGENCY_QUERY)?;

        statement.execute(named_params! {
            ":id": agency.id,
            ":name": agency.name,
            ":url": agency.url,
            ":timezone": agency.timezone,
            ":language": agency.language,
            ":phone": agency.phone,
            ":fare_url": agency.fare_url,
            ":email": agency.email
        })?;

        Ok(())
    }
}

impl Insert<Route> for Connection {
    fn insert(&self, route: Route) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_ROUTE_QUERY)?;

        statement.execute(named_params! {
           ":id": route.id,
           ":agency_id": route.agency_id,
           ":short_name": route.short_name,
           ":long_name": route.long_name,
           ":description": route.description,
           ":type": route.r#type,
           ":url": route.url,
           ":color": route.color,
           ":text_color": route.text_color,
           ":sort_order": route.sort_order,
           ":continuous_pickup": route.continuous_pickup,
           ":continuous_drop_off": route.continuous_drop_off,
           ":network_id": route.network_id,
        })?;

        Ok(())
    }
}

impl Insert<Stop> for Connection {
    fn insert(&self, stop: Stop) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_STOP_QUERY)?;

        statement.execute(named_params! {
            ":id": stop.id,
            ":code": stop.code,
            ":name": stop.name,
            ":text_to_speech_name": stop.text_to_speech_name,
            ":description": stop.description,
            ":latitude": stop.latitude,
            ":longitude": stop.longitude,
            ":zone_id": stop.zone_id,
            ":url": stop.url,
            ":location_type": stop.location_type,
            ":parent_station": stop.parent_station,
            ":timezone": stop.timezone,
            ":wheelchair_boarding": stop.wheelchair_boarding,
            ":level_id": stop.level_id,
            ":platform_code": stop.platform_code,
        })?;

        Ok(())
    }
}

impl ToSql for Time {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}
impl Insert<StopTime> for Connection {
    fn insert(&self, stop_time: StopTime) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_STOP_TIME_QUERY)?;

        statement.execute(named_params! {
            ":trip_id": stop_time.trip_id,
            ":arrival_time": stop_time.arrival_time,
            ":arrival_time_seconds": stop_time.arrival_time.map(Time::total_seconds),
            ":departure_time": stop_time.departure_time,
            ":departure_time_seconds": stop_time.departure_time.map(Time::total_seconds),
            ":stop_id": stop_time.stop_id,
            ":stop_sequence": stop_time.stop_sequence,
            ":stop_headsign": stop_time.stop_headsign,
            ":pickup_type": stop_time.pickup_type,
            ":drop_off_type": stop_time.drop_off_type,
            ":continuous_pickup": stop_time.continuous_pickup,
            ":continuous_drop_off": stop_time.continuous_drop_off,
            ":shape_distance_traveled": stop_time.shape_distance_travelled,
            ":timepoint": stop_time.timepoint,
        })?;

        Ok(())
    }
}

impl Insert<Trip> for Connection {
    fn insert(&self, trip: Trip) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_TRIP_QUERY)?;

        statement.execute(named_params! {
            ":id": trip.id,
            ":route_id": trip.route_id,
            ":service_id": trip.service_id,
            ":headsign": trip.headsign,
            ":short_name": trip.short_name,
            ":direction": trip.direction_id,
            ":block_id": trip.block_id,
            ":shape_id": trip.shape_id,
            ":wheelchair_accessible": trip.wheelchair_accessible,
            ":bikes_allowed": trip.bikes_allowed,
        })?;

        Ok(())
    }
}

impl Insert<Calendar> for Connection {
    fn insert(&self, calendar: Calendar) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_CALENDAR_QUERY)?;

        statement.execute(named_params! {
            ":service_id": calendar.service_id,
            ":monday": calendar.monday,
            ":tuesday": calendar.tuesday,
            ":wednesday": calendar.wednesday,
            ":thursday": calendar.thursday,
            ":friday": calendar.friday,
            ":saturday": calendar.saturday,
            ":sunday": calendar.sunday,
            ":start_date": calendar.start_date,
            ":end_date": calendar.end_date,
        })?;

        Ok(())
    }
}

impl Insert<CalendarDate> for Connection {
    fn insert(&self, calendar_date: CalendarDate) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_CALENDAR_DATE_QUERY)?;

        statement.execute(named_params! {
            ":service_id": calendar_date.service_id,
            ":date": calendar_date.date,
            ":exception_type": calendar_date.exception_type,
        })?;

        Ok(())
    }
}

impl Insert<FareAttribute> for Connection {
    fn insert(&self, fare_attribute: FareAttribute) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_FARE_ATTRIBUTE_QUERY)?;

        statement.execute(named_params! {
            ":fare_id": fare_attribute.fare_id,
            ":price": fare_attribute.price,
            ":currency_type": fare_attribute.currency_type,
            ":payment_method": fare_attribute.payment_method,
            ":transfers": fare_attribute.transfers,
            ":agency_id": fare_attribute.agency_id,
            ":transfer_duration": fare_attribute.transfer_duration,
        })?;

        Ok(())
    }
}

impl Insert<FareLegRule> for Connection {
    fn insert(&self, fare_leg_rule: FareLegRule) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_FARE_LEG_RULE_QUERY)?;

        statement.execute(named_params! {
            ":group_id": fare_leg_rule.group_id,
            ":network_id": fare_leg_rule.network_id,
            ":from_area_id": fare_leg_rule.from_area_id,
            ":to_area_id": fare_leg_rule.to_area_id,
            ":from_timeframe_group_id": fare_leg_rule.from_timeframe_group_id,
            ":to_timeframe_group_id": fare_leg_rule.to_timeframe_group_id,
            ":fare_product_id": fare_leg_rule.fare_product_id,
        })?;

        Ok(())
    }
}

impl Insert<FareMedia> for Connection {
    fn insert(&self, fare_media: FareMedia) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_FARE_MEDIA_QUERY)?;

        statement.execute(named_params! {
            ":id": fare_media.id,
            ":name": fare_media.name,
            ":type": fare_media.r#type
        })?;

        Ok(())
    }
}

impl Insert<FareProduct> for Connection {
    fn insert(&self, fare_product: FareProduct) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_FARE_PRODUCT_QUERY)?;

        statement.execute(named_params! {
            ":id": fare_product.id,
            ":name": fare_product.name,
            ":media_id": fare_product.media_id,
            ":amount": fare_product.amount,
            ":currency": fare_product.currency,
        })?;

        Ok(())
    }
}

impl Insert<FareRule> for Connection {
    fn insert(&self, fare_rule: FareRule) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_FARE_RULE_QUERY)?;

        statement.execute(named_params! {
            ":fare_id": fare_rule.fare_id,
            ":route_id": fare_rule.route_id,
            ":origin_id": fare_rule.origin_id,
            ":destination_id": fare_rule.destination_id,
            ":contains_id": fare_rule.contains_id,
        })?;

        Ok(())
    }
}

impl Insert<FareTransferRule> for Connection {
    fn insert(&self, fare_transfer_rule: FareTransferRule) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_FARE_TRANSFER_RULE_QUERY)?;

        statement.execute(named_params! {
            ":from_leg_group_id": fare_transfer_rule.from_leg_group_id,
            ":to_leg_group_id": fare_transfer_rule.to_leg_group_id,
            ":transfer_count": fare_transfer_rule.transfer_count,
            ":duration_limit": fare_transfer_rule.duration_limit,
            ":duration_limit_type": fare_transfer_rule.duration_limit_type,
            ":fare_transfer_type": fare_transfer_rule.fare_transfer_type,
            ":fare_product_id": fare_transfer_rule.fare_product_id,
        })?;

        Ok(())
    }
}

impl Insert<FeedInfo> for Connection {
    fn insert(&self, feed_info: FeedInfo) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_FEED_INFO_QUERY)?;

        statement.execute(named_params! {
            ":publisher_name": feed_info.publisher_name,
            ":publisher_url": feed_info.publisher_url,
            ":language": feed_info.language,
            ":default_language": feed_info.default_language,
            ":start_date": feed_info.start_date,
            ":end_date": feed_info.end_date,
            ":version": feed_info.version,
            ":contact_email": feed_info.contact_email,
            ":contact_url": feed_info.contact_url,
        })?;

        Ok(())
    }
}

impl Insert<Frequency> for Connection {
    fn insert(&self, frequency: Frequency) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_FREQUENCY_QUERY)?;

        statement.execute(named_params! {
            ":trip_id": frequency.trip_id,
            ":start_time": frequency.start_time,
            ":end_time": frequency.end_time,
            ":headway_seconds": frequency.headway_seconds,
            ":exact_times": frequency.exact_times,
        })?;

        Ok(())
    }
}

impl Insert<Level> for Connection {
    fn insert(&self, level: Level) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_LEVEL_QUERY)?;

        statement.execute(named_params! {
            ":id": level.id,
            ":index": level.index,
            ":name": level.name,
        })?;

        Ok(())
    }
}

impl Insert<Pathway> for Connection {
    fn insert(&self, pathway: Pathway) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_PATHWAY_QUERY)?;

        statement.execute(named_params! {
            ":id": pathway.id,
            ":from_stop_id": pathway.from_stop_id,
            ":to_stop_id": pathway.to_stop_id,
            ":mode": pathway.mode,
            ":is_bidirectional": pathway.is_bidirectional,
            ":length": pathway.length,
            ":traversal_time": pathway.traversal_time,
            ":stair_count": pathway.stair_count,
            ":maximum_slope": pathway.maximum_slope,
            ":minimum_width": pathway.minimum_width,
            ":signposted_as": pathway.signposted_as,
            ":reversed_signposted_as": pathway.reversed_signposted_as,
        })?;

        Ok(())
    }
}

impl Insert<Shape> for Connection {
    fn insert(&self, shape: Shape) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_SHAPE_QUERY)?;

        statement.execute(named_params! {
            ":id": shape.id,
            ":point_latitude": shape.point_latitude,
            ":point_longitude": shape.point_longitude,
            ":point_sequence": shape.point_sequence,
            ":distance_traveled": shape.distance_traveled,
        })?;

        Ok(())
    }
}

impl Insert<StopArea> for Connection {
    fn insert(&self, stop_area: StopArea) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_STOP_AREA_QUERY)?;

        statement.execute(
            named_params! {":area_id": stop_area.area_id, ":stop_id": stop_area.stop_id},
        )?;

        Ok(())
    }
}

impl Insert<Transfer> for Connection {
    fn insert(&self, transfer: Transfer) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_TRANSFER_QUERY)?;

        statement.execute(named_params! {
            ":from_stop_id": transfer.from_stop_id,
            ":to_stop_id": transfer.to_stop_id,
            ":from_route_id": transfer.from_route_id,
            ":to_route_id": transfer.to_route_id,
            ":from_trip_id": transfer.from_trip_id,
            ":to_trip_id": transfer.to_trip_id,
            ":type": transfer.r#type,
            ":minimum_transfer_time": transfer.minimum_transfer_time,
        })?;

        Ok(())
    }
}

impl Insert<Translation> for Connection {
    fn insert(&self, translation: Translation) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_TRANSLATION_QUERY)?;

        statement.execute(named_params! {
            ":table_name": translation.table_name,
            ":field_name": translation.field_name,
            ":language": translation.language,
            ":translation": translation.translation,
            ":record_id": translation.record_id,
            ":record_sub_id": translation.record_sub_id,
            ":field_value": translation.field_value,
        })?;

        Ok(())
    }
}

impl Insert<Timeframe> for Connection {
    fn insert(&self, timeframe: Timeframe) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_TIMEFRAME_QUERY)?;

        statement.execute(named_params! {
            ":group_id": timeframe.group_id,
            ":start_time": timeframe.start_time,
            ":end_time": timeframe.end_time,
            ":service_id": timeframe.service_id,
        })?;

        Ok(())
    }
}

impl Insert<Area> for Connection {
    fn insert(&self, area: Area) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_AREA_QUERY)?;

        statement.execute(named_params! {":id" :    area.id, ":name"     : area.name})?;

        Ok(())
    }
}

impl Insert<Attribution> for Connection {
    fn insert(&self, attribution: Attribution) -> rusqlite::Result<()> {
        let mut statement = self.prepare_cached(INSERT_ATTRIBUTION_QUERY)?;

        statement.execute(named_params! {
            ":id": attribution.id,
            ":agency_id": attribution.agency_id,
            ":route_id": attribution.route_id,
            ":trip_id": attribution.trip_id,
            ":organization_name": attribution.organization_name,
            ":is_producer": attribution.is_producer,
            ":is_operator": attribution.is_operator,
            ":is_authority": attribution.is_authority,
            ":url": attribution.url,
            ":email": attribution.email,
            ":phone": attribution.phone,
        })?;

        Ok(())
    }
}
