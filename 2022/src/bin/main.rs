use aoc_2022::{days::*, solver::Solver};
use std::{fmt::Debug, time::Instant};

const YEAR: &str = "2022";

#[cfg(debug_assertions)]
#[global_allocator]
static ALLOCATOR: dhat::Alloc = dhat::Alloc;

macro_rules! day {
	($d:expr) => {
		day!($d => None, None)
	};

	($d:expr, $o1:expr) => {
		day!($d => Some($o1), None)
	};

	($d:expr, $o1:expr, $o2:expr) => {
		day!($d => Some($o1), Some($o2))
	};

	($d:expr => $o1:expr, $o2:expr) => {
		paste::expr! {
			solve::<_, _, [<day $d>]::[<Day $d>]>($d, $o1, $o2)
		}
	};
}

fn main() {
	#[cfg(debug_assertions)]
	let _dhat = dhat::Profiler::new_heap();

	println!("AOC {YEAR}");

	day!(1, 72070, 211805);
	day!(2, 11475, 16862);
	day!(3, 7763, 2569);
	day!(4, 532, 854);
	day!(5, "QPJPLMNNR".into(), "BQDNWJPVJ".into());
	day!(6, 1142, 2803);
}

fn solve<O, O2, S: for<'a> Solver<'a, Output = O, Output2 = O2>>(
	day_number: u8,
	part1_output: Option<O>,
	part2_output: Option<O2>,
) {
	let input = std::fs::read_to_string(format!("../input/{YEAR}/day{day_number}.txt")).unwrap();
	let trimmed = input.trim();

	let mut args = std::env::args();
	if args.len() > 1 {
		let day_as_str = day_number.to_string();
		if args.any(|x| x == day_as_str || x == "a") {
			bench::<S>(day_number, trimmed);
		}
	} else {
		run::<S>(day_number, trimmed, part1_output, part2_output);
	}
}

fn run<'a, S: Solver<'a>>(
	day_number: u8,
	input: &'a str,
	part1_output: Option<S::Output>,
	part2_output: Option<S::Output2>,
) {
	let start_time = Instant::now();
	let parsed = S::parse(input);
	let end_time = Instant::now();

	println!("\nDay {day_number}:");
	println!("\tparser: {:?}", (end_time - start_time));

	run_part(parsed.clone(), 1, S::part1, part1_output);
	run_part(parsed, 2, S::part2, part2_output);
}

fn run_part<P, O: Debug + PartialEq>(
	parsed: P,
	part_number: u8,
	part: impl Fn(P) -> O,
	expected_output: Option<O>,
) {
	print!("Part {part_number}: ");

	let start_time = Instant::now();
	let result = part(parsed);
	let end_time = Instant::now();

	println!("{result:?}");
	println!("\tsolver: {:?}", (end_time - start_time));

	if let Some(expected) = expected_output {
		assert_eq!(expected, result);
	} else {
		println!("Not checking result!");
	}
}

fn bench<'a, S: Solver<'a>>(day_number: u8, input: &'a str) {
	let mut criterion = criterion::Criterion::default().without_plots();
	let mut group = criterion.benchmark_group(format!("Day {day_number}"));

	group.bench_with_input("parser", &input, |b, i| {
		b.iter_with_large_drop(|| S::parse(i));
	});

	let parsed = S::parse(input);

	group.bench_with_input("part 1", &parsed, |b, i| {
		b.iter_batched(|| i.clone(), S::part1, criterion::BatchSize::SmallInput)
	});

	group.bench_with_input("part 2", &parsed, |b, i| {
		b.iter_batched(|| i.clone(), S::part2, criterion::BatchSize::SmallInput)
	});
}
