use crate::parser::*;
use crate::*;
use ::reqwest::blocking as reqwest;
use itertools::*;
use std::cell::RefCell;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::ops::Range;
use std::time::SystemTime;
use std::sync::{Arc, Mutex};
use chrono::{Utc, Local, DateTime, NaiveDateTime};

lazy_static! {
    static ref last_send: Arc<Mutex<Option<DateTime<Utc>>>> =
        Arc::new(Mutex::new(None));
}

#[derive(Debug, Clone)]
pub struct Response {
	pub stage: i32,
	pub info: Info,
	pub state: State,
}

fn response_to_json(x: &Response) -> String {
	map_to_json(vec![
		("stage", format!("{}", x.stage)),
		("info", info_to_json(&x.info)),
		("state", state_to_json(&x.state)),
	])
}

#[derive(Debug, Clone)]
pub struct Info {
	pub deadline: i32,
	pub role: i32,
	pub ability: Ability,
	pub range: Range<i32>,
	pub opponent_params: Params,
}

fn info_to_json(x: &Info) -> String {
	map_to_json(vec![
		("deadline", format!("{}", x.deadline)),
		("role", format!("{}", x.role)),
		("opponent_params", params_to_json(&x.opponent_params)),
	])
}

#[derive(Debug, Clone)]
pub struct Ability {
	pub potential: i32,
	pub max_accelarate: i32,
	pub max_heat: i32,
}

#[derive(Debug, Clone)]
pub struct State {
	pub tick: i32,
	pub range: Range<i32>, // 侵入可能エリアの x,y の絶対値の範囲
	pub ships: Vec<Ship>,
}

fn state_to_json(x: &State) -> String {
	let mut ships = Vec::new();
	for s in &x.ships {
		ships.push(ship_to_json(&s));
	}
	map_to_json(vec![
		("tick", format!("{}", x.tick)),
		("ships", format!("[{}]", ships.join(","))),
	])
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
	pub max_accelarate: i32,
	pub commands: Vec<Command>,
}

fn ship_to_json(x: &Ship) -> String {
	let mut commands = Vec::new();
	for c in &x.commands {
		commands.push(command_to_json(&c));
	}
	map_to_json(vec![
		("role", format!("{}", x.role)),
		("x", format!("{}", x.pos.0)),
		("y", format!("{}", x.pos.1)),
		("vx", format!("{}", x.v.0)),
		("vy", format!("{}", x.v.1)),
		("status", params_to_json(&x.status)),
		("heat", format!("{}", x.heat)),
		("max_heat", format!("{}", x.max_heat)),
		("max_accelarate", format!("{}", x.max_accelarate)),
		("commands", format!("[{}]", commands.connect(","))),
	])
}

#[derive(Debug, Clone)]
pub enum Command {
	Accelerate(i32, (i32, i32)),
	Detonate(i32, Option<(i32, i32)>),               // 1, (impact, 32)
	Shoot(i32, (i32, i32), i32, Option<(i32, i32)>), // 2, target, power, (impact, 4)
	Split(i32, Params),
	Unknown,
}

fn command_to_json(x: &Command) -> String {
	match x {
		Command::Accelerate(id, (x, y)) => format!(
			"{{\"type\":\"accelerate\",\"id\":{},\"x\":{},\"y\":{}}}", id, x, y),
		Command::Detonate(id, _) => format!(
			"{{\"type\":\"detonate\",\"id\":{}}}", id),
		Command::Shoot(id, (x, y), power, _) => format!(
			"{{\"type\":\"shoot\",\"id\":{},\"x\":{},\"y\":{},\"power\":{}}}",
			id, x, y, power),
		Command::Split(id, params) => format!(
			"{{\"type\":\"split\",\"id\":{},\"params\":{}}}",
			id, params_to_json(&params)),
		_ => format!("{{}}"),
	}
}

#[derive(Debug, Clone)]
pub struct Params {
	pub energy: i32,
	pub power: i32,
	pub cool: i32,
	pub life: i32,
}

