use std::io;
use std::time::{Instant, Duration};
use rand::seq::SliceRandom;
use rand::Rng;
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;

// ANSI farby
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";

fn main() {
    clear_screen();
    println!("{}╔══════════════════════════════╗{}", CYAN, RESET);
    println!("{}║      SORTING MASTER 🎮       ║{}", CYAN, RESET);
    println!("{}╚══════════════════════════════╝{}", CYAN, RESET);

    println!("Enter your name:");
    let player = read_input();

    let mut score = 0;
    let mut level: i32 = 1;

    loop {
        clear_screen();
        print_header(&player, score, level);

        // ✅ FIX TU
        let size = (5 + level) as usize;

        let mut numbers = generate_numbers(size);
        shuffle(&mut numbers);

        println!("{}Sort these numbers:{} ", YELLOW, RESET);
        print_numbers(&numbers);

        println!("\nChoose mode:");
        println!("1) Normal");
        println!("2) HARDCORE 💀 (10s limit)");

        let mode = read_input();
        let hardcore = mode.trim() == "2";

        println!("\nChoose algorithm:");
        println!("1) QuickSort");
        println!("2) BubbleSort");

        let algo = read_input();

        let mut correct = numbers.clone();

        match algo.trim() {
            "1" => quick_sort(&mut correct),
            "2" => bubble_sort(&mut correct),
            _ => {
                println!("Invalid choice!");
                continue;
            }
        }

        println!("\nEnter sorted numbers:");
        let start = Instant::now();

        let input = read_input();
        let duration = start.elapsed();

        if hardcore && duration > Duration::from_secs(10) {
            println!("{}⏰ TIME OUT!{}", RED, RESET);
            score = score.saturating_sub(10);
            continue;
        }

        let user: Vec<i32> = input
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        println!("\nCorrect:");
        print_numbers(&correct);

        if user == correct {
            let points = calculate_score(duration, hardcore, level);
            println!("{}✅ Correct! +{} points{}", GREEN, points, RESET);
            score += points;
            level += 1;
        } else {
            println!("{}❌ Wrong!{}", RED, RESET);
            score = score.saturating_sub(5);
        }

        println!("\nContinue? (y/n)");
        if read_input() != "y" {
            break;
        }
    }

    save_score(&player, score);

    println!("\n{}🏆 LEADERBOARD{}", CYAN, RESET);
    show_leaderboard();
}

// ---------------- GAME SYSTEM ----------------

fn print_header(player: &str, score: i32, level: i32) {
    println!("{}Player:{} {}", CYAN, RESET, player);
    println!("{}Score:{} {}", CYAN, RESET, score);
    println!("{}Level:{} {}", CYAN, RESET, level);
    println!("-----------------------------------");
}

fn generate_numbers(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(1..200)).collect()
}

fn shuffle(numbers: &mut Vec<i32>) {
    let mut rng = rand::thread_rng();
    numbers.shuffle(&mut rng);
}

fn calculate_score(time: Duration, hardcore: bool, level: i32) -> i32 {
    let base = if time.as_secs() < 5 { 20 } else { 10 };
    let hardcore_bonus = if hardcore { 20 } else { 0 };
    base + hardcore_bonus + level * 2
}

// ---------------- SORTING ----------------

fn quick_sort(arr: &mut Vec<i32>) {
    if arr.len() > 1 {
        let len = arr.len();
        quick_sort_helper(arr, 0, len - 1);
    }
}

fn quick_sort_helper(arr: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let pi = partition(arr, low, high);

        if pi > 0 {
            quick_sort_helper(arr, low, pi - 1);
        }
        quick_sort_helper(arr, pi + 1, high);
    }
}

fn partition(arr: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;

    for j in low..high {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, high);
    i
}

fn bubble_sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

// ---------------- IO ----------------

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn print_numbers(numbers: &Vec<i32>) {
    print!("[ ");
    for n in numbers {
        print!("{} ", n);
    }
    println!("]");
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

// ---------------- LEADERBOARD ----------------

fn save_score(name: &str, score: i32) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("leaderboard.txt")
        .unwrap();

    writeln!(file, "{} {}", name, score).unwrap();
}

fn show_leaderboard() {
    if let Ok(content) = read_to_string("leaderboard.txt") {
        let mut scores: Vec<(&str, i32)> = content
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() == 2 {
                    Some((parts[0], parts[1].parse().ok()?))
                } else {
                    None
                }
            })
            .collect();

        scores.sort_by(|a, b| b.1.cmp(&a.1));

        for (name, score) in scores.iter().take(5) {
            println!("{} - {}", name, score);
        }
    }
}
