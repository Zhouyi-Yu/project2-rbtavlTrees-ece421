// =============================================================================
// benchmark.rs — Performance Benchmarking
// =============================================================================
//
// This module times insert and search operations on both trees for
// various tree sizes, then prints a formatted results table.
//
// TEST METHODOLOGY (as specified in the project):
//   For each tree_size in [10_000, 40_000, 70_000, 100_000, 130_000]:
//     1. Insert tree_size elements in ascending order (1, 2, 3, ..., n)
//        This is the WORST CASE for a naive BST (would become a linked list)
//        but both AVL and RBT handle it gracefully.
//     2. Search for the (tree_size / 10) lowest values (1, 2, ..., n/10)
//        These are at the BOTTOM-LEFT of the tree — hardest to find.
//     3. Record insert time and search time separately.
//     4. Average over multiple runs for accuracy.
// =============================================================================

use moneymaker_avl::avl::AVLTree;
use moneymaker_rbt::rbt::RedBlackTree;
use std::time::{Duration, Instant};

use rand::rngs::Xoshiro256PlusPlus;
use rand::SeedableRng;

const RUNS: u32 = 3;

const SIZES: &[u32] = &[10_000, 40_000, 70_000, 100_000, 130_000];


pub struct BenchResult {
    pub tree_size: u32,
    pub tree_name: &'static str,
    pub insert_ms: f64,
    pub search_ms: f64,
}


pub fn run_benchmarks() {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║              BENCHMARK: Insert & Search Performance              ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║ Methodology: Insert N values in ascending order (worst-case BST) ║");
    println!("║              then search for the N/10 smallest values.           ║");
    println!(
        "║ Each test averaged over {} runs.                                  ║",
        RUNS
    );
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let mut results: Vec<BenchResult> = Vec::new();

    for &size in SIZES {
        println!("▶ Benchmarking tree size: {}", size);

        let rbt_insert = bench_rbt_insert(size);
        let rbt_search = bench_rbt_search(size);
        let avl_insert = bench_avl_insert(size);
        let avl_search = bench_avl_search(size);

        results.push(BenchResult {
            tree_size: size,
            tree_name: "RBT",
            insert_ms: to_ms(rbt_insert),
            search_ms: to_ms(rbt_search),
        });
        results.push(BenchResult {
            tree_size: size,
            tree_name: "AVL",
            insert_ms: to_ms(avl_insert),
            search_ms: to_ms(avl_search),
        });
    }

    print_results_table(&results);
    print_analysis(&results);
}


fn bench_rbt_insert(size: u32) -> Duration {
    let mut total = Duration::ZERO;
    for _ in 0..RUNS {
        let mut tree = RedBlackTree::new();
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(BENCH_SEED);
        let start = Instant::now();
        for i in 1..=size {
            tree.insert(i);
        }
        total += start.elapsed();
    }
    total / RUNS
}

const BENCH_SEED: u64 = 69_420_1337_123456789;

fn bench_rbt_search(size: u32) -> Duration {
    let mut tree = RedBlackTree::new();
    for i in 1..=size {
        tree.insert(i);
    }
    let search_count = size / 10;

    let mut total = Duration::ZERO;
    for _ in 0..RUNS {
        let start = Instant::now();
        for i in 1..=search_count {
            let _ = tree.search(i);
        }
        total += start.elapsed();
    }
    total / RUNS
}

fn bench_avl_insert(size: u32) -> Duration {
    let mut total = Duration::ZERO;
    for _ in 0..RUNS {
        let mut tree = AVLTree::new();
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(BENCH_SEED);
        let start = Instant::now();
        for i in 1..=size {
            tree.insert(i);
        }
        total += start.elapsed();
    }
    total / RUNS
}

fn bench_avl_search(size: u32) -> Duration {
    let mut tree = AVLTree::new();
    for i in 1..=size {
        tree.insert(i);
    }
    let search_count = size / 10;

    let mut total = Duration::ZERO;
    for _ in 0..RUNS {
        let start = Instant::now();
        for i in 1..=search_count {
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
    println!("\n┌────────────┬──────┬──────────────┬──────────────┐");
    println!("│  Tree Size │ Tree │  Insert (ms) │  Search (ms) │");
    println!("├────────────┼──────┼──────────────┼──────────────┤");
    for r in results {
        println!(
            "│ {:>10} │ {:>4} │ {:>12.3} │ {:>12.3} │",
            r.tree_size, r.tree_name, r.insert_ms, r.search_ms
        );
    }
    println!("└────────────┴──────┴──────────────┴──────────────┘");
}

fn print_analysis(results: &[BenchResult]) {
    println!("\n── Analysis ──────────────────────────────────────────────────────────");

    let rbt_insert_total: f64 = results
        .iter()
        .filter(|r| r.tree_name == "RBT")
        .map(|r| r.insert_ms)
        .sum();
    let avl_insert_total: f64 = results
        .iter()
        .filter(|r| r.tree_name == "AVL")
        .map(|r| r.insert_ms)
        .sum();
    let rbt_search_total: f64 = results
        .iter()
        .filter(|r| r.tree_name == "RBT")
        .map(|r| r.search_ms)
        .sum();
    let avl_search_total: f64 = results
        .iter()
        .filter(|r| r.tree_name == "AVL")
        .map(|r| r.search_ms)
        .sum();

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
