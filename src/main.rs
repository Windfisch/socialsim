use rand::random;
use std::fs::File;
use std::io::Write;

const STEPS: usize = 100_000_000;
const SUBSTEPS: usize = 10;
const N_ACTORS: usize = 1000;

struct Traits {
	/// How much of an asshole
	egoism: f64,
	/// How much happiness contributes to the effective egoism
	rationality: f64,
	/// How fast happiness changes
	stability: f64
}

struct Mindstate {
	happiness: f64,
}

struct Actor {
	money: i64,
	traits: Traits,
	mindstate: Mindstate
}

trait SliceExt {
	type Output;
	fn get_mut2(&mut self, a: usize, b: usize) -> (&mut Self::Output, &mut Self::Output);
}

impl Actor {
	pub fn nudge_happiness(&mut self, target: f64) {
		self.mindstate.happiness = (1.0 - self.traits.stability) * target + self.traits.stability * self.mindstate.happiness;
	}

	pub fn effective_egoism(&mut self) -> f64 {
		self.traits.rationality * self.traits.egoism + (1.0 - self.traits.rationality) * (1.0 - self.mindstate.happiness)
	}
}

impl<T> SliceExt for [T] {
	type Output = T;
	fn get_mut2(&mut self, a: usize, b: usize) -> (&mut T, &mut T) {
		assert!(a != b);
		if a > b {
			self.get_mut2(b, a)
		}
		else {
			let (one, two) = self.split_at_mut(b);
			(&mut one[a], &mut two[0])
		}
	}
}

fn play_n(a1: &mut Actor, a2: &mut Actor, n: usize) {
	for i in 0..n {
		if i % 2 == 0 {
			play_one(a1, a2);
		}
		else {
			play_one(a2, a1);
		}
	}
}

fn play_one(a1: &mut Actor, a2: &mut Actor) {
	for _ in 0..SUBSTEPS {
		if a1.effective_egoism() < 0.5 {
			a1.money -= 1;
			a2.money += 2;
			a2.nudge_happiness(1.0);
		}
		else {
			a2.nudge_happiness(0.0);
		}
	}
}

fn avg_happiness(actors: &[Actor]) -> f64 {
	actors.iter().map(|a| a.mindstate.happiness).sum::<f64>() / (actors.len() as f64)
}

fn game(actors: &mut [Actor]) {
	let p1 = random::<usize>() % actors.len();
	let p2 = random::<usize>() % actors.len();

	if p1 != p2 {
		let (a1, a2) = actors.get_mut2(p1, p2);
		println!("Players playing: {p1} {p2}");
		println!("Money before: {} {}", a1.money, a2.money);
		play_n(a1, a2, SUBSTEPS);
		println!("Money after: {}, {}", a1.money, a2.money);
	}
}

fn bar(x: f64) -> String {
	let len = 40;
	let first = (len as f64 * x) as usize;
	format!("{}{} {:5.3}", "#".repeat(first), " ".repeat(len-first), x)
}

fn main() {
	let mut actors: Vec<Actor> = (0..N_ACTORS).map(
		|_| Actor {
			money: 100,
			traits: Traits {
				egoism: random(),
				rationality: 0.5,
				stability: 0.5
			},
			mindstate: Mindstate { happiness: 0.5 }
		}
	).collect();

	let mut happiness_file = File::create("happiness.sim").unwrap();
	writeln!(&mut happiness_file, "{:20} {:20}", "# iteration", "happiness").unwrap();

	for i in 0..STEPS {
		game(&mut actors);

		if i % 100000 == 0 {
			eprintln!("Progress: {} / {}\t\tHappiness: {} 1% / mid / 99% = {:5.3} {:e} {:5.3}",
				i,
				STEPS,
				&bar(avg_happiness(&actors)),
				actors.iter().filter(|a| a.mindstate.happiness < 0.01).count() as f64 / actors.len() as f64,
				actors.iter().filter(|a| (0.01 .. 0.99).contains(&a.mindstate.happiness)).count() as f64 / actors.len() as f64,
				actors.iter().filter(|a| a.mindstate.happiness > 0.99).count() as f64 / actors.len() as f64,
				);
		}

		if i % 10000 == 0 {
			writeln!(&mut happiness_file, "{:20} {:20}",
				i,
				avg_happiness(&actors)
			).unwrap();
		}
	}

	let mut f = File::create("data.sim").unwrap();
	writeln!(&mut f, "{:10} {:10} {:10}", "# player", "egoism", "money").unwrap();
	for (i, a) in actors.iter().enumerate() {
		writeln!(&mut f, "{:10} {:10} {:10}", i, a.traits.egoism, a.money).unwrap();
	}
}
