extern crate bit_set;

use std::fmt;
use bit_set::BitSet;

pub type Set = BitSet;

#[derive(Debug)]
pub struct Clause {
	pub t: Set,
	pub f: Set
}

impl Clause {
	pub fn eval(&self, tx: &Set, fx: &Set) -> bool {
		!(self.t.is_disjoint(tx) && self.f.is_disjoint(fx))
	}

	pub fn simplify(&self, tx: &Set, fx: &Set) -> Clause {
		Clause{ t: self.t.difference(tx).collect(), f: self.f.difference(fx).collect() }
	}
}

impl fmt::Display for Clause {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({:?}, {:?})", self.t, self.f)
	}
}

pub type CNF = Vec<Clause>;
