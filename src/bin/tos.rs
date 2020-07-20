use app::client::*;

fn run() {
	let server_url = std::env::args().nth(1).unwrap();
	let mut client = Client::new(server_url);
	if std::env::args().len() == 2 {
		client.send("[1, 0]");
		return;
	}
	let player_key = std::env::args().nth(2).unwrap();
	client.join(&player_key);
	let mut resp = client.start(446, 0, 0, 1);
	let id = resp.state.ships.iter().find_map(|s| if s.role == resp.info.role { Some(s.id) } else { None }).unwrap();
	while resp.stage != 2 {
		resp = client.command(&[Command::Accelerate(id, (0, -1))]);
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
