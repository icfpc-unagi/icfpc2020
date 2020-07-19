use ::reqwest::blocking as reqwest;
use crate::parser::*;
use crate::*;
use itertools::*;

#[derive(Debug, Clone)]
pub struct Response {
	pub stage: i32,
	pub info: Info,
	pub state: State
}

#[derive(Debug, Clone)]
pub struct Info {
	pub x0: i32,
	pub role: i32,
	pub x2: Vec<i32>,
	pub x3: Vec<i32>,
	pub opponent_params: Params,
}

#[derive(Debug, Clone)]
pub struct State {
	pub tick: i32,
	pub x1: Vec<i32>,
	pub ships: Vec<Ship>
}

#[derive(Debug, Clone)]
pub struct Ship {
	pub role: i32,
	pub id: i32,
	pub pos: (i32, i32),
	pub v: (i32, i32),
	pub status: Params,
	pub heat: i32,
	pub max_heat: i32,
	pub x7: i32,
	pub commands: Vec<Command>,
}

#[derive(Debug, Clone)]
pub enum Command {
	Accelerate(i32, (i32, i32)),
	Detonate(i32),
	Shoot(i32, (i32, i32), i32),
	Unknown,
}

#[derive(Debug, Clone)]
pub struct Params {
	pub energy: i32,
	pub power: i32,
	pub cool: i32,
	pub split: i32,
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

pub struct Client {
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
	pub fn start(&self, energy: i32, power: i32, cool: i32, split: i32) -> Response {
		let resp = self.send(&format!("[3, {}, [{}, {}, {}, {}]]", self.player_key, energy, power, cool, split));
		parse(resp)
	}
	pub fn command(&self, cs: &[Command]) -> Response {
		let resp = self.send(&format!("[4, {}, [{}]]", self.player_key, cs.iter().join(", ")));
		parse(resp)
	}
}

pub fn get_num(a: &E) -> i32 {
	if let E::Num(a) = a {
		*a as i32
	} else {
		panic!("not number");
	}
}

pub fn get_pair(a: &E) -> (i32, i32) {
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
	let x0 = get_num(&info[0]);
	let role = get_num(&info[1]);
	let x2 = get_list(&info[2]).unwrap().into_iter().map(|e| get_num(&e)).collect();
	let x3 = get_list(&info[3]).unwrap().into_iter().map(|e| get_num(&e)).collect();
	let params = get_list(&info[4]).unwrap().into_iter().map(|e| get_num(&e)).collect::<Vec<_>>();
	let opponent_params = if params.len() != 4 {
		Params {
			energy: -1,
			power: -1,
			cool: -1,
			split: -1,
		}
	} else {
		Params {
			energy: params[0],
			power: params[1],
			cool: params[2],
			split: params[3],
		}
	};
	let state = get_list(&a[3]).unwrap();
	let (tick, x1, ships) = if state.len() > 0 {
		let tick = get_num(&state[0]);
		let x1 = get_list(&state[1]).unwrap().into_iter().map(|e| get_num(&e)).collect();
		let ships = get_list(&state[2]).unwrap().into_iter().map(|a| {
			let tmp = get_list(&a).unwrap();
			let s = get_list(&tmp[0]).unwrap();
			let commands = get_list(&tmp[1]).unwrap();
			let role = get_num(&s[0]);
			let id = get_num(&s[1]);  // shipId
			let pos = get_pair(&s[2]);
			let v = get_pair(&s[3]);
			let status = get_list(&s[4]).unwrap().into_iter().map(|e| get_num(&e)).collect::<Vec<_>>();
			let status = Params {
				energy: status[0],
				power: status[1],
				cool: status[2],
				split: status[3],
			};
			let heat = get_num(&s[5]);
			let max_heat = get_num(&s[6]);
			let x7 = get_num(&s[7]);
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
				status,
				heat,
				max_heat,
				x7,
				commands
			}
		}).collect();
		(tick, x1, ships)
	} else {
		(0, vec![], vec![])
	};
	Response {
		stage,
		info: Info {
			x0, role, x2, x3, opponent_params
		},
		state: State {
			tick, x1, ships
		}
	}
}
