extern crate bit_set;

use std::fmt;
use bit_set::BitSet;

pub type Set = BitSet;
pub type Bag = Vec<usize>;

#[derive(Debug)]
pub struct Clause {
	pub t: Bag,
	pub f: Bag
}

impl Clause {
	pub fn eval(&self, t: &Set, f: &Set) -> bool {
		self.t.iter().any( |&v| t.contains(v) ) ||
		self.f.iter().any( |&v| f.contains(v) )
	}

	pub fn eval_complete(&self, t: &Set) -> bool {
		self.t.iter().any( |&v|  t.contains(v) ) ||
		self.f.iter().any( |&v| !t.contains(v) )
	}
}

impl fmt::Display for Clause {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({:?}, {:?})", self.t, self.f)
	}
}
