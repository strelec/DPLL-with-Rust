extern crate solver;

use self::solver::*;

pub struct Sudoku {
	pub size: usize,
	pub data: Vec<usize>,
}

impl Sudoku {
	pub fn to_cnf(self) -> CNF {
		let size = self.size;
		let n = size * size;
		assert!(self.data.len() == n*n);
		
		// Step 1: Lambdas to get a specific cell, row, column or block
		let u = |r: usize, c: usize, d: usize| {
			r*n*n + c*n + d + 1
		};
		
		let cell = |r: usize, c: usize| {
			(0..n).map( |d| u(r, c, d) ).collect::<Vec<_>>()
		};
		let row = |r: usize, d: usize| {
			(0..n).map( |c| u(r, c, d) ).collect::<Vec<_>>()
		};
		let column = |c: usize, d: usize| {
			(0..n).map( |r| u(r, c, d) ).collect::<Vec<_>>()
		};
		let block = |b: usize, d: usize| {
			(0..n).map( |f|
				u(b/size*size + f/size, b%size*size + f%size, d)
			).collect::<Vec<_>>()
		};

		// Step 2: Collect all the elements
		fn collect<F>(n: usize, f: F) -> Vec<Vec<usize>>
		where F: Fn(usize, usize) -> Vec<usize> {
			(0..n).flat_map( |i|
				(0..n).map( |j| f(i, j) ).collect::<Vec<_>>()
			).collect()
		}
		
		let cells   = collect(n, cell);
		let rows    = collect(n, row);
		let columns = collect(n, column);
		let blocks  = collect(n, block);
		
		// Step 3: Encode the given digits
		let digits = self.data.iter().enumerate().flat_map( |(i, d)|
			if *d == 0 { None } else {
				Some(Clause { t: vec![u(i/n, i%n, d-1)], f: Vec::new() })
			}
		).collect::<Vec<_>>();
		
		// Step 4: Define quantificators
		fn at_least_one(d: &Vec<Vec<usize>>) -> Vec<Clause> {
			d.iter().map( |x|
				Clause { t: x.clone(), f: Vec::new() }
			).collect::<Vec<_>>()
		}
		
		fn at_most_one(d: &Vec<Vec<usize>>) -> Vec<Clause> {
			d.iter().flat_map( |x|
				x.iter().enumerate().flat_map( |(i, d1)|
					x.iter().skip(i+1).map( |d2|
						Clause { t: Vec::new(), f: vec![*d1, *d2] }
					).collect::<Vec<_>>()
				).collect::<Vec<_>>()
			).collect::<Vec<_>>()
		}
		
		// Step 5: Generate the CNF
		let mut o = digits;

		o.extend( at_least_one(&cells) );
		o.extend( at_most_one(&cells) );
		
		o.extend( at_least_one(&rows) );
		o.extend( at_most_one(&rows) );
		
		o.extend( at_least_one(&columns) );
		o.extend( at_most_one(&columns) );
		
		o.extend( at_least_one(&blocks) );
		o.extend( at_most_one(&blocks) );
		
		CNF::new(o)
	}
}
