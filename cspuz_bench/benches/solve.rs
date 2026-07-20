use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use cspuz_rs_puzzles::puzzles::{heyawake, nurikabe, slitherlink, yajilin};
use cspuz_solver_backend::decode_and_solve;
use json::JsonValue;

const TEST_PROBLEMS: &str = include_str!("../problems.json");
const GENERATED_PROBLEMS: &str = include_str!("../generated_problems.json");

fn problem_urls(problems: &str) -> JsonValue {
    json::parse(problems).expect("benchmark problem data must be valid JSON")
}

fn generated_problem_urls<'a>(
    problems: &'a JsonValue,
    name: &str,
) -> impl Iterator<Item = &'a str> {
    problems[name].members().map(|problem| {
        problem
            .as_str()
            .expect("generated benchmark problem must be a pzpr URL")
    })
}

fn bench_solve_group(c: &mut Criterion, group_name: &str, problems: &str) {
    let problems = problem_urls(problems);
    let mut group = c.benchmark_group(group_name);

    for name in ["slitherlink", "nurikabe", "heyawake", "yajilin"] {
        let url = problems[name]
            .as_str()
            .expect("benchmark problem must be a pzpr URL");

        group.bench_function(name, |b| {
            b.iter(|| {
                decode_and_solve(black_box(url.as_bytes()))
                    .expect("benchmark problem must be solvable")
            })
        });
    }
}

fn bench_solve(c: &mut Criterion) {
    bench_solve_group(c, "solve", TEST_PROBLEMS);
    bench_solve_generated(c);
}

fn bench_solve_generated(c: &mut Criterion) {
    let problems = problem_urls(GENERATED_PROBLEMS);
    let mut group = c.benchmark_group("solve_generated");

    for (i, url) in generated_problem_urls(&problems, "slitherlink").enumerate() {
        let (full, problem) =
            slitherlink::deserialize_problem(url).expect("benchmark problem must be valid");
        group.bench_with_input(
            BenchmarkId::new("slitherlink", i),
            &problem,
            |b, problem| b.iter(|| slitherlink::solve_slitherlink(full, black_box(problem))),
        );
    }

    for (i, url) in generated_problem_urls(&problems, "nurikabe").enumerate() {
        let problem = nurikabe::deserialize_problem(url).expect("benchmark problem must be valid");
        group.bench_with_input(BenchmarkId::new("nurikabe", i), &problem, |b, problem| {
            b.iter(|| nurikabe::solve_nurikabe(black_box(problem)))
        });
    }

    for (i, url) in generated_problem_urls(&problems, "heyawake").enumerate() {
        let (borders, clues) =
            heyawake::deserialize_problem(url).expect("benchmark problem must be valid");
        group.bench_with_input(
            BenchmarkId::new("heyawake", i),
            &(borders, clues),
            |b, (borders, clues)| {
                b.iter(|| heyawake::solve_heyawake(black_box(borders), black_box(clues)))
            },
        );
    }

    for (i, url) in generated_problem_urls(&problems, "yajilin").enumerate() {
        let (outside, problem) =
            yajilin::deserialize_problem(url).expect("benchmark problem must be valid");
        group.bench_with_input(BenchmarkId::new("yajilin", i), &problem, |b, problem| {
            b.iter(|| yajilin::solve_yajilin(outside, black_box(problem)))
        });
    }
}

criterion_group!(benches, bench_solve);
criterion_main!(benches);
