extern crate bit_set;

use std::cmp::Ordering::*;
use std::io;

mod solver;
use solver::*;

fn read_input<T: io::BufRead + Sized>(source: T) -> CNF {
	source.lines().flat_map( |x| {
		let y = x.unwrap();
		let line = y.trim();
		match line.chars().nth(0).unwrap() {
			'c' | 'p' => None,
			_ => {
				let mut t = Set::new();
				let mut f = Set::new();
				for v in line.split_whitespace() {
					let n: i32 = v.parse::<i32>().unwrap();
					match n.cmp(&0) {
						Equal => break,
						Greater => t.insert(n as usize),
						Less => f.insert(-n as usize)
					};
				}
				Some(Clause { t: t, f: f })
			}
		}
	}).collect()
}

fn main() {
	let stdin = io::stdin();
	let formula = read_input(stdin.lock());

	println!("{:?}", dpll(formula));
}
