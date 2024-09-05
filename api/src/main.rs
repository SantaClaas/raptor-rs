use std::collections::HashMap;
use std::convert::Infallible;
use std::env;
use std::env::{current_dir, current_exe};
use std::net::Ipv4Addr;
use std::sync::Arc;
use askama_axum::{Response, Template};
use axum::response::{Html, IntoResponse, Redirect};
use axum::{async_trait, Form, Router};
use axum::extract::{FromRef, FromRequestParts, Query, RawQuery, State};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::routing::{get, post};
use libsql::{named_params, Connection};
use raptor::{raptor, Time};
use tower_http::services::ServeDir;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use serde::Deserialize;
use time::format_description::well_known::{Iso8601, Rfc3339};
use time::{OffsetDateTime, PrimitiveDateTime};
use time::format_description::well_known;
use tracing::{debug, error};
use raptor::shared::{RoutesData, StopsData};
use sql2raptor::{assemble_raptor_data, get_routes, get_stops, GetStopsReturn, PartialStop};
use crate::request::{DateTimeLocal, SearchConnectionRequest};

mod request;

struct RaptorDataSet {
    index_by_stop_id: HashMap<String, usize>,
    routes_data: RoutesData,
    stops_data: StopsData,
}
async fn setup_raptor(connection: &libsql::Connection) -> Result<RaptorDataSet, libsql::Error> {
    let GetStopsReturn {
        transfers,
        stops: partial_stops,
        index_by_stop_id,
    } = get_stops(&connection).await?;

    let step_2_result = get_routes(
        &connection,
        //TODO remove debug clone clown
        index_by_stop_id.clone(),
    ).await?;

    //TODO check if trip ids are needed
    let (routes_data, stops_data, _trip_ids) =
        assemble_raptor_data(step_2_result, partial_stops, transfers);

    Ok(RaptorDataSet { index_by_stop_id, routes_data, stops_data })
}

#[derive(Clone)]
struct AppState {
    connection: libsql::Connection,
    raptor_data: Arc<RaptorDataSet>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=trace,tower_http=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database = libsql::Builder::new_local("gtfs.db").build().await.unwrap();
    let connection = database.connect().unwrap();

    let raptor_data = setup_raptor(&connection).await.unwrap();

    let state = AppState { connection, raptor_data: Arc::new(raptor_data) };


