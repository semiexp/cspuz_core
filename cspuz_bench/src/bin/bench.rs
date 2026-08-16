fn main() {
    // cspuz_bench/tasks/base.json -> cspuz_bench/tasks/bench.json.zst
    // NOTE: relative to this file, not the current working directory

    cspuz_bench::benchmarks::run_benchmarks(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tasks/bench.json.zst"
    ));
}
