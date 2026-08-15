use cspuz_core::integration::{reset_thread_local_perf_stats, thread_local_perf_stats};
use cspuz_rs::graph::BoolGridEdgesIrrefutableFacts as CspuzRsBoolGridEdgesIrrefutableFacts;
use cspuz_rs::graph::BoolInnerGridEdgesIrrefutableFacts as CspuzRsBoolInnerGridEdgesIrrefutableFacts;
use serde::{Deserialize, Serialize};

mod dbchoco;
mod slitherlink;
mod yajilin;

#[derive(Serialize, Deserialize)]
pub struct BoolGridEdgesIrrefutableFacts {
    pub horizontal: Vec<Vec<Option<bool>>>,
    pub vertical: Vec<Vec<Option<bool>>>,
}

impl BoolGridEdgesIrrefutableFacts {
    pub fn to_cspuz_rs(&self) -> CspuzRsBoolGridEdgesIrrefutableFacts {
        CspuzRsBoolGridEdgesIrrefutableFacts {
            horizontal: self.horizontal.clone(),
            vertical: self.vertical.clone(),
        }
    }

    pub fn from_cspuz_rs(facts: &CspuzRsBoolGridEdgesIrrefutableFacts) -> Self {
        Self {
            horizontal: facts.horizontal.clone(),
            vertical: facts.vertical.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct BoolInnerGridEdgesIrrefutableFacts {
    pub horizontal: Vec<Vec<Option<bool>>>,
    pub vertical: Vec<Vec<Option<bool>>>,
}

impl BoolInnerGridEdgesIrrefutableFacts {
    pub fn to_cspuz_rs(&self) -> CspuzRsBoolInnerGridEdgesIrrefutableFacts {
        CspuzRsBoolInnerGridEdgesIrrefutableFacts {
            horizontal: self.horizontal.clone(),
            vertical: self.vertical.clone(),
        }
    }

    pub fn from_cspuz_rs(facts: &CspuzRsBoolInnerGridEdgesIrrefutableFacts) -> Self {
        Self {
            horizontal: facts.horizontal.clone(),
            vertical: facts.vertical.clone(),
        }
    }
}

pub struct BenchResult {
    pub elapsed_time_seconds: f64,
    pub sat_num_propagations: f64,
}

fn accumulate(results: &[BenchResult]) -> BenchResult {
    let elapsed_time_seconds = results.iter().map(|r| r.elapsed_time_seconds).sum();
    let sat_num_propagations = results.iter().map(|r| r.sat_num_propagations).sum();
    BenchResult {
        elapsed_time_seconds,
        sat_num_propagations,
    }
}

#[derive(Debug)]
pub enum BenchmarkError {
    AnswerMismatch,
}

fn run_with_bench_result<F, T>(f: F) -> (T, BenchResult)
where
    F: FnOnce() -> T,
{
    reset_thread_local_perf_stats();

    let orig_default_config = cspuz_core::config::Config::default();
    let updated_default_config = cspuz_core::config::Config {
        record_perf_stats_thread_local: true,
        ..orig_default_config
    };
    cspuz_core::config::Config::set_default(updated_default_config);

    let start = std::time::Instant::now();
    let answer = f();
    let elapsed_time_seconds = start.elapsed().as_secs_f64();

    let perf_stats = thread_local_perf_stats();
    let sat_num_propagations = perf_stats.propagations();

    cspuz_core::config::Config::set_default(orig_default_config);

    (
        answer,
        BenchResult {
            elapsed_time_seconds,
            sat_num_propagations: sat_num_propagations as f64,
        },
    )
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Task {
    DoubleChoco(dbchoco::DoubleChocoTask),
    Slitherlink(slitherlink::SlitherlinkTask),
    Yajilin(yajilin::YajilinTask),
}

pub fn run_benchmark(task: &Task) -> Result<BenchResult, BenchmarkError> {
    match task {
        Task::DoubleChoco(task) => dbchoco::run_benchmark(task),
        Task::Slitherlink(task) => slitherlink::run_benchmark(task),
        Task::Yajilin(task) => yajilin::run_benchmark(task),
    }
}

#[derive(Serialize, Deserialize)]
struct BaseBenchmarkSet {
    solve_tasks: Vec<BaseSolveTask>,
    generate_tasks: Vec<BaseGenerateTaskSet>,
}

#[derive(Serialize, Deserialize)]
struct BaseSolveTask {
    name: String,
    puzzle_type: String,
    url: String,
    comment: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct BaseGenerateTaskSet {
    name: String,
    puzzle_type: String,
    height: usize,
    width: usize,
    seed: u64,
}

#[derive(Serialize, Deserialize)]
struct BenchmarkSet {
    solve_tasks: Vec<SolveTask>,
    generate_tasks: Vec<GenerateTaskSet>,
}

#[derive(Serialize, Deserialize)]
struct SolveTask {
    name: String,
    puzzle_type: String,
    task: Task,
}

#[derive(Serialize, Deserialize)]
struct GenerateTaskSet {
    name: String,
    puzzle_type: String,
    tasks: Vec<Task>,
}

fn materialize_benchmark_set(base: BaseBenchmarkSet) -> BenchmarkSet {
    let solve_tasks = base
        .solve_tasks
        .into_iter()
        .map(|base_task| {
            let task = match base_task.puzzle_type.as_str() {
                "dbchoco" => dbchoco::materialize_solve_task(&base_task.url),
                "slitherlink" => slitherlink::materialize_solve_task(&base_task.url),
                "yajilin" => yajilin::materialize_solve_task(&base_task.url),
                _ => panic!("Unknown puzzle type: {}", base_task.puzzle_type),
            };
            SolveTask {
                name: base_task.name,
                puzzle_type: base_task.puzzle_type,
                task,
            }
        })
        .collect();

    let generate_tasks = base
        .generate_tasks
        .into_iter()
        .map(|base_task| match base_task.puzzle_type.as_str() {
            "slitherlink" => slitherlink::materialize_generate_task(base_task),
            _ => panic!("Unknown puzzle type: {}", base_task.puzzle_type),
        })
        .collect();

    BenchmarkSet {
        solve_tasks,
        generate_tasks,
    }
}

pub fn run_materialize(src_path: &str, dest_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let base_benchmark_set: BaseBenchmarkSet =
        serde_json::from_reader(std::fs::File::open(src_path)?)?;
    let benchmark_set = materialize_benchmark_set(base_benchmark_set);
    let dest_file = std::fs::File::create(dest_path)?;
    let mut encoder = zstd::stream::write::Encoder::new(dest_file, 0)?;
    serde_json::to_writer(&mut encoder, &benchmark_set)?;
    encoder.finish()?;
    Ok(())
}

pub fn run_benchmarks(src_path: &str) {
    let src_file = std::fs::File::open(src_path).unwrap();
    let decoder = zstd::stream::read::Decoder::new(src_file).unwrap();
    let benchmark_set: BenchmarkSet = serde_json::from_reader(decoder).unwrap();

    for solve_task in &benchmark_set.solve_tasks {
        println!("Running solve benchmark: {}", solve_task.name);
        let result = run_benchmark(&solve_task.task).unwrap();
        println!(
            "Elapsed time: {:.3} seconds, SAT propagations: {}",
            result.elapsed_time_seconds, result.sat_num_propagations
        );
    }

    for generate_task_set in &benchmark_set.generate_tasks {
        println!("Running generate benchmark set: {}", generate_task_set.name);

        let mut results = vec![];
        for task in &generate_task_set.tasks {
            let result = run_benchmark(task).unwrap();
            results.push(result);
        }
        let accumulated_result = accumulate(&results);
        println!(
            "Accumulated result for generate benchmark set {}: Elapsed time: {:.3} seconds, SAT propagations: {}",
            generate_task_set.name, accumulated_result.elapsed_time_seconds, accumulated_result.sat_num_propagations
        );
    }
}
