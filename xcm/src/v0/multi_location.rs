// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Cross-Consensus Message format data structures.

use sp_std::{result, convert::TryFrom};
use sp_runtime::RuntimeDebug;
use codec::{self, Encode, Decode};
use super::Junction;
use crate::VersionedMultiLocation;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, RuntimeDebug)]
pub enum MultiLocation {
	Null,
	X1(Junction),
	X2(Junction, Junction),
	X3(Junction, Junction, Junction),
	X4(Junction, Junction, Junction, Junction),
}

impl From<Junction> for MultiLocation {
	fn from(x: Junction) -> Self {
		MultiLocation::X1(x)
	}
}

impl From<()> for MultiLocation {
	fn from(_: ()) -> Self {
		MultiLocation::Null
	}
}
impl From<(Junction,)> for MultiLocation {
	fn from(x: (Junction,)) -> Self {
		MultiLocation::X1(x.0)
	}
}
impl From<(Junction, Junction)> for MultiLocation {
	fn from(x: (Junction, Junction)) -> Self {
		MultiLocation::X2(x.0, x.1)
	}
}
impl From<(Junction, Junction, Junction)> for MultiLocation {
	fn from(x: (Junction, Junction, Junction)) -> Self {
		MultiLocation::X3(x.0, x.1, x.2)
	}
}
impl From<(Junction, Junction, Junction, Junction)> for MultiLocation {
	fn from(x: (Junction, Junction, Junction, Junction)) -> Self {
		MultiLocation::X4(x.0, x.1, x.2, x.3)
	}
}

impl From<[Junction; 0]> for MultiLocation {
	fn from(_: [Junction; 0]) -> Self {
		MultiLocation::Null
	}
}
impl From<[Junction; 1]> for MultiLocation {
	fn from(x: [Junction; 1]) -> Self {
		let [x0] = x;
		MultiLocation::X1(x0)
	}
}
impl From<[Junction; 2]> for MultiLocation {
	fn from(x: [Junction; 2]) -> Self {
		let [x0, x1] = x;
		MultiLocation::X2(x0, x1)
	}
}
impl From<[Junction; 3]> for MultiLocation {
	fn from(x: [Junction; 3]) -> Self {
		let [x0, x1, x2] = x;
		MultiLocation::X3(x0, x1, x2)
	}
}
impl From<[Junction; 4]> for MultiLocation {
	fn from(x: [Junction; 4]) -> Self {
		let [x0, x1, x2, x3] = x;
		MultiLocation::X4(x0, x1, x2, x3)
	}
}

pub struct MultiLocationIterator(MultiLocation);
impl Iterator for MultiLocationIterator {
	type Item = Junction;
	fn next(&mut self) -> Option<Junction> {
		self.0.take_first()
	}
}

pub struct MultiLocationReverseIterator(MultiLocation);
impl Iterator for MultiLocationReverseIterator {
	type Item = Junction;
	fn next(&mut self) -> Option<Junction> {
		self.0.take_last()
	}
}

pub struct MultiLocationRefIterator<'a>(&'a MultiLocation, usize);
impl<'a> Iterator for MultiLocationRefIterator<'a> {
	type Item = &'a Junction;
	fn next(&mut self) -> Option<&'a Junction> {
		let result = self.0.at(self.1);
		self.1 += 1;
		result
	}
}

pub struct MultiLocationReverseRefIterator<'a>(&'a MultiLocation, usize);
impl<'a> Iterator for MultiLocationReverseRefIterator<'a> {
	type Item = &'a Junction;
	fn next(&mut self) -> Option<&'a Junction> {
		self.1 += 1;
		self.0.at(self.0.len().checked_sub(self.1)?)
	}
}

impl MultiLocation {
	pub fn first(&self) -> Option<&Junction> {
		match &self {
			MultiLocation::Null => None,
			MultiLocation::X1(ref a) => Some(a),
			MultiLocation::X2(ref a, ..) => Some(a),
			MultiLocation::X3(ref a, ..) => Some(a),
			MultiLocation::X4(ref a, ..) => Some(a),
		}
	}
	pub fn last(&self) -> Option<&Junction> {
		match &self {
			MultiLocation::Null => None,
			MultiLocation::X1(ref a) => Some(a),
			MultiLocation::X2(.., ref a) => Some(a),
			MultiLocation::X3(.., ref a) => Some(a),
			MultiLocation::X4(.., ref a) => Some(a),
		}
	}
	pub fn split_first(self) -> (MultiLocation, Option<Junction>) {
		match self {
			MultiLocation::Null => (MultiLocation::Null, None),
			MultiLocation::X1(a) => (MultiLocation::Null, Some(a)),
			MultiLocation::X2(a, b) => (MultiLocation::X1(b), Some(a)),
			MultiLocation::X3(a, b, c) => (MultiLocation::X2(b, c), Some(a)),
			MultiLocation::X4(a, b, c ,d) => (MultiLocation::X3(b, c, d), Some(a)),
		}
	}
	pub fn split_last(self) -> (MultiLocation, Option<Junction>) {
		match self {
			MultiLocation::Null => (MultiLocation::Null, None),
			MultiLocation::X1(a) => (MultiLocation::Null, Some(a)),
			MultiLocation::X2(a, b) => (MultiLocation::X1(a), Some(b)),
			MultiLocation::X3(a, b, c) => (MultiLocation::X2(a, b), Some(c)),
			MultiLocation::X4(a, b, c ,d) => (MultiLocation::X3(a, b, c), Some(d)),
		}
	}
	pub fn take_first(&mut self) -> Option<Junction> {
		let mut d = MultiLocation::Null;
		sp_std::mem::swap(&mut *self, &mut d);
		let (tail, head) = d.split_first();
		*self = tail;
		head
	}
	pub fn take_last(&mut self) -> Option<Junction> {
		let mut d = MultiLocation::Null;
		sp_std::mem::swap(&mut *self, &mut d);
		let (head, tail) = d.split_last();
		*self = head;
		tail
	}
	pub fn pushed_with(self, new: Junction) -> result::Result<Self, Self> {
		Ok(match self {
			MultiLocation::Null => MultiLocation::X1(new),
			MultiLocation::X1(a) => MultiLocation::X2(a, new),
			MultiLocation::X2(a, b) => MultiLocation::X3(a, b, new),
			MultiLocation::X3(a, b, c) => MultiLocation::X4(a, b, c, new),
			s => Err(s)?,
		})
	}
	pub fn pushed_front_with(self, new: Junction) -> result::Result<Self, Self> {
		Ok(match self {
			MultiLocation::Null => MultiLocation::X1(new),
			MultiLocation::X1(a) => MultiLocation::X2(new, a),
			MultiLocation::X2(a, b) => MultiLocation::X3(new, a, b),
			MultiLocation::X3(a, b, c) => MultiLocation::X4(new, a, b, c),
			s => Err(s)?,
		})
	}
	pub fn len(&self) -> usize {
		match &self {
			MultiLocation::Null => 0,
			MultiLocation::X1(..) => 1,
			MultiLocation::X2(..) => 2,
			MultiLocation::X3(..) => 3,
			MultiLocation::X4(..) => 4,
		}
	}