    let app = Router::new()
        .route("/", get(index))
        .route("/stops/start", post(search_start_stops))
        .route("/stops/end", post(search_end_stops))
        // If the route could not be matched it might be a file
        // This needs to be relative from where the app is run
        // If you run from the workspace root, it will not work
        .fallback_service(ServeDir::new("public"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind((Ipv4Addr::new(127, 0, 0, 1), 3000))
        .await
        .unwrap();


    tracing::debug!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Serialize)]
struct ResultRow {
    stop_name: String,
    route: usize,
    trip_number: usize,
    boarded: String,
    exited: String,
}

#[derive(Template, Default)]
#[template(path = "index.html")]
struct IndexTemplate {
    error: Option<String>,
    start: Option<String>,
    end: Option<String>,
    start_error: Option<String>,
    end_error: Option<String>,
    departure: Option<String>,
    results: Option<Vec<Vec<ResultRow>>>,
}

async fn get_stop_id(connection: &libsql::Connection, stop_name: &str) -> Result<Option<String>, libsql::Error> {
    let mut rows = connection.query(
        "SELECT id FROM stops WHERE name = :name", libsql::named_params!(":name": stop_name)).await?;

    let row = rows.next().await?;
    row.map(|row| row.get::<String>(0)).transpose()
}
fn try_format(departure: &DateTimeLocal) -> Option<String> {
    match departure.format() {
        Ok(departure) => Some(departure),
        Err(error) => {
            error!("Error formatting departure: {error}");
            None
        }
    }
}
async fn index(State(state): State<AppState>, Query(request): Query<SearchConnectionRequest>) -> impl IntoResponse {
    match request {
        SearchConnectionRequest {
            start: Some(start),
            end: Some(end),
            departure: Some(departure),
            ..
        } => {
            let start_result = get_stop_id(&state.connection, &start).await;
            // let formatted = departure.format(&well_known::Rfc2822::);
            // let formatted = departure.format()
            // debug!("Departure: {formatted:?}");
            // let result = OffsetDateTime::parse(&departure, &Iso8601::DEFAULT);
            // debug!("Departure: {departure}");

            // Don't proceed if the first already failed
            let Ok(start_id) = start_result else {
                error!("Error searching for start stop: {start_result:?}");
                let template = IndexTemplate {
                    error: Some("Sorry, something on our side went wrong. Could not search for connections.".to_string()),
                    start: Some(start),
                    end: Some(end),
                    departure: try_format(&departure),

                    ..Default::default()
                };
                return template;
            };

            let end_result = get_stop_id(&state.connection, &end).await;
            let Ok(end_id) = end_result else {
                error!("Error searching for end stop: {end_result:?}");

                let template = IndexTemplate {
                    error: Some("Sorry, something on our side went wrong. Could not search for connections.".to_string()),
                    start: Some(start),
                    end: Some(end),
                    departure: try_format(&departure),
                    ..Default::default()
                };
                return template;
            };

            match (start_id, end_id) {
                (Some(start_id), Some(end_id)) => {
                    //TODO search for connections
                    debug!("Searching for connection from start id {} to end id {}", &start_id, &end_id);

                    let source_index = *state.raptor_data.index_by_stop_id.get(&start_id).unwrap();
                    let target_index = *state.raptor_data.index_by_stop_id.get(&end_id).unwrap();
                    // let (hours, minutes, seconds) = departure.time().as_hms();
                    let raptor_departure = Time::from(departure.to_seconds());
                    let rounds = raptor(
                        source_index,
                        target_index,
                        &raptor_departure,
                        //TODO remove clone. These should be read only by reference
                        state.raptor_data.routes_data.clone(),
                        state.raptor_data.stops_data.clone(),
                    );

                    dbg!(&rounds[0].len());

                    let with_id = |(stop_index, connection): (usize, raptor::Connection)| {
                        let stop_id = state.raptor_data.stops_data.stops[stop_index].id.clone();
                        (stop_id, connection)
                    };

                    let collect_ids = |(stop_id, connection): &(String, raptor::Connection)| {
                        let mut ids = vec![stop_id.clone()];

                        if let raptor::Connection::Connection { boarded_at_stop, exited_at_stop, .. } = connection {
                            let boarded_id = state.raptor_data.stops_data.stops[*boarded_at_stop].id.clone();
                            let exited_id = state.raptor_data.stops_data.stops[*exited_at_stop].id.clone();
                            ids.push(boarded_id);
                            ids.push(exited_id);
                        }

                        ids
                    };
                    // let with_ids = | mut round: HashMap<usize, Connection>| round.into_iter().map(with_id).collect();
                    let rounds: Vec<Vec<(String, raptor::Connection)>> = rounds.into_iter().map(|round| round.into_iter().map(with_id).collect()).collect();

                    // Collect all distinct stop ids for a batched SQL query
                    let mut ids: Vec<String> = rounds.iter().flatten().map(collect_ids).flatten().collect();

                    // Remove duplicates
                    // Nor sure if this is faster than collecting to a hashset TODO measure
                    ids.sort();
                    ids.dedup();
                    /// Count of ids has to be less than SQLITE_MAX_VARIABLE_NUMBER.
                    /// See https://www.sqlite.org/lang_expr.html#parameters
                    let query_parameters = ids.iter().enumerate().map(|(index, _)| format!("?{}", index + 1)).collect::<Vec<String>>().join(", ");
                    let query = format!("SELECT id, name FROM stops WHERE id IN ({query_parameters})");
                    // dbg!(&query, libsql::ffi::SQLITE_VAR);
                    let result = state.connection.query(&query, ids).await;
                    let mut rows = match result {
                        Ok(rows) => rows,
                        Err(error) => {
                            error!("Error looking up stop names: {error}");
                            let template = IndexTemplate {
                                error: Some("Sorry, something on our side went wrong. Could not search for connections.".to_string()),
                                start: Some(start),
                                end: Some(end),
                                departure: try_format(&departure),
                                ..Default::default()
                            };

                            return template;
                        }
                    };

                    let mut names_by_id = HashMap::new();
                    loop {
                        let row = match rows.next().await {
                            Ok(Some(row)) => row,
                            Ok(None) => break,
                            Err(error) => {
                                error!("Error reading stop names rows: {error}");
                                let template = IndexTemplate {
                                    error: Some("Sorry, something on our side went wrong. Could not search for connections.".to_string()),
                                    start: Some(start),
                                    end: Some(end),
                                    departure: try_format(&departure),
                                    ..Default::default()
                                };

                                return template;
                            }
                        };

                        let id: String = match row.get(0) {
                            Ok(id) => id,
                            Err(error) => {
                                error!("Error reading id from stop names row: {error}");
                                let template = IndexTemplate {
                                    error: Some("Sorry, something on our side went wrong. Could not search for connections.".to_string()),
                                    start: Some(start),
                                    end: Some(end),
                                    departure: try_format(&departure),
                                    ..Default::default()
                                };

                                return template;
                            }
                        };

                        let name: String = match row.get(1) {
                            Ok(name) => name,
                            Err(error) => {
                                error!("Error reading name from stop names row: {error}");
                                let template = IndexTemplate {
                                    error: Some("Sorry, something on our side went wrong. Could not search for connections.".to_string()),
                                    start: Some(start),
                                    end: Some(end),
                                    departure: try_format(&departure),
                                    ..Default::default()
                                };

                                return template;
                            }
                        };

                        names_by_id.insert(id, name);
                    }

                    let mut results = Vec::new();

                    for round in rounds.into_iter() {
                        let mut output = Vec::new();
                        for (stop_id, connection) in round {
                            let raptor::Connection::Connection { route, trip_number, boarded_at_stop, exited_at_stop } = connection else {
                                error!("Unexpected connection type while constructing output");
                                continue;
                            };

                            let stop_name = names_by_id.get(&stop_id).unwrap();
                            let id = state.raptor_data.stops_data.stops[boarded_at_stop].id.clone();
                            let boarded = names_by_id.get(&id).unwrap();
                            let id = state.raptor_data.stops_data.stops[exited_at_stop].id.clone();
                            let exited = names_by_id.get(&id).unwrap();
                            output.push(ResultRow { stop_name: stop_name.to_string(), route, trip_number, boarded: boarded.to_string(), exited: exited.to_string() });
                        }

                        results.push(output);
                    }

                    let template = IndexTemplate {
                        start: Some(start),
                        end: Some(end),
                        departure: try_format(&departure),
                        results: Some(results),
                        ..Default::default()
                    };
                    template
                }
                (start_id, end_id) => {
                    let start_error = start_id.map_or(Some("Not found. Please try another one".to_string()), |_| None);;
                    let end_error = end_id.map_or(Some("Not found. Please try another one".to_string()), |_| None);;
                    let template = IndexTemplate {
                        start_error,
                        end_error,
                        start: Some(start),
                        end: Some(end),
                        departure: try_format(&departure),
                        ..Default::default()
                    };
                    template
                }
            }
        }
        SearchConnectionRequest {
            start,
            end,
            departure,
        } => IndexTemplate { start, end, departure: departure.as_ref().and_then(try_format), ..Default::default() },
    }
}


#[derive(serde::Deserialize)]
struct SearchStartRequest {
    start: String,
}

#[derive(serde::Deserialize)]
struct SearchEndRequest {
    end: String,
}

async fn search_stops(connection: &libsql::Connection, query: &str) -> impl IntoResponse {
    if query.is_empty() {
        return Html(String::default()).into_response();
    }

    let result = connection.query(
        "SELECT name FROM stop_names WHERE stop_names MATCH concat('name:', :query) ORDER BY rank;",
        libsql::named_params! {":query": format!("\"{}\"", query) }).await;

    let mut rows = match result {
        Ok(rows) => rows,
        Err(error) => {
            error!("Error searching for stops: {error}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };


    let mut options_html = String::new();
    loop {
        let row = match rows.next().await {
            Ok(Some(row)) => row,
            Ok(None) => break,
            Err(error) => {
                error!("Error reading search result rows: {error}");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };

        let stop_name = match row.get_str(0) {
            Ok(stop_name) => stop_name,
            Err(error) => {
                error!("Error reading search result row: {error}");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };

        options_html.push_str(&format!("<option value=\"{}\"></option>", stop_name));
    }

    Html(options_html).into_response()
}

async fn search_end_stops(State(state): State<AppState>, Form(request): Form<SearchEndRequest>) -> impl IntoResponse {
    search_stops(&state.connection, &request.end).await
}

async fn search_start_stops(State(state): State<AppState>, Form(request): Form<SearchStartRequest>) -> impl IntoResponse {
    debug!("Searching for stops");

    search_stops(&state.connection, &request.start).await
}
