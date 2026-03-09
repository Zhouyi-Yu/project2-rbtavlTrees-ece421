// =============================================================================
// benchmark.rs — Performance Benchmarking
// =============================================================================
//
// This module times insert and search operations on both trees for
// various tree sizes, then prints a formatted results table.
//
// TEST METHODOLOGY (as specified in the project):
//   For each tree_size in [10_000, 40_000, 70_000, 100_000, 130_000]:
//     1. Insert tree_size elements using specific input sources
//     2. Search for the N/10 values
//     3. Record insert time and search time separately.
//     4. Average over multiple runs for accuracy.
// =============================================================================

use moneymaker_avl::AVLTree;
use moneymaker_rbt::RedBlackTree;
use std::time::{Duration, Instant};

use rand::prelude::*;
use rand::rngs::Xoshiro256PlusPlus;

const RUNS: u32 = 3;
const SIZES: &[u32] = &[10_000, 40_000, 70_000, 100_000, 130_000];
const BENCH_SEED: u64 = 69_420_1337_123456789;

pub struct BenchResult {
    pub tree_size: u32,
    pub input_source: &'static str,
    pub rbt_insert_ms: f64,
    pub avl_insert_ms: f64,
    pub rbt_search_ms: f64,
    pub avl_search_ms: f64,
}

#[derive(Copy, Clone)]
enum InputSource {
    Ascending,
    Random,
}

impl InputSource {
    fn name(&self) -> &'static str {
        match self {
            InputSource::Ascending => "Ascending",
            InputSource::Random => "Random",
        }
    }

    fn iter(&self, size: u32) -> Box<dyn Iterator<Item = u32>> {
        match self {
            InputSource::Ascending => Box::new(1..=size),
            InputSource::Random => {
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(BENCH_SEED);
                Box::new((0..size).map(move |_| rng.random::<u32>()))
            }
        }
    }
}

pub fn run_benchmarks() {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║              BENCHMARK: Insert & Search Performance              ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║ Methodology: Insert N values for each input source type          ║");
    println!("║              then search for N/10 of those values.               ║");
    println!(
        "║ Each test averaged over {:<2} runs.                                 ║",
        RUNS
    );
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let mut results: Vec<BenchResult> = Vec::new();
    let sources = [InputSource::Ascending, InputSource::Random];

    for &size in SIZES {
        println!("▶ Benchmarking tree size: {}", size);

        for source in sources {
            let rbt_insert = bench_rbt_insert(source, size);
            let rbt_search = bench_rbt_search(source, size);
            let avl_insert = bench_avl_insert(source, size);
            let avl_search = bench_avl_search(source, size);

            results.push(BenchResult {
                tree_size: size,
                input_source: source.name(),
                rbt_insert_ms: to_ms(rbt_insert),
                avl_insert_ms: to_ms(avl_insert),
                rbt_search_ms: to_ms(rbt_search),
                avl_search_ms: to_ms(avl_search),
            });
        }
    }

    print_results_table(&results);
    print_analysis(&results);
}

fn bench_rbt_insert(source: InputSource, size: u32) -> Duration {
    let mut total = Duration::ZERO;
    for _ in 0..RUNS {
        let mut tree = RedBlackTree::new();
        let start = Instant::now();
        for i in source.iter(size) {
            tree.insert(i);
        }
        total += start.elapsed();
    }
    total / RUNS
}

fn bench_rbt_search(source: InputSource, size: u32) -> Duration {
    let mut tree = RedBlackTree::new();
    for i in source.iter(size) {
        tree.insert(i);
    }
    let search_count = (size / 10) as usize;

    let mut total = Duration::ZERO;
    for _ in 0..RUNS {
        let start = Instant::now();
        for i in source.iter(size).take(search_count) {
            let _ = tree.search(i);
        }
        total += start.elapsed();
    }
    total / RUNS
}

fn bench_avl_insert(source: InputSource, size: u32) -> Duration {
    let mut total = Duration::ZERO;
    for _ in 0..RUNS {
        let mut tree = AVLTree::new();
        let start = Instant::now();
        for i in source.iter(size) {
            tree.insert(i);
        }
        total += start.elapsed();
    }
    total / RUNS
}

fn bench_avl_search(source: InputSource, size: u32) -> Duration {
    let mut tree = AVLTree::new();
    for i in source.iter(size) {
        tree.insert(i);
    }
    let search_count = (size / 10) as usize;

    let mut total = Duration::ZERO;
    for _ in 0..RUNS {
        let start = Instant::now();
        for i in source.iter(size).take(search_count) {
            let _ = tree.search(i);
        }
        total += start.elapsed();
    }
    total / RUNS
}

fn to_ms(d: Duration) -> f64 {
    d.as_secs_f64() * 1000.0
}

fn print_results_table(results: &[BenchResult]) {
    println!("\n┌────────────┬───────────┬───────────────────────┬───────────────────────┐");
    println!("│  Tree Size │   Input   │      Insert (ms)      │      Search (ms)      │");
    println!("│            │           │       RBT │       AVL │       RBT │       AVL │");
    println!("├────────────┼───────────┼───────────┼───────────┼───────────┼───────────┤");
    for r in results {
        println!(
            "│ {:>10} │ {:>9} │ {:>9.3} │ {:>9.3} │ {:>9.3} │ {:>9.3} │",
            r.tree_size,
            r.input_source,
            r.rbt_insert_ms,
            r.avl_insert_ms,
            r.rbt_search_ms,
            r.avl_search_ms
        );
    }
    println!("└────────────┴───────────┴───────────┴───────────┴───────────┴───────────┘");
}

fn print_analysis(results: &[BenchResult]) {
    println!("\n── Analysis ──────────────────────────────────────────────────────────");

    let rbt_insert_total: f64 = results.iter().map(|r| r.rbt_insert_ms).sum();
    let avl_insert_total: f64 = results.iter().map(|r| r.avl_insert_ms).sum();
    let rbt_search_total: f64 = results.iter().map(|r| r.rbt_search_ms).sum();
    let avl_search_total: f64 = results.iter().map(|r| r.avl_search_ms).sum();

    println!(
        "Insert:  RBT total = {:.3}ms  |  AVL total = {:.3}ms  →  {} is faster",
        rbt_insert_total,
        avl_insert_total,
        if rbt_insert_total < avl_insert_total {
            "RBT"
        } else {
            "AVL"
        }
    );
    println!(
        "Search:  RBT total = {:.3}ms  |  AVL total = {:.3}ms  →  {} is faster",
        rbt_search_total,
        avl_search_total,
        if rbt_search_total < avl_search_total {
            "RBT"
        } else {
            "AVL"
        }
    );

    println!("\nExpected result:");
    println!("  • RBT is typically faster at INSERT (fewer rotations needed).");
    println!("  • AVL is typically faster at SEARCH (more strictly balanced,");
    println!("    so tree height is slightly shorter → fewer comparisons).");
    println!("─────────────────────────────────────────────────────────────────────");
}
