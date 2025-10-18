use std::fmt;

enum Block {
	Occupied(u32),
	Empty,
}

impl fmt::Display for Block {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Occupied(id) => write!(f, "{}", id),
			Self::Empty => write!(f, "."),
		}
	}
}

struct Memory(Vec<Block>);

impl Memory {
	fn prepare(input: &[u8]) -> impl Iterator<Item = u8> + use<'_> {
		input.into_iter().filter_map(|d| {
			if *d >= '0' as u8 && *d <= '9' as u8 {
				Some(d - '0' as u8)
			} else {
				None
			}
		})
	}

	fn new_blocks(input: &[u8]) -> Self {
		let blocks = Self::prepare(input)
			.enumerate()
			.flat_map(|(i, d)| {
				if i % 2 == 0 {
					let id = i as u32 / 2;
					(0..d)
						.into_iter()
						.map(|_| Block::Occupied(id))
						.collect::<Vec<_>>()
				} else {
					(0..d).into_iter().map(|_| Block::Empty).collect::<Vec<_>>()
				}
			})
			.collect::<Vec<_>>();

		Self(blocks)
	}

	fn checksum(&self) -> u64 {
		self.0
			.iter()
			.filter_map(|b| match b {
				Block::Occupied(id) => Some(id),
				Block::Empty => None,
			})
			.enumerate()
			.map(|(i, n)| i as u64 * *n as u64)
			.sum()
	}
}

impl fmt::Display for Memory {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for block in &self.0 {
			write!(f, "{block}")?;
		}
		Ok(())
	}
}

fn fill_empty_block(mut mem: Memory) -> u64 {
	let mut i = 0;

	while i < mem.0.len() {
		if let Block::Empty = mem.0[i] {
			mem.0.swap_remove(i);
		} else {
			i += 1;
		}
	}

	mem.checksum()
}

fn fill_empty_pages(mut mem: Memory) -> u64 {
	let mut next_id = match mem.0[mem.len() - 1] {
		Block::Occupied(id) => id,
		Block::Empty => unreachable!(),
	};

	let mut space_index = 0;
	let mut space_len = 0;

	while space_index < mem.0.len() {
		if let Block::Empty = mem.0[i] {
			while space_index + space_len < mem.0.len() {
				if let Block::Empty = mem.0[space_index + space_len - 1] {
					space_len += 1;
				}
			}
		} else {
			i += 1;
		}
	}

	mem.checksum()
}

fn main() {
	let input = std::fs::read("../input/2024/day9.txt").unwrap();
	// let input = std::fs::read("../input/2024/day9.sample.txt").unwrap();

	let bmemory = Memory::new_blocks(&input);
	let _silver = dbg!(fill_empty_block(bmemory));

	let pmemory = Memory::new_pages(&input);
	let _golden = dbg!(fill_empty_pages(pmemory));
}
