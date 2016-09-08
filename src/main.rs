extern crate solver;

use std::cmp::Ordering::*;
use std::fs::File;
use std::io;
use std::env;

use solver::*;

fn read_input<T: io::BufRead + Sized>(source: T) -> CNF {
	let mut n = 0;
	let clauses = source.lines().flat_map( |x| {
		let line = x.expect("Error reading a source.");
		let elements: Vec<_> = line.trim().split_whitespace().collect();
		match elements.get(0) {
			None | Some(&"c") => None,
			Some(&"p") => {
				n = elements[2].parse::<usize>().expect("Expected number of variables.");
				None
			},
			_ => {
				let mut t = Bag::new();
				let mut f = Bag::new();
				for v in elements {
					let n: i32 = v.parse::<i32>().expect("Expected a number.");
					match n.cmp(&0) {
						Equal   => break,
						Greater => t.push(n as usize),
						Less    => f.push(-n as usize)
					};
				}
				Some(Clause { t: t, f: f })
			}
		}
	}).collect();
	CNF::new(clauses, n)
}

fn read_stdin() -> CNF {
	let stdin = io::stdin();
	let locked = stdin.lock();
	read_input(locked)
}

fn read_file(name: String) -> CNF {
	let file = io::BufReader::new(File::open(name).expect("File not found."));
	read_input(file)
}

fn main() {
	let mut formula = match env::args().nth(1) {
		Some(name) => read_file(name),
		None       => read_stdin()
	};

	match formula.dpll(&Set::new(), &Set::new()) {
		Some(solution) => {
			println!("{}", formula.variables().map( |v|
				if solution.contains(v) { v.to_string() } else { format!("-{}", v) }
			).collect::<Vec<_>>().join(" ") );

			// Assert that the solution is correct.
			assert!(formula.validate(&solution))
		}
		None => println!("There is no solution.")
	}
}
