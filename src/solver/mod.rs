pub use self::clause::*;

mod clause;

#[derive(Clone)]
pub struct CNF {
	formula: Vec<Clause>,
	mask: Vec<bool>,
	history: Vec<usize>,
	count_t: Vec<u16>, count_f: Vec<u16>,
}

impl CNF {
	pub fn new(formula: Vec<Clause>) -> CNF {
		let len = formula.len();

		let mut variable_count = 0usize;
		for clause in &formula {
			for v in &clause.t { if *v > variable_count { variable_count = *v } }
			for v in &clause.f { if *v > variable_count { variable_count = *v } }
		}

		let mut count_t = vec![0u16; variable_count+1];
		let mut count_f = vec![0u16; variable_count+1];
		for clause in &formula {
			for v in &clause.t { count_t[*v] += 1 }
			for v in &clause.f { count_f[*v] += 1 }
		}

		CNF {
			formula: formula,
			mask: vec![false; len],
			history: Vec::with_capacity(len),
			count_t: count_t, count_f: count_f,
		}
	}

	fn branching_strategy(self: &CNF, t: &Set, f: &Set) -> usize {
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

	fn find_unit(bag: &Bag, filter: &Set, up_to: u8) -> (u8, usize) {
		let mut count = 0u8;
		let mut var = 0;
		for &v in bag {
			if !filter.contains(v) {
				count += 1;
				var = v;
				if count == up_to { break }
			}
		}
		(count, var)
	}

	fn all_satisfied(self: &CNF) -> bool {
		self.history.len() == self.formula.len()
	}

	fn mark_satisfied(self: &mut CNF, i: usize) {
		self.mask[i] = true;
		self.history.push(i);
		for v in &self.formula[i].t { self.count_t[*v] -= 1 }
		for v in &self.formula[i].f { self.count_f[*v] -= 1 }
	}

	fn check_satisfied(self: &mut CNF, i: usize, t: &Set, f: &Set) -> bool {
		self.mask[i] || self.formula[i].eval(&t, &f) && {
			self.mark_satisfied(i); true
		}
	}

	fn pop_state(self: &mut CNF, height: usize) {
		for i in self.history[height..].into_iter() {
			self.mask[*i] = false;
			for v in &self.formula[*i].t { self.count_t[*v] += 1 }
			for v in &self.formula[*i].f { self.count_f[*v] += 1 }
		}
		self.history.truncate(height)
	}


	pub fn dpll(self: &mut CNF, t: &Set, f: &Set) -> Option<Set> {
		let height = self.history.len();

		let mut known = t.len() + f.len();
		let mut t = t.clone();
		let mut f = f.clone();

		loop { // until there are no more pure or unit clauses
			// Step 1: Detect unit clauses
			for i in 0..self.formula.len() {
				if self.check_satisfied(i, &t, &f) { continue }

				match CNF::find_unit(&self.formula[i].t, &f, 2) {
					(0, _) =>
						match CNF::find_unit(&self.formula[i].f, &t, 2) {
							(0, _) => { self.pop_state(height); return None },
							(1, v) => { f.insert(v); self.mark_satisfied(i) },
							_ => {}
						},
					(1, v) =>
						match CNF::find_unit(&self.formula[i].f, &t, 1) {
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

			let new_known = t.len() + f.len();
			assert!(new_known >= known);
			if new_known == known { break }
			known = new_known
		}

		// Step 3: Branch
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

		self.pop_state(height);
		None
	}
	
	pub fn validate(self: &CNF, solution: &Set) -> bool {
		self.formula.iter().all( |c| c.eval_complete(solution) )
	}
}
