pub use self::clause::*;

mod clause;

fn select_var_to_branch(formula: &CNF) -> usize {
	// Naive selection
	formula[0].t.iter().next().unwrap_or_else( ||
		formula[0].f.iter().next().unwrap()
	)
}

fn propagate(formula: &CNF, t: &Set, f: &Set) -> CNF {
	formula.iter().filter( |c| !c.eval(t, f) ).map( |c| c.simplify(t, f) ).collect()
}


pub fn dpll(formula: &CNF) -> Option<Set> {
	if formula.is_empty() { return Some(Set::new()) }

	// Step 1: Detect unit clauses
	let mut t = Set::new();
	let mut f = Set::new();

	for clause in formula {
		match (clause.t.len(), clause.f.len()) {
			(0, 0) => return None,
			(1, 0) => t.union_with(&clause.t),
			(0, 1) => f.union_with(&clause.f),
			_ => {}
		}
	}

	if !t.is_disjoint(&f) {
		return None
	}

	// Step 2: Detect pure variables
	let mut trues = Set::new();
	let mut falses = Set::new();

	for clause in formula {
		trues.union_with(&clause.t);
		falses.union_with(&clause.f);
	}

	t.union_with(&trues);
	t.difference_with(&falses);
	f.union_with(&falses);
	f.difference_with(&trues);

	// Step 3: Apply free variables or branch if forced to
	if t.is_empty() && f.is_empty() {
		let branch_var = select_var_to_branch(formula);

		t.insert(branch_var);
		if let Some(set) = dpll(&propagate(formula, &t, &f)) {
			return Some(set.union(&t).collect())
		}

		t.remove(branch_var);
		f.insert(branch_var);
		if let Some(set) = dpll(&propagate(formula, &t, &f)) {
			return Some(set)
		}
	} else {
		if let Some(set) = dpll(&propagate(formula, &t, &f)) {
			return Some(set.union(&t).collect())
		}
	}

	None
}