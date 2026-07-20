use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cspuz_solver_backend::decode_and_solve;
use json::JsonValue;

const PROBLEMS: &str = include_str!("../problems.json");

fn problem_urls() -> JsonValue {
    json::parse(PROBLEMS).expect("benchmark problem data must be valid JSON")
}

fn bench_solve(c: &mut Criterion) {
    let problems = problem_urls();
    let mut group = c.benchmark_group("solve");

    for name in ["slitherlink", "nurikabe", "heyawake", "yajilin"] {
        let url = problems[name]
            .as_str()
            .expect("benchmark problem must be a pzpr URL");

        decode_and_solve(url.as_bytes()).expect("benchmark problem must be solvable");
        group.bench_function(name, |b| {
            b.iter(|| {
                decode_and_solve(black_box(url.as_bytes()))
                    .expect("benchmark problem must be solvable")
            })
        });
    }
}

criterion_group!(benches, bench_solve);
criterion_main!(benches);
