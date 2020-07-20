use app::client::*;

fn get_ship(resp: &Response, id: i32) -> &Ship {
	for ship in &resp.state.ships {
		if ship.id == id {
			return ship;
		}
	}
	panic!()
}

fn get_next_pos(ship: &Ship) -> (i32, i32) {
	(ship.pos.0 + ship.v.0, ship.pos.1 + ship.v.1)
}



fn run() {
	use app::routing::PosVel;

	println!("!!!TOTSUGEKI!!!!");
	let mut router = app::routing::Router::new();

	/*
	dbg!(&router.get_next_move(1, 35, 0, 0, -100, -100));
	return;

	router.get_next_move(50, 50, 0, 0, -50, -50);
	router.get_next_move(30, 30, 0, 0, -20, -20);
	return;
	 */

	let server_url = std::env::args().nth(1).unwrap();
	let mut client = Client::new(server_url);
	if std::env::args().len() == 2 {
		client.send("[1, 0]");
		return;
	}
	let player_key = std::env::args().nth(2).unwrap();

	client.join(&player_key);

	// TODO: sideによってトータル変える
	let cool = 16;
	let mut resp = client.start(448 - cool * 12 - 1 * 2, 0, cool, 1);

	let my_id = resp.state.ships.iter().find_map(|s| if s.role == resp.info.role { Some(s.id) } else { None }).unwrap();
	let en_id = 1 - my_id;  // TODO: 分裂したらやばい・・・・・・しらない・・・・・・

	while resp.stage != 2 {
		let mut commands = vec![];

		// 移動！
		let my_ship = get_ship(&resp, my_id);
		let en_ship = get_ship(&resp, en_id);
		let (dvx, dvy) = router.doit(&my_ship, &en_ship);
		commands.push(Command::Accelerate(my_id, (-dvx, -dvy)));

		// 次ステップのポジで重なるなら爆発！
		let myp = PosVel::from(my_ship).apply_gravity().accelerate_and_move(dvx, dvy);
		let enp = PosVel::from(en_ship).apply_gravity().accelerate_and_move(0, 0);
		if myp.x == enp.x && myp.y == enp.y {
			eprintln!("{}", "BOMB!!!!!!!!".repeat(10));
			commands.push(Command::Detonate(my_id, None));
		}

		resp = client.command(&commands);
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
