use rand::random;
use std::fs::File;
use std::io::Write;

const STEPS: usize = 10_000_000;
const SUBSTEPS: usize = 10;
const N_ACTORS: usize = 1000;

struct Traits {
	egoism: f64
}

struct Mindstate {
	happiness: f64
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
		if a1.mindstate.happiness > a1.traits.egoism {
			a1.money -= 1;
			a2.money += 2;
			a2.mindstate.happiness = (1.0 + a2.mindstate.happiness) / 2.0;
		}
		else {
			a2.mindstate.happiness = (0.0 + a2.mindstate.happiness) / 2.0;
		}
	}
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

fn main() {
	let mut actors: Vec<Actor> = (0..N_ACTORS).map(
		|_| Actor {
			money: 100,
			traits: Traits { egoism: random() },
			mindstate: Mindstate { happiness: 0.5 }
		}
	).collect();

	let mut happiness_file = File::create("happiness.sim").unwrap();
	writeln!(&mut happiness_file, "{:20} {:20}", "# iteration", "happiness").unwrap();

	for i in 0..STEPS {
		game(&mut actors);

		if i % 10000 == 0 {
			eprintln!("Progress: {} / {}", i, STEPS);
		}

		if i % 1000 == 0 {
			writeln!(&mut happiness_file, "{:20} {:20}",
				i,
				actors.iter().map(|a| a.mindstate.happiness).sum::<f64>() / (actors.len() as f64)
			).unwrap();
		}
	}

	let mut f = File::create("data.sim").unwrap();
	writeln!(&mut f, "{:10} {:10} {:10}", "# player", "egoism", "money").unwrap();
	for (i, a) in actors.iter().enumerate() {
		writeln!(&mut f, "{:10} {:10} {:10}", i, a.traits.egoism, a.money).unwrap();
	}
}
