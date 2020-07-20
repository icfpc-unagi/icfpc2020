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
	let role = resp.info.role;
	println!("{}", role);
	let power = if role == 0 {
		100
	} else {
		0
	};
	let cool = 4;
	let life = 1;
	resp = client.start(resp.info.ability.potential - power * 4 - cool * 12 - life * 2, power, cool, life);

	// let mut dx: i32 = std::env::var("DX").unwrap().parse().unwrap();
	// let dy: i32 = std::env::var("DY").unwrap().parse().unwrap();
	let mut dx = 0;
	let mut dy = 0;
	let mut shoot_flag = false;
	let mut teki_energy = 0;
	while resp.stage != 2 {
		let mut myship = None;
		let mut tekiship = None;
		for ship in resp.state.ships.iter() {
			if ship.role != resp.info.role {
				tekiship = Some(ship.clone());
			}
			myship = Some(ship.clone());
		}
		let myship = myship.unwrap();
		let tekiship = tekiship.unwrap();


		if shoot_flag {
			// println!("{}", resp);
			println!("S H O O O O O O O T ? ? ? ? ? ? ?");
			for cmd in myship.commands.iter() {
				match cmd {
					Command::Shoot(_, (x,y), p, Some((impact, four))) => {
						println!("S H O O O O O O O T ! ! ! ! ! ! !");
						println!("[shoot], {}, {}, {}, {}, {}, {}, {}, {}",
							x - myship.pos.0, y - myship.pos.1,
							x - tekiship.pos.0, y - tekiship.pos.1,
							tekiship.status.energy - teki_energy,
							p, impact, four);
					},
					_=> {},
				}
			}
		}
		teki_energy = tekiship.status.energy;

		let mut commands = vec![];

		{
			// anti gravity
			let (x, y) = myship.pos;
			println!("{}, {}", x, y);
			// assert_eq!(myship.v, (0, 0));
			println!("{:?}", myship.v);
			let (gx, gy) = gravity(x, y);
			println!("{}, {}", gx, gy);
			commands.push(
				Command::Accelerate(myship.id, gravity(x, y))
			);
		}
		
		shoot_flag = role == 0;
		if shoot_flag {
			println!("S H O O O O O O O T ? ? ?");
			println!("{}, {}, {}", myship.heat, myship.status.cool, myship.max_heat);
			let fix = resp.info.role;
			dx = rng.gen_range(0, 2);
			dy = dx + rng.gen_range(0, 2);
			// dy = rng.gen_range(0, 20) + fix;
			let shoot_power = power;
			let (mut x, mut y) = tekiship.pos;
			x += tekiship.v.0;
			y += tekiship.v.1;
			commands.push(Command::Shoot(myship.id, (x + dx, y + dy), shoot_power, None));
		}

		// !!!
		resp = client.command(&commands);
		// !!!
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