fn params_to_json(x: &Params) -> String {
	format!("{{\"energy\":{},\"power\":{},\"cool\":{},\"life\":{}}}",
		x.energy, x.power, x.cool, x.life)
}

fn map_to_json(m: Vec<(&str, String)>) -> String {
	let mut kvs = Vec::new();
	for kv in m {
		kvs.push(format!("\"{}\":{}", kv.0, kv.1));
	}
	format!("{{{}}}", kvs.join(","))
}

impl std::fmt::Display for Command {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Command::Accelerate(id, v) => write!(f, "[0, {}, <{}, {}>]", id, v.0, v.1)?,
			Command::Detonate(id, None) => write!(f, "[1, {}]", id)?,
			Command::Detonate(id, Some((a, b))) => write!(f, "[1, {}, {}, {}]", id, a, b)?,
			Command::Shoot(id, t, p, None) => write!(f, "[2, {}, <{}, {}>, {}]", id, t.0, t.1, p)?,
			Command::Shoot(id, t, p, Some((a, b))) => {
				write!(f, "[2, {}, <{}, {}>, {}, {}, {}]", id, t.0, t.1, p, a, b)?
			}
			Command::Split(id, params) => write!(
				f,
				"[3, {}, [{}, {}, {}, {}]]",
				id, params.energy, params.power, params.cool, params.life
			)?,
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
			0 => Command::Accelerate(-1, get_pair(&e[1])),
			1 => Command::Detonate(
				-1,
				if e.len() < 3 {
					None
				} else {
					Some((get_num(&e[1]), get_num(&e[2])))
				},
			),
			2 => Command::Shoot(
				-1,
				get_pair(&e[1]),
				get_num(&e[2]),
				if e.len() < 5 {
					None
				} else {
					Some((get_num(&e[3]), get_num(&e[4])))
				},
			),
			3 => {
				let params = get_list(&e[1])
					.unwrap()
					.into_iter()
					.map(|e| get_num(&e))
					.collect::<Vec<_>>();
				Command::Split(
					-1,
					Params {
						energy: params[0],
						power: params[1],
						cool: params[2],
						life: params[3],
					},
				)
			}
			_ => Command::Unknown,
		}
	}
}

pub struct Client {
	server_url: String,
	player_key: String,
	file: Option<RefCell<BufWriter<File>>>,
	client: reqwest::Client,
}

impl Client {
	pub fn new(server_url: String) -> Self {
		let server_url = if server_url.contains("?apiKey") {
			server_url
		} else {
			server_url + "/aliens/send"
		};
		Self {
			server_url,
			player_key: String::new(),
			file: None,
			client: reqwest::Client::new(),
		}
	}

