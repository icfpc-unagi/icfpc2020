use app::client::*;

pub trait SetMinMax {
	fn setmin(&mut self, v: Self) -> bool;
	fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T where T: PartialOrd {
	fn setmin(&mut self, v: T) -> bool {
		*self > v && { *self = v; true }
	}
	fn setmax(&mut self, v: T) -> bool {
		*self < v && { *self = v; true }
	}
}

macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

pub fn get_time() -> f64 {
	let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
	let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
	ms
}

const INF: i32 = 1000000000;

const W: i32 = 100;


fn check_range(x: i32, y: i32) -> bool {
	x < -W || x > W || y < -W || y > W || -16 <= x && x <= 16 && -16 <= y && y <= 16
}

fn check_v(dx: i32, dy: i32) -> bool {
	dx < -10 || dx > 10 || dy < -10 || dy > 10
}

pub fn test_naive(mut x: i32, mut y: i32, mut dx: i32, mut dy: i32, gx: &Vec<Vec<i32>>, gy: &Vec<Vec<i32>>) -> i32 {
	for t in 1..256 {
		dx += gx[(x + W) as usize][(y + W) as usize];
		dy += gy[(x + W) as usize][(y + W) as usize];
		x += dx;
		y += dy;
		if check_range(x, y) || check_v(dx, dy) {
			return t;
		}
	}
	256
}

struct Preprocess {
	gx: Vec<Vec<i32>>,
	gy: Vec<Vec<i32>>,
	dp: Vec<Vec<Vec<Vec<i32>>>>
}

fn preprocess() -> Preprocess {
	let stime = get_time();
	let n = (W as usize * 2) + 1;
	let m = 21;
	let mut gx = mat![0; n; n];
	let mut gy = mat![0; n; n];
	for i in 0..n {
		let x = i as i32 - W;
		for j in 0..n {
			let y = j as i32 - W;
			if x.abs() > y.abs() {
				if x < 0 {
					gx[i][j] = 1;
				} else {
					gx[i][j] = -1;
				}
			} else if x.abs() < y.abs() {
				if y < 0 {
					gy[i][j] = 1;
				} else {
					gy[i][j] = -1;
				}
			} else {
				if x < 0 {
					gx[i][j] = 1;
				} else if x > 0 {
					gx[i][j] = -1;
				}
				if y < 0 {
					gy[i][j] = 1;
				} else if y > 0 {
					gy[i][j] = -1;
				}
			}
		}
	}
	let mut dp = mat![INF; n; n; m; m];
	let mut que = std::collections::VecDeque::new();
	for i in 0..n {
		let x = i as i32 - W;
		for j in 0..n {
			let y = j as i32 - W;
			for di in 0..m {
				let dx = di as i32 - 10;
				for dj in 0..m {
					let dy = dj as i32 - 10;
					if check_range(x, y) {
						dp[i][j][di][dj] = 0;
					} else {
						let dx = dx + gx[i][j];
						let dy = dy + gy[i][j];
						let x = x + dx;
						let y = y + dy;
						if check_range(x, y) || check_v(dx, dy) {
							dp[i][j][di][dj] = 1;
							que.push_back((i, j, di, dj, 1));
						}
					}
				}
			}
		}
	}
	while let Some((i, j, di, dj, c)) = que.pop_front() {
		let x = i as i32 - W;
		let y = j as i32 - W;
		let dx = di as i32 - 10;
		let dy = dj as i32 - 10;
		let x = x - dx;
		let y = y - dy;
		if check_range(x, y) {
			continue;
		}
		let i = (x + W) as usize;
		let j = (y + W) as usize;
		let dx = dx - gx[i][j];
		let dy = dy - gy[i][j];
		if check_v(dx, dy) {
			continue;
		}
		let di = (dx + 10) as usize;
		let dj = (dy + 10) as usize;
		if dp[i][j][di][dj].setmin(c + 1) {
			que.push_back((i, j, di, dj, c + 1));
		}
	}
	let mut count = 0;
	for i in 0..n {
		for j in 0..n {
			for di in 0..m {
				for dj in 0..m {
					if dp[i][j][di][dj] >= 256 {
					// if test_naive(i as i32 - W, j as i32 - W, di as i32 - 10, dj as i32 - 10, &gx, &gy) >= 256 {
						count += 1;
					}
				}
			}
		}
	}
	eprintln!("preprocessed: {}", count);
	eprintln!("time: {:.3}", get_time() - stime);
	Preprocess { gx, gy, dp }
}

fn rec(x: i32, y: i32, dx: i32, dy: i32, last_ax: i32, last_ay: i32, d: usize, prep: &Preprocess) -> (i32, usize) {
	if check_range(x, y) || check_v(dx, dy) {
		return (0, 0);
	}
	let i = (x + W) as usize;
	let j = (y + W) as usize;
	let di = (dx + 10) as usize;
	let dj = (dy + 10) as usize;
	let mut best = (prep.dp[i][j][di][dj], d);
	if d == 0 {
		return best;
	}
	for ax in -2..=2 {
		if last_ax * ax < 0 {
			continue;
		}
		for ay in -2..=2 {
			if last_ay * ay < 0 {
				continue;
			}
			let dx = dx + ax + prep.gx[i][j];
			let dy = dy + ay + prep.gy[i][j];
			best.setmax(rec(x + dx, y + dy, dx, dy, ax, ay, d - 1, prep));
		}
	}
	best
}

fn next_move(x: i32, y: i32, dx: i32, dy: i32, force: bool, tick: i32, prep: &Preprocess) -> (i32, i32) {
	if !check_range(x, y) && tick > 20 {
		let i = (x + W) as usize;
		let j = (y + W) as usize;
		let mut best = 0;
		let mut best_x = 0;
		let mut best_y = 0;
		for ax in -1..=1 {
			for ay in -1..=1 {
				if force && ax == 0 && ay == 0 {
					continue;
				}
				let dx = dx + ax;
				let dy = dy + ay;
				if check_v(dx, dy) {
					continue;
				}
				let di = (dx + 10) as usize;
				let dj = (dy + 10) as usize;
				if best.setmax(prep.dp[i][j][di][dj]) {
					best_x = ax;
					best_y = ay;
				}
			}
		}
		dbg!(best);
		if best > 0 {
			return (best_x, best_y);
		}
	}
	if check_range(x, y) || check_v(dx, dy) {
		let mut addy = 0;
		let mut addx = 0;
	
		if x.abs() < 30 && y.abs() < 30 {
			if x < 0 { addx = -1; }
			else {addx = 1;}
			if y < 0 { addy = -1; }
			else {addy = 1;}
		}
		else
		{
	
			if x >= 0 && x.abs() >= y.abs() {
				if dy < 7 {
					addy = 1;
					if dx < 0 {addx = 1;}
				}
			}
			if x <= 0 && x.abs() >= y.abs() {
				if dy > -7 { 
					addy = -1;
					if dx > 0 {addx = -1;}
				}
			}
	
			if y >= 0 && y.abs() >= x.abs() {
				if dx > -7 {
					addx = -1;
					if dy < 0 {addy = 1;}
				}
			}
			if y <= 0 && y.abs() >= x.abs() {
				if dx < 7 { 
					addx = 1;
					if dy > 0 {addy = -1;}
				}
			}
		}
	
		if x.abs() > 100{
			if x < 0 { addx = 1; }
			else {addx = -1;}
		}
		
		if y.abs() > 100{
			if y < 0 { addy = 1; }
			else {addy = -1;}
		}
		(addx, addy)
	} else {
		let i = (x + W) as usize;
		let j = (y + W) as usize;
		let mut best = (0, 0);
		let mut best_x = 0;
		let mut best_y = 0;
		for ax in -2..=2 {
			for ay in -2..=2 {
				if force && ax == 0 && ay == 0 {
					continue;
				}
				let dx = dx + ax + prep.gx[i][j];
				let dy = dy + ay + prep.gy[i][j];
				if best.setmax(rec(x + dx, y + dy, dx, dy, ax, ay, 5, prep)) {
					best_x = ax;
					best_y = ay;
				}
			}
		}
		dbg!(best);
		(best_x, best_y)
	}
}

fn run() {
	let prep = preprocess();
	let server_url = std::env::args().nth(1).unwrap();
	let mut client = Client::new(server_url);
	let player_key = std::env::args().nth(2).unwrap();
	let mut resp = client.join(&player_key);
	dbg!(&resp);
	if resp.info.role == 0 {
		app::chokudAI::run(client, resp);
		return;
		// let power = 30;
		// let cool = 10;
		// let life = 1;
		// resp = client.start(512 - power * 4 - cool * 12 - life * 2, power, cool, life);
	} else {
		let power = 0;
		let cool = 8;
		let life = 100;
		resp = client.start(448 - power * 4 - cool * 12 - life * 2, power, cool, life);
	}
	dbg!(&resp);
	while resp.stage != 2 {
		let stime = get_time();
		let mut ship = resp.state.ships[0].clone();
		let mut size = 0;
		for s in &resp.state.ships {
			if s.role == resp.info.role && size.setmax(s.status.life) {
				ship = s.clone();
			}
		}
		let mut count = 0;
		for s in &resp.state.ships {
			if s.role == resp.info.role && s.status.life > 0 && s.pos == ship.pos {
				count += 1;
			}
		}
		let mut commands = vec![];
		if ship.status.life > 1 && ship.status.energy > 1 && count == 1 && !check_range(ship.pos.0, ship.pos.1) && !check_v(ship.v.0, ship.v.1) {
			let i = (ship.pos.0 + W) as usize;
			let j = (ship.pos.1 + W) as usize;
			let di = (ship.v.0 + 10) as usize;
			let dj = (ship.v.1 + 10) as usize;
			if prep.dp[i][j][di][dj] + resp.state.tick > resp.info.deadline {
				commands.push(Command::Split(ship.id, Params { energy: 0, power: 0, cool: 0, life: 1 }));
				eprintln!("split!!!!!!!!!!!!!!!!!!!");
			}
		}
		if commands.len() == 0 {
			let (dx, dy) = next_move(ship.pos.0, ship.pos.1, ship.v.0, ship.v.1, count > 1, resp.state.tick, &prep);
			if dx != 0 || dy != 0 {
				commands.push(Command::Accelerate(ship.id, (-dx, -dy)));
			}
		}
		eprintln!("time = {:.3}", get_time() - stime);
		resp = client.command(&commands);
		dbg!(&resp);
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
