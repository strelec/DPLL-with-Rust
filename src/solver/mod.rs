pub use self::clause::*;

mod clause;

use std::path::Path;

pub struct CNF {
	formula: Vec<Clause>,
	mask: Vec<bool>,
	history: Vec<usize>,
	count_t: Vec<u16>,
	count_f: Vec<u16>,
}

impl CNF {
	pub fn dpll(&mut self, t: &Set, f: &Set) -> Option<Set> {
		let height = self.history.len();

		let mut known = t.len() + f.len();
		let mut t = t.clone();
		let mut f = f.clone();

		loop {
			// Step 1: Detect unit clauses
			for i in 0..self.formula.len() {
				if self.check_satisfied(i, &t, &f) { continue }

				match Clause::find_unit(&self.formula[i].t, &f, 2) {
					(0, _) =>
						match Clause::find_unit(&self.formula[i].f, &t, 2) {
							(0, _) => { self.pop_state(height); return None },
							(1, v) => { f.insert(v); self.mark_satisfied(i) },
							_ => {}
						},
					(1, v) =>
						match Clause::find_unit(&self.formula[i].f, &t, 1) {
							(0, _) => { t.insert(v); self.mark_satisfied(i) },
							_ => {}
						},
					_ => {}
				}
			}

			if self.all_satisfied() { return Some(t) }
			if !t.is_disjoint(&f) { self.pop_state(height); return None }

			// Step 2: Detect pure variables
			for i in 0..self.count_t.len() {
				let is_true = self.count_t[i] != 0;
				let is_false = self.count_f[i] != 0;
				if is_true && !is_false && !f.contains(i) { t.insert(i); }
				if is_false && !is_true && !t.contains(i) { f.insert(i); }
			}

			// Step 3: Check if we have produced any variables in this iteration
			let new_known = t.len() + f.len();
			assert!(new_known >= known);
			if new_known == known { break }
			known = new_known
		}

		// Step 4: Select the best variable to branch
		let branch_var = self.branching_strategy(&t, &f);

		t.insert(branch_var);
		if let Some(set) = self.dpll(&t, &f) {
			return Some(set)
		}

		t.remove(branch_var);
		f.insert(branch_var);
		if let Some(set) = self.dpll(&t, &f) {
			return Some(set)
		}

		// Step 5: Neither true nor false brach succeeded
		self.pop_state(height);
		None
	}

	pub fn new(formula: Vec<Clause>) -> CNF {
		let len = formula.len();

		let mut vlen = 0usize;
		for clause in &formula {
			for v in &clause.t { if *v > vlen { vlen = *v } }
			for v in &clause.f { if *v > vlen { vlen = *v } }
		}
		vlen += 1;

		let mut count_t = vec![0u16; vlen];
		let mut count_f = vec![0u16; vlen];
		for clause in &formula {
			for v in &clause.t { count_t[*v] += 1 }
			for v in &clause.f { count_f[*v] += 1 }
		}

		CNF {
			formula: formula,
			mask: vec![false; len],
			history: Vec::with_capacity(len),
			count_t: count_t,
			count_f: count_f,
		}
	}

	fn branching_strategy(&self, t: &Set, f: &Set) -> usize {
		// Select the most commonly occuring variable
		let mut count = 0u16;
		let mut result = 0usize;
		for i in 0..self.count_t.len() {
			let item = self.count_t[i] + self.count_f[i];
			if item > count && !t.contains(i) && !f.contains(i) {
				count = item;
				result = i;
			}
		}
		result
	}

	fn all_satisfied(&self) -> bool {
		self.history.len() == self.formula.len()
	}

	fn mark_satisfied(&mut self, i: usize) {
		self.mask[i] = true;
		self.history.push(i);
		for v in &self.formula[i].t { self.count_t[*v] -= 1 }
		for v in &self.formula[i].f { self.count_f[*v] -= 1 }
	}

	fn check_satisfied(&mut self, i: usize, t: &Set, f: &Set) -> bool {
		self.mask[i] || self.formula[i].eval(&t, &f) && {
			self.mark_satisfied(i); true
		}
	}

	fn pop_state(&mut self, height: usize) {
		for i in self.history[height..].into_iter() {
			self.mask[*i] = false;
			for v in &self.formula[*i].t { self.count_t[*v] += 1 }
			for v in &self.formula[*i].f { self.count_f[*v] += 1 }
		}
		self.history.truncate(height)
	}

	pub fn validate(&self, solution: &Set) -> bool {
		self.formula.iter().all( |c| c.eval_complete(solution) )
	}
	
	pub fn to_file(&self, path: &Path) {
		use std::error::Error;
		use std::io::prelude::*;
		use std::fs::File;
	
		let display = path.display();

		let mut file = match File::create(path) {
			Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
			Ok(file) => file,
		};
	
		file.write(format!("p cnf {} {}\n", self.count_t.len(), self.formula.len()).as_bytes());
		
		for clause in &self.formula {
			for v in &clause.t {
				file.write(format!("{} ", v).as_bytes());
			}
			for v in &clause.f {
				file.write(format!("-{} ", v).as_bytes());
			}
			file.write("0\n".as_bytes());
		}
	}
}
