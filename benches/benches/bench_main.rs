mod benchmarks;

use criterion::criterion_main;

criterion_main! {
    benchmarks::always_success::process_block,
    benchmarks::secp_2in2out::process_block,
}
