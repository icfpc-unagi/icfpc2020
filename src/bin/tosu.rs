use app::client::*;

fn run() {
	let server_url = std::env::args().nth(1).unwrap();
	let mut client = Client::new(server_url);
	if std::env::args().len() == 2 {
		client.send("[1, 0]");
		return;
	}
	let player_key = std::env::args().nth(2).unwrap();
	let mut resp = client.join(&player_key);
	let power = 0;
	let cool = 0;
	let life = 1;
	resp = client.start(
		resp.info.ability.potential - power * 4 - cool * 12 - life * 2,
		power,
		cool,
		life,
	);

	let dx: i32 = std::env::var("DX").unwrap().parse().unwrap();
	let dy: i32 = std::env::var("DY").unwrap().parse().unwrap();
	let dx = if resp.info.role == 0 { dx } else { -dx };
	let dy = if resp.info.role == 0 { dy } else { -dy };
	let mut router = app::routing::Router::new();
	while resp.stage != 2 {
		let myship = resp
			.state
			.ships
			.iter()
			.find(|x| x.role == resp.info.role)
			.unwrap();
		let enship = resp
			.state
			.ships
			.iter()
			.find(|x| x.role != resp.info.role)
			.unwrap();

		let mut commands = vec![];

		let (egx, egy) = gravity(enship.pos.0, enship.pos.1);
		let (ex, ey) = (
			enship.pos.0 + enship.v.0 + egx - dx,
			enship.pos.1 + enship.v.1 + egy - dy,
		);
		let (mgx, mgy) = gravity(myship.pos.0, myship.pos.1);
		let (mx, my) = (
			myship.pos.0 + myship.v.0 + mgx,
			myship.pos.1 + myship.v.1 + mgy,
		);
		let (ax, ay) = (ex - mx, ey - my);
		let detonate = ax.abs() <= 2 && ay.abs() <= 2;
		if detonate {
			eprintln!("DETONATE");
			if resp.info.role == 0 {
				if ax != 0 || ay != 0 {
					commands.push(Command::Accelerate(myship.id, (-ax, -ay)));
				}
				commands.push(Command::Detonate(myship.id, None));
			}
		} else {
			eprintln!("going to {},{}", ex, ey);
			let (dvx, dvy) = router
				.get_next_move(myship.pos.0, myship.pos.1, myship.v.0, myship.v.1, ex, ey)
				.0;
			commands.push(Command::Accelerate(myship.id, (-dvx, -dvy)));
		}
		// !!!
		let new_resp = client.command(&commands);
		// !!!

		if detonate {
			let new_myship = new_resp
				.state
				.ships
				.iter()
				.find(|x| x.role == new_resp.info.role)
				.unwrap();
			let new_enship = new_resp
				.state
				.ships
				.iter()
				.find(|x| x.role != new_resp.info.role)
				.unwrap();
			let mut consumed = 0;
			for cmd in &new_myship.commands {
				if let Command::Accelerate(_, (x, y)) = cmd {
					consumed = x.abs().max(y.abs()) * 8;
				}
			}
			let damage = new_enship.heat - enship.heat; // excl. cool
			for cmd in &new_myship.commands {
				if let Command::Detonate(_, Some((impact, thirtytwo))) = cmd {
					println!(
						"detonate,{},{},{},{},{},{}",
						new_enship.pos.0 - new_myship.pos.0,
						new_enship.pos.1 - new_myship.pos.1,
						myship.status.energy - consumed,
						damage,
						impact,
						thirtytwo,
					);
				}
			}
		}
		resp = new_resp;
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
		} else {
			0
		},
		if x.abs() <= y.abs() {
			if y < 0 {
				1
			} else {
				-1
			}
		} else {
			0
		},
	)
}
