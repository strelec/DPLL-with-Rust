pub use self::clause::*;
use std::cmp::max;

mod clause;

#[derive(Clone)]
pub struct CNF {
	formula: Vec<Clause>,
	mask: Vec<bool>,
	history: Vec<usize>,
}

impl CNF {
	pub fn new(formula: Vec<Clause>) -> CNF {
		let len = formula.len();
		CNF {
			formula: formula,
			mask: vec![false; len],
			history: Vec::with_capacity(len),
		}
	}

	fn branching_strategy(self: &CNF, t: &Set, f: &Set) -> usize {
		// Select the most commonly occuring variable
		let mut counts = vec![0; max(t.capacity(), f.capacity())+1];
		for i in 0..self.formula.len() {
			if self.mask[i] { continue }
			let Clause { t: ref ct, f: ref cf } = self.formula[i];
			for v in ct { counts[*v] += 1 }
			for v in cf { counts[*v] += 1 }
		}
		for v in t { counts[v] = 0 };
		for v in f { counts[v] = 0 };

		let mut count = 0usize;
		let mut result = 0usize;
		for (i, item) in counts.iter().enumerate() {
			if *item > count {
				count = *item;
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
		self.history.push(i)
	}

	fn check_satisfied(self: &mut CNF, i: usize, t: &Set, f: &Set) -> bool {
		self.mask[i] || self.formula[i].eval(&t, &f) && {
			self.mark_satisfied(i); true
		}
	}

	fn pop_state(self: &mut CNF, height: usize) {
		for i in self.history[height..].into_iter() {
			self.mask[*i] = false;
		}
		self.history.truncate(height)
	}


	pub fn dpll(self: &mut CNF, t: &Set, f: &Set) -> Option<Set> {
		let height = self.history.len();

		let mut known = t.len() + f.len();
		let mut t = t.clone();
		let mut f = f.clone();

		let mut trues = Set::new();
		let mut falses = Set::new();

		loop { // until there are no more pure or unit clauses
			trues.clear();
			falses.clear();

			for i in 0..self.formula.len() {
				if self.check_satisfied(i, &t, &f) { continue }
				// let Clause { t: ref ct, f: ref cf } = self.formula[i];

				// Step 1: Detect unit clauses
				match CNF::find_unit(&self.formula[i].t, &f, 2) {
					(0, _) =>
						match CNF::find_unit(&self.formula[i].f, &t, 2) {
							(0, _) => { self.pop_state(height); return None},
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

				// Step 2: Detect pure variables
				for v in &self.formula[i].t { trues.insert(*v); };
				for v in &self.formula[i].f { falses.insert(*v); }
			}

			if self.all_satisfied() { return Some(t) }
			if !t.is_disjoint(&f) { self.pop_state(height); return None }

			trues.difference_with(&f);
			falses.difference_with(&t);
			t.union_with(&trues);
			t.difference_with(&falses);
			f.union_with(&falses);
			f.difference_with(&trues);

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
