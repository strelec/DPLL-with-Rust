extern crate bit_set;

use self::bit_set::BitSet;

pub type Set = BitSet;
pub type Bag = Vec<usize>;

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

	pub fn find_unit(bag: &Bag, filter: &Set, up_to: u8) -> (u8, usize) {
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
}
