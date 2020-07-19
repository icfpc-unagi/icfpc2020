use reqwest::blocking as reqwest;
use parser::*;
use app::*;
use itertools::*;

#[derive(Debug, Clone)]
pub struct Response {
	pub stage: i32,
	pub info: Info,
	pub state: State
}

#[derive(Debug, Clone)]
pub struct Info {
	pub x0: E,
	pub role: i32,
	pub x2: E,
	pub x3: E,
	pub x4: E,
}

#[derive(Debug, Clone)]
pub struct State {
	pub tick: i32,
	pub x1: E,
	pub ships: Vec<Ship>
}

#[derive(Debug, Clone)]
pub struct Ship {
	pub role: i32,
	pub id: i32,
	pub pos: (i32, i32),
	pub v: (i32, i32),
	pub x4: E,
	pub x5: E,
	pub x6: E,
	pub x7: E,
	pub commands: Vec<Command>,
}

#[derive(Debug, Clone)]
pub enum Command {
	Accelerate(i32, (i32, i32)),
	Detonate(i32),
	Shoot(i32, (i32, i32), i32),
	Unknown,
}

impl std::fmt::Display for Command {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Command::Accelerate(id, v) => {
				write!(f, "[0, {}, <{}, {}>]", id, v.0, v.1)?
			},
			Command::Detonate(id) => {
				write!(f, "[1, {}]", id)?
			},
			Command::Shoot(id, t, x3) => {
				write!(f, "[2, {}, <{}, {}>, {}]", id, t.0, t.1, x3)?
			},
			_ => {
				panic!("unreachable");
			}
		}
		Ok(())
	}
}

impl From<&E> for Command {
	fn from(e: &E) -> Command {
		let e = get_list(e).unwrap();
		match get_num(&e[0]) {
			0 => {
				Command::Accelerate(-1, get_pair(&e[1]))
			},
			1 => {
				Command::Detonate(-1)
			},
			2 => {
				Command::Shoot(-1, get_pair(&e[1]), get_num(&e[2]))
			},
			_ => {
				Command::Unknown
			}
		}
	}
}

struct Client {
	server_url: String,
	player_key: String,
	client: reqwest::Client
}

impl Client {
	pub fn new(server_url: String) -> Self {
		Self {
			server_url,
			player_key: String::new(),
			client: reqwest::Client::new()
		}
	}
	pub fn send(&self, msg: &str) -> E {
		eprintln!("send: {}", msg);
		let msg = to_text(&parse_lisp(msg).0);
		let ss = msg.split_whitespace().collect::<Vec<_>>();
		let (exp, n) = parser::parse(&ss, 0);
		assert_eq!(n, ss.len());
		let e = parser::eval(&exp, true);
		let msg = modulation::modulate(&e);
		eprintln!("send: {}", msg);
		let resp = self.client.post(&self.server_url).body(msg).send().unwrap().text().unwrap();
		eprintln!("resp: {}", resp);
		let resp = modulation::demodulate(&resp);
		eprintln!("resp: {}", resp);
		resp
	}
	pub fn join(&mut self, player_key: &str) -> Response {
		self.player_key = player_key.to_owned();
		let resp = self.send(&format!("[2, {}, [192496425430, 103652820]]", player_key));
		parse(resp)
	}
	pub fn start(&self, x0: i32, x1: i32, x2: i32, x3: i32) -> Response {
		let resp = self.send(&format!("[3, {}, [{}, {}, {}, {}]]", self.player_key, x0, x1, x2, x3));
		parse(resp)
	}
	pub fn command(&self, cs: &[Command]) -> Response {
		let resp = self.send(&format!("[4, {}, [{}]]", self.player_key, cs.iter().join(", ")));
		parse(resp)
	}
}

fn get_num(a: &E) -> i32 {
	if let E::Num(a) = a {
		*a as i32
	} else {
		panic!("not number");
	}
}

fn get_pair(a: &E) -> (i32, i32) {
	if let E::Pair(a, b) = a {
		(get_num(a), get_num(b))
	} else {
		panic!("not pair");
	}
}

pub fn parse(e: E) -> Response {
	let a = get_list(&e).unwrap();
	assert_eq!(a.len(), 4);
	assert_eq!(get_num(&a[0]), 1);
	let stage = get_num(&a[1]);
	let info = get_list(&a[2]).unwrap();
	let x0 = info[0].as_ref().clone();
	let role = get_num(&info[1]);
	let x2 = info[2].as_ref().clone();
	let x3 = info[3].as_ref().clone();
	let x4 = info[4].as_ref().clone();
	let state = get_list(&a[3]).unwrap();
	let (tick, x1, ships) = if state.len() > 0 {
		let tick = get_num(&state[0]);
		let x1 = state[1].as_ref().clone();
		let ships = get_list(&state[2]).unwrap().into_iter().map(|a| {
			let tmp = get_list(&a).unwrap();
			let s = get_list(&tmp[0]).unwrap();
			let commands = get_list(&tmp[1]).unwrap();
			let role = get_num(&s[0]);
			let id = get_num(&s[1]);  // shipId
			let pos = get_pair(&s[2]);
			let v = get_pair(&s[3]);
			let x4 = s[4].as_ref().clone();
			let x5 = s[5].as_ref().clone();
			let x6 = s[6].as_ref().clone();
			let x7 = s[7].as_ref().clone();
			// [1, 1, [256, 1, [448, 2, 128], [16, 128], []], [1, [16, 128], [[[1, 0, <34, -46>, <0, 2>, [445, 0, 0, 1], 8, 128, 2], [[0, <0, -1>]]], [[0, 1, <-34, 48>, <0, 0>, [445, 0, 0, 1], 8, 128, 2], [[0, <0, -1>]]]]]]
			// [src/bin/app.rs:177] &commands = [
			// 	Pair(
			// 		Num(
			// 			0,
			// 		),
			// 		Pair(
			// 			Pair(
			// 				Num(
			// 					0,
			// 				),
			// 				Num(
			// 					-1,
			// 				),
			// 			),
			// 			Nil,
			// 		),
			// 	),
			// ]
			
			let commands = commands.into_iter().map(|e| e.as_ref().into()).collect();
			Ship {
				role,
				id,
				pos,
				v,
				x4,
				x5,
				x6,
				x7,
				commands
			}
		}).collect();
		(tick, x1, ships)
	} else {
		(0, E::Nil, vec![])
	};
	Response {
		stage,
		info: Info {
			x0, role, x2, x3, x4
		},
		state: State {
			tick, x1, ships
		}
	}
}

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