	pub fn at(&self, i: usize) -> Option<&Junction> {
		Some(match (i, &self) {
			(0, MultiLocation::X1(ref a)) => a,
			(0, MultiLocation::X2(ref a, ..)) => a,
			(0, MultiLocation::X3(ref a, ..)) => a,
			(0, MultiLocation::X4(ref a, ..)) => a,
			(1, MultiLocation::X2(_, ref a)) => a,
			(1, MultiLocation::X3(_, ref a, ..)) => a,
			(1, MultiLocation::X4(_, ref a, ..)) => a,
			(2, MultiLocation::X3(_, _, ref a)) => a,
			(2, MultiLocation::X4(_, _, ref a, ..)) => a,
			(3, MultiLocation::X4(_, _, _, ref a)) => a,
			_ => return None,
		})
	}

	pub fn at_mut(&mut self, i: usize) -> Option<&mut Junction> {
		Some(match (i, self) {
			(0, MultiLocation::X1(ref mut a)) => a,
			(0, MultiLocation::X2(ref mut a, ..)) => a,
			(0, MultiLocation::X3(ref mut a, ..)) => a,
			(0, MultiLocation::X4(ref mut a, ..)) => a,
			(1, MultiLocation::X2(_, ref mut a)) => a,
			(1, MultiLocation::X3(_, ref mut a, ..)) => a,
			(1, MultiLocation::X4(_, ref mut a, ..)) => a,
			(2, MultiLocation::X3(_, _, ref mut a)) => a,
			(2, MultiLocation::X4(_, _, ref mut a, ..)) => a,
			(3, MultiLocation::X4(_, _, _, ref mut a)) => a,
			_ => return None,
		})
	}

	pub fn iter(&self) -> MultiLocationRefIterator {
		MultiLocationRefIterator(&self, 0)
	}
	pub fn iter_rev(&self) -> MultiLocationReverseRefIterator {
		MultiLocationReverseRefIterator(&self, 0)
	}
	pub fn into_iter(self) -> MultiLocationIterator {
		MultiLocationIterator(self)
	}
	pub fn into_iter_rev(self) -> MultiLocationReverseIterator {
		MultiLocationReverseIterator(self)
	}

	pub fn push(&mut self, new: Junction) -> result::Result<(), ()> {
		let mut n = MultiLocation::Null;
		sp_std::mem::swap(&mut *self, &mut n);
		match n.pushed_with(new) {
			Ok(result) => { *self = result; Ok(()) }
			Err(old) => { *self = old; Err(()) }
		}
	}

	pub fn push_front(&mut self, new: Junction) -> result::Result<(), ()> {
		let mut n = MultiLocation::Null;
		sp_std::mem::swap(&mut *self, &mut n);
		match n.pushed_front_with(new) {
			Ok(result) => { *self = result; Ok(()) }
			Err(old) => { *self = old; Err(()) }
		}
	}

	/// Returns partial result as error in case of failure (e.g. because out of space).
	pub fn appended_with(self, new: MultiLocation) -> result::Result<Self, Self> {
		let mut result= self;
		for j in new.into_iter() {
			result = result.pushed_with(j)?;
		}
		Ok(result)
	}

	/// Ensure that the `prefix` len plus the `self` len is less than the max length, if not
	/// the result is undefined.
	pub fn prepend_with(&mut self, prefix: &MultiLocation) {
		let mut prefix = prefix.clone();
		while match (prefix.last(), self.first()) {
			(Some(x), Some(Junction::Parent)) if x != &Junction::Parent => {
				prefix.take_last();
				self.take_first();
				true
			}
			_ => false,
		} {}
		for j in prefix.into_iter_rev() {
			// Fail silently.
			let _ = self.push_front(j);
		}
	}
}

impl From<MultiLocation> for VersionedMultiLocation {
	fn from(x: MultiLocation) -> Self {
		VersionedMultiLocation::V0(x)
	}
}

impl TryFrom<VersionedMultiLocation> for MultiLocation {
	type Error = ();
	fn try_from(x: VersionedMultiLocation) -> result::Result<Self, ()> {
		match x {
			VersionedMultiLocation::V0(x) => Ok(x),
		}
	}
}