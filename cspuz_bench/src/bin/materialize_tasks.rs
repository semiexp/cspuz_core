fn main() {
    cspuz_bench::benchmarks::run_materialize(
        concat!(env!("CARGO_MANIFEST_DIR"), "/tasks/base.json"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/tasks/bench.json"),
    )
    .unwrap();
}
