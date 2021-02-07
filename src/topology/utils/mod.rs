#[inline]
pub fn rbpi_ordered(a: u32, b: u32) -> (u32, u32) {
	if a == 0 || b == 0 {
		(0, a | b)
	} else {
		let mask = u32::MAX >> std::cmp::max(u32::leading_zeros(a), u32::leading_zeros(b));
		let ma = a & mask;
		let mb = b & mask;

		if ma == mb {
			if a < b {
				(a, b)
			} else {
				(b, a)
			}
		} else {
			if ma < mb {
				(a, b)
			} else {
				(b, a)
			}
		}
	}
}

#[inline]
pub fn nrbpi_ordered(a: u32, b: u32) -> (u32, u32) {
	if a == 0 {
		(b, 0)
	} else if b == 0 {
		(a, 0)
	} else {
		let (a, b) = rbpi_ordered(nrbpi_to_rbpi(a), nrbpi_to_rbpi(b));
		(rbpi_to_nrbpi(a), rbpi_to_nrbpi(b))
	}
}

#[inline]
pub fn rbpi_min(a: u32, b: u32) -> u32 {
	rbpi_ordered(a, b).0
}

#[inline]
pub fn rbpi_next(n: u32, depth: u32) -> u32 {
	n | (1 << depth)
}

/// From RBPI space to normalized RBPI space.
#[inline]
pub fn rbpi_to_nrbpi(a: u32) -> u32 {
	a + 1
}

/// From normalized RBPI space to RBPI space.
#[inline]
pub fn nrbpi_to_rbpi(a: u32) -> u32 {
	a - 1
}

#[inline]
pub fn nrbpi_mean(a: u32, b: u32) -> u32 {
	if a == b {
		a
	} else {
		let (a, b) = nrbpi_ordered(a, b);
		if b == 0 {
			let a = nrbpi_to_rbpi(a);

			let depth = 32 - u32::leading_zeros(a);
			rbpi_to_nrbpi(rbpi_next(a, depth))
		} else {
			let a = nrbpi_to_rbpi(a);
			let b = nrbpi_to_rbpi(b);

			let depth = 32 - std::cmp::min(u32::leading_zeros(a), u32::leading_zeros(b));
			println!("depth, a, b: {}, {}, {}", depth, a, b);
			rbpi_to_nrbpi(rbpi_next(a, depth))
		}
	}
}

/// Implementation of [Matthew Szudzik "elegant" pairing function](http://szudzik.com/ElegantPairing.pdf).
#[inline]
pub fn elegant_pair(x: u32, y: u32) -> u32 {
	if x == std::cmp::max(x, y) {
		x * x + x + y
	} else {
		y * y + x
	}
}

/// Implementation of [Matthew Szudzik "elegant" unpairing function](http://szudzik.com/ElegantPairing.pdf).
#[inline]
pub fn elegant_unpair(z: u32) -> (u32, u32) {
	use integer_sqrt::IntegerSquareRoot;
	let sz = z.integer_sqrt();
	let sz2 = sz * sz;
	let zmsz2 = z - sz2;

	if zmsz2 < sz {
		(zmsz2, sz)
	} else {
		(sz, zmsz2 - sz)
	}
}

#[inline]
pub fn nrbpi2_mean(a: u32, b: u32) -> u32 {
	let (ax, ay) = elegant_unpair(a);
	let (bx, by) = elegant_unpair(b);

	let cx = nrbpi_mean(ax, bx);
	let cy = nrbpi_mean(ay, by);
	elegant_pair(cx, cy)
}

#[cfg(test)]
mod test {
	#[test]
	fn rbpi_min1() {
		use super::rbpi_min;
		assert_eq!(rbpi_min(0, 4), 0);
		assert_eq!(rbpi_min(2, 4), 4);
		assert_eq!(rbpi_min(2, 6), 2);
		assert_eq!(rbpi_min(1, 6), 6);
		assert_eq!(rbpi_min(1, 5), 1);
		assert_eq!(rbpi_min(3, 5), 5);
		assert_eq!(rbpi_min(3, 7), 3);
	}

	#[test]
	fn nrbpi_mean() {
		use super::nrbpi_mean;
		assert_eq!(nrbpi_mean(1, 3), 5);
		assert_eq!(nrbpi_mean(3, 2), 7);
		assert_eq!(nrbpi_mean(2, 4), 6);
		assert_eq!(nrbpi_mean(4, 0), 8);
	}
}
