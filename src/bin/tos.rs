use app::client::*;
// use std;
use std::convert::*;
use rand::Rng;

fn run() {
	let mut rng = rand::thread_rng();
	let server_url = std::env::args().nth(1).unwrap();
	let mut client = Client::new(server_url);
	if std::env::args().len() == 2 {
		client.send("[1, 0]");
		return;
	}
	let player_key = std::env::args().nth(2).unwrap();
	let mut resp = client.join(&player_key);
	let power = 32;
	let cool = 0;
	let life = 1;
	resp = client.start(resp.info.ability.potential - power * 4 - cool * 12 - life * 2, power, cool, life);

	// let mut dx: i32 = std::env::var("DX").unwrap().parse().unwrap();
	// let dy: i32 = std::env::var("DY").unwrap().parse().unwrap();
	let mut dx = 0;
	let mut dy = 0;
	while resp.stage != 2 {
		let mut myship = None;
		for ship in resp.state.ships.iter() {
			if ship.role != resp.info.role {
				continue;
			}
			myship = Some(ship.clone());
		}
		let myship = myship.unwrap();

		let mut commands = vec![];

		{
			// anti gravity
			let (x, y) = myship.pos;
			// println!("{}, {}", x, y);
			// assert_eq!(myship.v, (0, 0));
			// println!("{:?}", myship.v);
			let (gx, gy) = gravity(x, y);
			// println!("{}, {}", gx, gy);
			commands.push(
				Command::Accelerate(myship.id, gravity(x, y))
			);
		}
		
		let shoot_now = myship.heat -myship.status.cool + 16 <= myship.max_heat;
		if shoot_now {
			let fix = resp.info.role;
			dx = rng.gen_range(0, 20) + fix;
			dy = rng.gen_range(0, 20) + fix;
			let shoot_power = power;
			commands.push(Command::Shoot(1, (myship.pos.0 + dx, myship.pos.1 + dy), shoot_power, None));
		}

		// !!!
		resp = client.command(&commands);
		// !!!

		if shoot_now {
			// println!("{}", resp);
			println!("S H O O O O O O O T ! ! ! ! ! ! !");
			for ship in resp.state.ships.iter() {
				for cmd in ship.commands.iter() {
					match cmd {
						Command::Shoot(_, (x,y), p, Some((impact, four))) => {
							println!("_, x, y, power, impact, four");
							println!("[shoot], {}, {}, {}, {}, {}",
								x - ship.pos.0,y - ship.pos.1, p, impact, four);
						},
						_=> {},
					}
				}
			}
		}
	}
}

fn main() {
	let _ = ::std::thread::Builder::new()
		.name("run".to_string())
		.stack_size(32 * 1024 * 1024)
		.spawn(run)
		.unwrap()
		.join();
}

fn gravity(x: i32, y: i32) -> (i32, i32) {
	(
		if x.abs() >= y.abs() {
			if x < 0 {
				1
			} else {
				-1
			}
		} else { 0 },

		if x.abs() <= y.abs() {
			if y < 0 {
				1
			} else {
				-1
			}
		} else { 0 },
	)
}