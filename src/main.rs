use std::collections::HashSet;
use std::fmt;
use std::io;
use std::io::BufRead;

type Set = HashSet<u16>;

#[derive(Debug)]
struct Clause {
	t: Set,
	f: Set
}

impl Clause {
	fn eval(&self, tx: &Set, fx: &Set) -> bool {
		!(self.t.is_disjoint(tx) && self.f.is_disjoint(fx))
	}

	fn simplify(&self, tx: &Set, fx: &Set) -> Clause {
		Clause{ t: &self.t - fx, f: &self.f - tx }
	}
}

impl fmt::Display for Clause {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({:?}, {:?})", self.t, self.f)
	}
}

type CNF = Vec<Clause>;




fn select_var_to_branch(formula: &CNF) -> u16 {
	// Naive selection
	*formula[0].t.iter().next().unwrap_or(
		&formula[0].f.iter().next().unwrap_or(&0)
	)
}

fn propagate(formula: &CNF, t: &Set, f: &Set) -> CNF {
	formula.iter().filter( |c| !c.eval(t, f) ).map( |c| c.simplify(t, f) ).collect()
}

fn dpll(formula: CNF) -> Option<Set> {
	//println!("Call: {:?}", formula);
	if formula.is_empty() { return Some(Set::new()) }

	// Step 1: Detect unit clauses
	let mut t = Set::new();
	let mut f = Set::new();

	for clause in &formula {
		match (clause.t.len(), clause.f.len()) {
			(0, 0) => return None,
			(1, 0) => t.extend(&clause.t),
			(0, 1) => f.extend(&clause.f),
			_ => {}
		}
	}

	//println!("Unit clauses: {:?} {:?}", t, f);
	if !t.is_disjoint(&f) {
		return None
	}

	// Step 2: Detect pure variables
	let mut trues = Set::new();
	let mut falses = Set::new();

	for clause in &formula {
		trues.extend(&clause.t);
		falses.extend(&clause.f);
	}

	t.extend(&trues);
	for s in &falses { t.remove(&s); }
	f.extend(&falses);
	for s in &trues { f.remove(&s); }

	//println!("Known clauses: {:?} {:?}", t, f);

	if t.is_empty() && f.is_empty() {
		let branch_var = select_var_to_branch(&formula);
		if branch_var == 0 {
			return None
		}

		t.insert(branch_var);
		if let Some(set) = dpll(propagate(&formula, &t, &f)) {
			return Some(&set | &t)
		}

		t.remove(&branch_var);
		f.insert(branch_var);
		if let Some(set) = dpll(propagate(&formula, &t, &f)) {
			return Some(set)
		}
	} else {
		if let Some(set) = dpll(propagate(&formula, &t, &f)) {
			return Some(&set | &t)
		}
	}

	None
}

fn main() {
	let stdin = io::stdin();
	let formula = stdin.lock().lines().flat_map( |x| {
		let y = x.unwrap();
		let line = y.trim();
		match line.chars().nth(0).unwrap() {
			'c' | 'p' => None,
			_ => {
				let mut t = Set::new();
				let mut f = Set::new();
				for v in line.split_whitespace() {
					let n: i32 = v.parse::<i32>().unwrap();
					//assert!(n != 0);
					if n > 0 { t.insert(n as u16); } else if n < 0 { f.insert(-n as u16); }
				}
				Some(Clause { t: t, f: f })
			}
		}
	}).collect();

	println!("{:?}", dpll(formula));
}