	pub fn gui(&self, name: &str, msg: &str) {
		if let Ok(_) = env::var("JUDGE_SERVER") {
			return;
		}
		let t = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
			Ok(t) => t.as_nanos(),
			_ => 0,
		};
		let msg = format!("###GUI\t{}\t{}\t{}\t{}\n", t, self.player_key, name, msg);
		let mut printed = false;
		if let Some(f) = &self.file {
			f.borrow_mut()
				.write_all(msg.as_bytes())
				.expect("Failed to write to file");
			printed = true;
		}
		if let Ok(_) = env::var("GUI") {
			print!("{}", &msg);
		} else if !printed {
			print!("{}", &msg);
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
		// eprintln!("send: {}", msg);
		if let Ok(mut guard) = last_send.lock() {
			if let Some(t) = guard.clone() {
				let duration = Utc::now() - t;
				if duration.num_milliseconds() > 500 {
					eprintln!("############################################################");
					eprintln!("AI took too much CPU time! ({} ms)", duration.num_milliseconds());
					eprintln!("############################################################");
				}
				eprintln!("AI took {} ms.", duration.num_milliseconds());
			} else {
				eprintln!("First send request.");
			}
		}
		let resp = self
			.client
			.post(&self.server_url)
			.body(msg)
			.send()
			.unwrap()
			.text()
			.unwrap();
		if let Ok(mut guard) = last_send.lock() {
			*guard = Some(Utc::now());
		}
		// eprintln!("resp: {}", resp);
		let resp = modulation::demodulate(&resp);
		eprintln!("resp: {}", &resp);
		if let Some(state) = &resp.into_iter().skip(3).next() {
			if let Some(ship_and_cmds) = state.into_iter().skip(2).next() {
				for ship_and_cmd in ship_and_cmds {
					eprintln!("ship: {}", &ship_and_cmd);
				}
			}
		}
		resp
	}
	pub fn join(&mut self, player_key: &str) -> Response {
		self.player_key = player_key.to_owned();
		if let Err(_) = env::var("JUDGE_SERVER") {
			self.file = Some(RefCell::new(BufWriter::new(
				File::create(&format!("out/{}", self.player_key)).expect("out directory is missing"),
			)));
		}
		let resp = self.send(&format!("[2, {}, [192496425430, 103652820]]", player_key));
		parse(resp)
	}
	pub fn start(&self, energy: i32, power: i32, cool: i32, life: i32) -> Response {
		let resp = self.send(&format!(
			"[3, {}, [{}, {}, {}, {}]]",
			self.player_key, energy, power, cool, life
		));
		parse(resp)
	}
	pub fn command(&self, cs: &[Command]) -> Response {
		let resp = self.send(&format!(
			"[4, {}, [{}]]",
			self.player_key,
			cs.iter().join(", ")
		));
		let resp = parse(resp);
		self.gui("RESP", &response_to_json(&resp));
		return resp;
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
	let deadline = get_num(&info[0]);
	let role = get_num(&info[1]);
	let ability = get_list(&info[2])
		.unwrap()
		.into_iter()
		.map(|e| get_num(&e))
		.collect::<Vec<_>>();
	let ability = Ability {
		potential: ability[0],
		max_heat: ability[1],
		max_accelarate: ability[2],
	};
	let range = get_list(&info[3])
		.unwrap()
		.into_iter()
		.map(|e| get_num(&e))
		.collect::<Vec<_>>();
	let range = range[0]..range[1];
	let params = get_list(&info[4])
		.unwrap()
		.into_iter()
		.map(|e| get_num(&e))
		.collect::<Vec<_>>();
	let opponent_params = if params.len() != 4 {
		Params {
			energy: -1,
			power: -1,
			cool: -1,
			life: -1,
		}
	} else {
		Params {
			energy: params[0],
			power: params[1],
			cool: params[2],
			life: params[3],
		}
	};
	let state = get_list(&a[3]).unwrap();
	let (tick, strange, ships) = if state.len() > 0 {
		let tick = get_num(&state[0]);
		let strange = get_list(&state[1])
			.unwrap()
			.into_iter()
			.map(|e| get_num(&e))
			.collect::<Vec<i32>>();
		let strange = strange[0]..strange[1];
		let ships = get_list(&state[2])
			.unwrap()
			.into_iter()
			.map(|a| {
				let tmp = get_list(&a).unwrap();
				let s = get_list(&tmp[0]).unwrap();
				let commands = get_list(&tmp[1]).unwrap();
				let role = get_num(&s[0]);
				let id = get_num(&s[1]); // shipId
				let pos = get_pair(&s[2]);
				let v = get_pair(&s[3]);
				let status = get_list(&s[4])
					.unwrap()
					.into_iter()
					.map(|e| get_num(&e))
					.collect::<Vec<_>>();
				let status = Params {
					energy: status[0],
					power: status[1],
					cool: status[2],
					life: status[3],
				};
				let heat = get_num(&s[5]);
				let max_heat = get_num(&s[6]);
				let max_accelarate = get_num(&s[7]);
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
					max_accelarate,
					commands,
				}
			})
			.collect();
		(tick, strange, ships)
	} else {
		(0, 0..0, vec![])
	};
	Response {
		stage,
		info: Info {
			deadline,
			role,
			ability,
			range,
			opponent_params,
		},
		state: State {
			tick,
			range: strange,
			ships,
		},
	}
}
