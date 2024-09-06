use benchmark::fibonacci;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use raptor::shared::{RoutesData, StopsData};
use raptor::{raptor, raptor_bugged, Time};
use sql2raptor::{setup_raptor, RaptorDataSet};
use std::time::Duration;

fn setup(start: &str, end: &str) -> (usize, usize, Time, RoutesData, StopsData) {
    // Quick and dirty
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let data = runtime.block_on(async {
        let database = libsql::Builder::new_local("gtfs.db").build().await.unwrap();
        let connection = database.connect().unwrap();

        setup_raptor(&connection).await.unwrap()
    });

    let RaptorDataSet {
        index_by_stop_id,
        routes_data,
        stops_data,
    } = data;
    let departure = Time::from(12 * 60 * 60);
    let source_index = *index_by_stop_id.get(start).unwrap();
    let target_index = *index_by_stop_id.get(end).unwrap();

    (
        source_index,
        target_index,
        departure,
        routes_data,
        stops_data,
    )
}

pub fn benchmark(criterion: &mut Criterion) {
    // Prepare data
    // Haven't found a better solution than to create a temporary async runtime
    // to call an async setup function

    let mut group = criterion.benchmark_group("raptor");
    group.bench_function("first case", |bencher| {
        bencher.iter_batched(
            || setup("1808", "1811"),
            |(source_index, target_index, departure, routes_data, stops_data)| {
                raptor(
                    source_index,
                    target_index,
                    &departure,
                    routes_data,
                    stops_data,
                )
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("first case (bugged)", |bencher| {
        bencher.iter_batched(
            || setup("1808", "1811"),
            |(source_index, target_index, departure, routes_data, stops_data)| {
                raptor_bugged(
                    source_index,
                    target_index,
                    &departure,
                    routes_data,
                    stops_data,
                )
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("second case", |bencher| {
        bencher.iter_batched(
            || setup("687", "2"),
            |(source_index, target_index, departure, routes_data, stops_data)| {
                raptor(
                    source_index,
                    target_index,
                    &departure,
                    routes_data,
                    stops_data,
                )
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("second case (bugged)", |bencher| {
        bencher.iter_batched(
            || setup("687", "2"),
            |(source_index, target_index, departure, routes_data, stops_data)| {
                raptor_bugged(
                    source_index,
                    target_index,
                    &departure,
                    routes_data,
                    stops_data,
                )
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
