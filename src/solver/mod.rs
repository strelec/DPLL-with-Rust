pub use self::clause::*;
use std::cmp::max;

mod clause;

pub struct CNF {
	pub formula: Vec<Clause>
}

impl CNF {
	fn branching_strategy(self: &CNF, t: &Set, f: &Set) -> usize {
		// Select the most commonly occuring variable
		let mut counts = vec![0; max(t.capacity(), f.capacity())+1];
		for clause in &self.formula {
			if clause.eval(t, f) { continue }
			for v in &clause.t { counts[*v] += 1 };
			for v in &clause.f { counts[*v] += 1 };
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

	fn find_unit(self: &CNF, bag: &Bag, filter: &Set, up_to: u8) -> (u8, usize) {
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


	pub fn dpll(self: &CNF, t: &Set, f: &Set) -> Option<Set> {
		let mut known = t.len() + f.len();

		let mut t = t.clone();
		let mut f = f.clone();

		let mut trues = Set::new();
		let mut falses = Set::new();

		loop { // until there are no more pure or unit clauses
			trues.clear();
			falses.clear();

			let mut satisfied = true;
			for clause in &self.formula {
				if clause.eval(&t, &f) {
					continue
				}
				satisfied = false;

				// Step 1: Detect unit clauses
				match self.find_unit(&clause.t, &f, 2) {
					(0, _) =>
						match self.find_unit(&clause.f, &t, 2) {
							(0, _) => return None,
							(1, v) => {f.insert(v);},
							_ => {}
						},
					(1, v) =>
						match self.find_unit(&clause.f, &t, 1) {
							(0, _) => {t.insert(v);},
							_ => {}
						},
					_ => {}
				}

				// Step 2: Detect pure variables
				for v in &clause.t { trues.insert(*v); };
				for v in &clause.f { falses.insert(*v); }
			}

			if satisfied { return Some(t) }
			if !t.is_disjoint(&f) { return None }

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
			return Some(set.union(&t).collect())
		}

		t.remove(branch_var);
		f.insert(branch_var);
		if let Some(set) = self.dpll(&t, &f) {
			return Some(set)
		}

		None
	}
	
	pub fn validate(self: &CNF, solution: &Set) -> bool {
		self.formula.iter().all( |c| c.eval_complete(solution) )
	}
}
