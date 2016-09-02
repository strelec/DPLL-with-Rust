extern crate bit_set;

use std::cmp::Ordering::*;
use std::fs::File;
use std::io;

mod solver;
use solver::*;

fn read_input<T: io::BufRead + Sized>(source: T) -> CNF {
	CNF::new(source.lines().flat_map( |x| {
		let y = x.unwrap();
		let line = y.trim();
		match line.chars().nth(0).unwrap() {
			'c' | 'p' => None,
			_ => {
				let mut t = Bag::new();
				let mut f = Bag::new();
				for v in line.split_whitespace() {
					let n: i32 = v.parse::<i32>().unwrap();
					match n.cmp(&0) {
						Equal   => break,
						Greater => t.push(n as usize),
						Less    => f.push(-n as usize)
					};
				}
				Some(Clause { t: t, f: f })
			}
		}
	}).collect())
}

fn read_stdin() -> CNF {
	let stdin = io::stdin();
	let locked = stdin.lock();
	read_input(locked)
}

fn read_file(name: String) -> CNF {
	let file = io::BufReader::new(File::open(name).unwrap());
	read_input(file)
}

fn main() {
	let mut formula = read_stdin();
	let formula_copy = formula.clone();

	match formula.dpll(&Set::new(), &Set::new()) {
		Some(solution) => {
			println!("{:?}", solution);
			// Assert that the solution is correct.
			assert!(formula_copy.validate(&solution))
		}
		None => println!("There is no solution.")
	}
}
