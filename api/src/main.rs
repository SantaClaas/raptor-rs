extern crate core;

use std::env;
use std::env::{current_dir, current_exe};
use std::net::Ipv4Addr;
use askama_axum::Template;
use axum::response::{Html, IntoResponse};
use axum::{Form, Router};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use raptor::data::{assemble_raptor_data, get_routes, get_stops, GetStopsReturn};
use raptor::{raptor, Time};
use rusqlite::{named_params, Connection};
use tower_http::services::ServeDir;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use log::debug;
use serde::Deserialize;
use tracing::error;

fn old_main() {
    let connection = Connection::open("gtfs.db").unwrap();

    let GetStopsReturn {
        transfers,
        stops,
        index_by_stop_id,
    } = get_stops(&connection).unwrap();

    let step_2_result = get_routes(
        &connection,
        //TODO remove debug clone clown
        index_by_stop_id.clone(),
    )
        .unwrap();

    //TODO check if trip ids are needed
    let (routes_data, stops_data, _trip_ids) =
        assemble_raptor_data(step_2_result, stops, transfers);

    let dream_source_stop_id = "1808";
    let dream_target_stop_id = "1811";
    let source_index = *index_by_stop_id.get(dream_source_stop_id).unwrap();
    let target_index = *index_by_stop_id.get(dream_target_stop_id).unwrap();
    let departure = Time::from(12 * 60 * 60);

    //TODO remove clown copy
    let stops = stops_data.stops.clone();
    let results = raptor(
        source_index,
        target_index,
        &departure,
        routes_data,
        stops_data,
    );

    let mut statement = connection
        .prepare("SELECT name FROM stops WHERE id = :id")
        .unwrap();

    let mut get_stop_name = |stop_index: usize| -> String {
        let stop_id = &stops[stop_index].id;
        statement
            .query_row(named_params! {":id": stop_id}, |row| {
                row.get::<_, String>("name")
            })
            .unwrap()
    };

    let from = get_stop_name(source_index);
    let to = get_stop_name(target_index);
    println!("From {from} to {to}");

    let mut round = 1;
    for result in results {
        println!("Round {round} reached stops...");
        for (stop_index, connection) in result {
            let stop_name = get_stop_name(stop_index);

            print!("\t{stop_name}:\t");
            match connection {
                raptor::Connection::Connection {
                    route,
                    trip_number,
                    boarded_at_stop,
                    exited_at_stop,
                } => {
                    let boarded = get_stop_name(boarded_at_stop);
                    let exited = get_stop_name(exited_at_stop);
                    println!("Route {route} Trip {trip_number} Connection {boarded} -> {exited}");
                }
                raptor::Connection::FootPath { .. } => {}
            }
        }
        println!();

        round += 1;
    }
}

#[derive(Clone)]
struct AppState {
    connection: libsql::Connection,
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
    let state = AppState { connection };


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


#[derive(Template, Default)]
#[template(path = "index.html")]
struct IndexTemplate {
    error: Option<String>,
    start: Option<String>,
    end: Option<String>,
    start_error: Option<String>,
    end_error: Option<String>,
}


#[derive(serde::Deserialize)]
struct SearchConnectionRequest {
    start: Option<String>,
    end: Option<String>,
}

async fn get_stop_id(connection: &libsql::Connection, stop_name: &str) -> Result<Option<String>, libsql::Error> {
    let mut rows = connection.query(
        "SELECT id FROM stops WHERE name = :name", libsql::named_params!(":name": stop_name)).await?;

    let row = rows.next().await?;
    row.map(|row| row.get::<String>(0)).transpose()
}

async fn index(State(state): State<AppState>, Query(request): Query<SearchConnectionRequest>) -> impl IntoResponse {

    match request {
        SearchConnectionRequest {
            start: Some(start),
            end: Some(end),
        } => {
            let start_result = get_stop_id(&state.connection, &start).await;
            // Don't proceed if the first already failed
            let Ok(start_id) = start_result else {
                error!("Error searching for start stop: {start_result:?}");
                let template = IndexTemplate {
                    error: Some("Sorry, something on our side went wrong. Could not search for connections.".to_string()),
                    start: Some(start),
                    end: Some(end),
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
                    ..Default::default()
                };
                return template;
            };

            match (start_id, end_id) {
                (Some(start_id), Some(end_id)) => {
                    //TODO search for connections
                    let template = IndexTemplate {
                        start: Some(start),
                        end: Some(end),
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
                        ..Default::default()
                    };
                    template
                }
            }
        }
        SearchConnectionRequest {
            start,
            end
        } => IndexTemplate { start, end, ..Default::default() },
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
        libsql::named_params! {":query": query }).await;

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
