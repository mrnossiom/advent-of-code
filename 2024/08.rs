use std::collections::HashMap;
use std::{fmt, ops};

const WORLD_LEN: usize = 50;
// const WORLD_LEN: usize = 12;
const WORLD_SIZE: usize = WORLD_LEN * WORLD_LEN;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct IVec2(i32, i32);

impl ops::Add<IVec2> for IVec2 {
	type Output = IVec2;

	fn add(self, other: IVec2) -> Self::Output {
		Self(self.0 + other.0, self.1 + other.1)
	}
}

impl ops::Sub<IVec2> for IVec2 {
	type Output = IVec2;

	fn sub(self, other: IVec2) -> Self::Output {
		Self(self.0 - other.0, self.1 - other.1)
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
	Empty,
	Emitter(u8),
	AntiNode,
}

#[derive(Clone)]
struct World([Cell; WORLD_SIZE]);

type Emitters = HashMap<u8, Vec<IVec2>>;

impl World {
	fn new(base: [Cell; WORLD_SIZE]) -> Self {
		Self(base)
	}

	fn parse(input: &[u8]) -> (Self, Emitters) {
		let mut emitters = HashMap::<u8, Vec<IVec2>>::new();

		let mut line = 0;
		let mut pos = 0;

		let world = input
			.into_iter()
			.filter_map(|byte| match byte {
				b'.' => {
					pos += 1;
					Some(Cell::Empty)
				}
				b'\n' => {
					pos = 0;
					line += 1;
					None
				}
				emitter => {
					emitters.entry(*emitter).or_default().push(IVec2(pos, line));

					pos += 1;
					Some(Cell::Emitter(*emitter))
				}
			})
			.collect::<Vec<_>>();

		let world = World::new(world.try_into().unwrap());

		(world, emitters)
	}

	fn get(&self, IVec2(x, y): IVec2) -> &Cell {
		&self.0[y as usize * WORLD_LEN + x as usize]
	}

	fn get_mut(&mut self, IVec2(x, y): IVec2) -> &mut Cell {
		&mut self.0[y as usize * WORLD_LEN + x as usize]
	}

	fn pos_in_bounds(&self, IVec2(x, y): IVec2) -> bool {
		x >= 0 && x < WORLD_LEN as i32 && y >= 0 && y < WORLD_LEN as i32
	}
}

impl fmt::Display for World {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for y in 0..WORLD_LEN {
			for x in 0..WORLD_LEN {
				match self.get(IVec2(x as i32, y as i32)) {
					Cell::Empty => write!(f, ".")?,
					Cell::Emitter(chr) => write!(f, "{}", *chr as char)?,
					Cell::AntiNode => write!(f, "#")?,
				}
			}
			writeln!(f)?;
		}
		Ok(())
	}
}

fn mirror_point(point: IVec2, origin: IVec2) -> IVec2 {
	let dir = point - origin;
	origin - dir
}

fn count_antinodes(mut world: World, emitters: Emitters) -> u64 {
	let mut count = 0;

	for (kind, list) in emitters {
		for el1 in &list {
			for el2 in &list {
				if *el1 == *el2 {
					continue;
				}

				let mirror = mirror_point(*el1, *el2);
				if !world.pos_in_bounds(mirror) {
					continue;
				}

				match world.get_mut(mirror) {
					cell @ Cell::Empty => {
						count += 1;
						*cell = Cell::AntiNode;
					}
					// counted but do not replace antenna
					cell @ Cell::Emitter(_) => {
						count += 1;
						*cell = Cell::AntiNode;
					}
					// is not unique
					Cell::AntiNode => {}
				}
			}
		}
	}

	count
}

fn count_antinodes_in_line(mut world: World, emitters: Emitters) -> u64 {
	let mut count = 0;

	for (kind, list) in emitters {
		for target in &list {
			for origin in &list {
				if *target == *origin {
					if list.len() > 1 {
						count += 1;
					}
					continue;
				}

				let delta = *origin - *target;

				let mut pos = *origin + delta;
				while world.pos_in_bounds(pos) {
					match world.get_mut(pos) {
						cell @ Cell::Empty => {
							count += 1;
							*cell = Cell::AntiNode;
						}
						// counted but do not replace antenna
						cell @ Cell::Emitter(_) => {
							// count += 1;
							*cell = Cell::AntiNode;
						}
						// is not unique
						Cell::AntiNode => {}
					}

					pos = pos + delta;
				}
			}
		}
	}

	println!("{world}");

	count
}

fn main() {
	let input = std::fs::read("../input/2024/day8.txt").unwrap();
	// let input = std::fs::read("../input/2024/day8.sample.txt").unwrap();

	let (world, emitters) = World::parse(&input);

	let _silver = dbg!(count_antinodes(world.clone(), emitters.clone()));
	let _golden = dbg!(count_antinodes_in_line(world.clone(), emitters.clone()));
}
