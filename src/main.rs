use rand::{
	distributions::{Distribution, Standard},
	Rng,
};
use rayon::prelude::*;

fn main() {
	let max: u32 = u32::MAX;

	// Run the simulation max times.
	let (wins_false, wins_true) = (0..max)
		.into_par_iter()
		.map(|_| (monty_hall(false), monty_hall(true)))
		.map(|(x, y)| (x as u32, y as u32))
		.reduce(|| (0, 0), |(x1, y1), (x2, y2)| (x1 + x2, y1 + y2));
	println!("Wins without switching doors: {}", wins_false);
	println!(
		"Win percentage without switching doors: {}%",
		wins_false as f64 / max as f64 * 100.0
	);
	println!("Wins with switching doors: {}", wins_true);
	println!(
		"Win percentage with switching doors: {}%",
		wins_true as f64 / max as f64 * 100.0
	);
}

#[derive(Debug, PartialEq)]
enum Door {
	Prize,
	Goat,
}

#[derive(Debug)]
enum Selection {
	Door1,
	Door2,
	Door3,
}

impl Distribution<Selection> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Selection {
		match rng.gen_range(0..3) {
			0 => Selection::Door1,
			1 => Selection::Door2,
			2 => Selection::Door3,
			_ => unreachable!(),
		}
	}
}

/// Simulate the Monty Hall problem.
///
/// # Arguments
///
/// * `switch_doors` - Whether to switch doors after the host opens one.
///
/// # Returns
///
/// * `true` if the player wins, `false` otherwise.
fn monty_hall(switch_doors: bool) -> bool {
	let mut rng = rand::thread_rng();

	// There are three doors.
	let mut doors = [Door::Goat, Door::Goat, Door::Goat];
	// The prize is behind one of the doors.
	let prize_door = rng.gen_range(0..3);
	doors[prize_door] = Door::Prize;

	// The player selects a door. (Selection enum)
	let mut player_selection: Selection = rand::random();

	// The host opens a door.
	let host_opening: Selection = match player_selection {
		Selection::Door1 => match doors[1] {
			Door::Goat => Selection::Door2,
			Door::Prize => Selection::Door3,
		},
		Selection::Door2 => match doors[0] {
			Door::Goat => Selection::Door1,
			Door::Prize => Selection::Door3,
		},
		Selection::Door3 => match doors[0] {
			Door::Goat => Selection::Door1,
			Door::Prize => Selection::Door2,
		},
	};

	// The player switches doors if they want to.
	if switch_doors {
		player_selection = match player_selection {
			Selection::Door1 => match host_opening {
				Selection::Door2 => Selection::Door3,
				Selection::Door3 => Selection::Door2,
				_ => unreachable!(),
			},
			Selection::Door2 => match host_opening {
				Selection::Door1 => Selection::Door3,
				Selection::Door3 => Selection::Door1,
				_ => unreachable!(),
			},
			Selection::Door3 => match host_opening {
				Selection::Door1 => Selection::Door2,
				Selection::Door2 => Selection::Door1,
				_ => unreachable!(),
			},
		};
	}

	// The player wins if they selected the door with the prize.
	match player_selection {
		Selection::Door1 => doors[0] == Door::Prize,
		Selection::Door2 => doors[1] == Door::Prize,
		Selection::Door3 => doors[2] == Door::Prize,
	}
}
