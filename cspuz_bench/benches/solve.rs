use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cspuz_rs::graph::BoolGridEdgesIrrefutableFacts;
use cspuz_rs_puzzles::puzzles::{heyawake, nurikabe, slitherlink, yajilin};
use cspuz_solver_backend::decode_and_solve;
use json::JsonValue;

const TEST_PROBLEMS: &str = include_str!("../problems.json");
const GENERATED_PROBLEMS: &str = include_str!("../generated_problems.json");

fn problem_urls(problems: &str) -> JsonValue {
    json::parse(problems).expect("benchmark problem data must be valid JSON")
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

    let (full, problem) = slitherlink::deserialize_problem(
        problems["slitherlink"]
            .as_str()
            .expect("benchmark problem must be a pzpr URL"),
    )
    .expect("benchmark problem must be valid");
    assert_eq!(
        slitherlink::solve_slitherlink(full, &problem),
        Some(BoolGridEdgesIrrefutableFacts {
            horizontal: option_grid([[1, 0, 1], [0, 1, 0], [0, 1, 0], [1, 0, 1]]),
            vertical: option_grid([[1, 1, 1, 1], [1, 0, 0, 1], [1, 1, 1, 1]]),
        })
    );
    group.bench_function("slitherlink", |b| {
        b.iter(|| slitherlink::solve_slitherlink(full, black_box(&problem)))
    });

    let problem = nurikabe::deserialize_problem(
        problems["nurikabe"]
            .as_str()
            .expect("benchmark problem must be a pzpr URL"),
    )
    .expect("benchmark problem must be valid");
    assert_eq!(
        nurikabe::solve_nurikabe(&problem),
        Some(option_grid([[0, 1, 0], [1, 1, 0], [1, 0, 0],]))
    );
    group.bench_function("nurikabe", |b| {
        b.iter(|| nurikabe::solve_nurikabe(black_box(&problem)))
    });

    let (borders, clues) = heyawake::deserialize_problem(
        problems["heyawake"]
            .as_str()
            .expect("benchmark problem must be a pzpr URL"),
    )
    .expect("benchmark problem must be valid");
    assert_eq!(
        heyawake::solve_heyawake(&borders, &clues),
        Some(option_grid([[-1, -1], [-1, -1]]))
    );
    group.bench_function("heyawake", |b| {
        b.iter(|| heyawake::solve_heyawake(black_box(&borders), black_box(&clues)))
    });

    let (outside, problem) = yajilin::deserialize_problem(
        problems["yajilin"]
            .as_str()
            .expect("benchmark problem must be a pzpr URL"),
    )
    .expect("benchmark problem must be valid");
    assert_eq!(
        yajilin::solve_yajilin(outside, &problem)
            .expect("benchmark problem must be solvable")
            .1,
        option_grid([
            [0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
            [1, 0, 1, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0, 1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0, 0, 0, 0, 0, 1],
        ])
    );
    group.bench_function("yajilin", |b| {
        b.iter(|| yajilin::solve_yajilin(outside, black_box(&problem)))
    });
}

fn option_grid<const H: usize, const W: usize>(grid: [[i8; W]; H]) -> Vec<Vec<Option<bool>>> {
    grid.map(|row| {
        row.map(|cell| match cell {
            -1 => None,
            0 => Some(false),
            1 => Some(true),
            _ => unreachable!(),
        })
        .to_vec()
    })
    .to_vec()
}

criterion_group!(benches, bench_solve);
criterion_main!(benches);
