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

fn preprocess() -> Vec<Vec<Vec<Vec<i32>>>> {
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
	dp
}

fn next_move(x: i32, y: i32, dx: i32, dy: i32, force: bool, tick: i32, dp: &Vec<Vec<Vec<Vec<i32>>>>) -> (i32, i32) {
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
				if best.setmax(dp[i][j][di][dj]) {
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
}

fn run() {
	let dp = preprocess();
	let server_url = std::env::args().nth(1).unwrap();
	let mut client = Client::new(server_url);
	let player_key = std::env::args().nth(2).unwrap();
	let mut resp = client.join(&player_key);
	dbg!(&resp);
	if resp.info.role == 0 {
		chokudai::run(client, resp);
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
			if dp[i][j][di][dj] + resp.state.tick > resp.info.deadline {
				commands.push(Command::Split(ship.id, Params { energy: 1, power: 0, cool: 0, life: 1 }));
				eprintln!("split!!!!!!!!!!!!!!!!!!!");
			}
		}
		if commands.len() == 0 {
			let (dx, dy) = next_move(ship.pos.0, ship.pos.1, ship.v.0, ship.v.1, count > 1, resp.state.tick, &dp);
			if dx != 0 || dy != 0 {
				commands.push(Command::Accelerate(ship.id, (-dx, -dy)));
			}
		}
		resp = client.command(&commands);
		dbg!(&resp);
	}
}

mod chokudai {
	use app::client::*;
	use rand::prelude::*;
	
	struct EnemyData{
		pub pattern: Vec<Vec<i32>>,
	
	}
	
	pub fn run(client: Client, join_resp: Response){
		//START
		//set firstなんとか
		
		let mut all = 448;
		if join_resp.info.role == 0 { all = 512; }
		let shoot = 64;
		let heal = 10;
		let life = 1;
		let energy = all - shoot * 4 - heal * 12 - life * 2;
		
		let mut resp = client.start(energy, shoot, heal, life);
	
		let mut e_data = EnemyData{
			pattern: vec![vec![0;5];5],
		};
	
		let id = resp.state.ships.iter().find_map(|s| if s.role == resp.info.role { Some(s.id) } else { None }).unwrap();
		let my_role = join_resp.info.role;
		//COMMANDS
		while resp.stage != 2 {
	
			resp = client.command(&chokud_ai(&resp, &id, &my_role, &e_data));
			//dbg!(&resp);
		}
	
	
	
	}
	
	fn chokud_ai(resp: &Response, id: &i32, my_role: &i32, e_data: &EnemyData) -> Vec<Command> {
		
		let mut myship = resp.state.ships[0].clone();
		let mut enemyship = resp.state.ships[0].clone();
	
		for i in 0..resp.state.ships.len() {
			let nowship = resp.state.ships[i].clone();
			if nowship.role == *my_role {myship = nowship; }
			else {
				enemyship = nowship;
				
			}
		}
	
	
		let mut next_enemy = vec![enemyship.pos.0 + enemyship.v.0, enemyship.pos.1 + enemyship.v.1];
	
		
		if enemyship.pos.0.abs() <= enemyship.pos.1.abs(){
			if enemyship.pos.1 >= 0 {
				next_enemy[1] -= 1;
			}
			else{
				next_enemy[1] += 1;
			}
		}
		if enemyship.pos.1.abs() <= enemyship.pos.0.abs(){
			if enemyship.pos.0 >= 0 {
				next_enemy[0] -= 1;
			}
			else{
				next_enemy[0] += 1;
			}
		}
	
		let mut enemyMoveX = 0;
		let mut enemyMoveY = 0;
		let mut enemyCnt = 0;
		for x in 0..5 {
			for y in 0..5 {
				if e_data.pattern[x][y] > enemyCnt {
					enemyCnt = e_data.pattern[x][y];
					enemyMoveX = x - 2;
					enemyMoveY = y - 2;
				}
			}
		}
		
	
		let mut addy = 0;
		let mut addx = 0;
	
		if myship.pos.0.abs() < 30 && myship.pos.1.abs() < 30{
			if myship.pos.0 < 0 { addx = -1; }
			else {addx = 1;}
			if myship.pos.1 < 0 { addy = -1; }
			else {addy = 1;}
		}
		else
		{
	
			if myship.pos.0 >= 0 && myship.pos.0.abs() >= myship.pos.1.abs() {
				if myship.v.1 < 7 {
					addy = 1;
					if myship.v.0 < 0 {addx = 1;}
				}
			}
			if myship.pos.0 <= 0 && myship.pos.0.abs() >= myship.pos.1.abs() {
				if myship.v.1 > -7 { 
					addy = -1;
					if myship.v.0 > 0 {addx = -1;}
				}
			}
	
			if myship.pos.1 >= 0 && myship.pos.1.abs() >= myship.pos.0.abs() {
				if myship.v.0 > -7 {
					addx = -1;
					if myship.v.1 < 0 {addy = 1;}
				}
			}
			if myship.pos.1 <= 0 && myship.pos.1.abs() >= myship.pos.0.abs() {
				if myship.v.0 < 7 { 
					addx = 1;
					if myship.v.1 > 0 {addy = -1;}
				}
			}
		}
	
		if myship.pos.0.abs() > 100{
			if myship.pos.0 < 0 { addx = 1; }
			else {addx = -1;}
		}
		
		if myship.pos.1.abs() > 100{
			if myship.pos.1 < 0 { addy = 1; }
			else {addy = -1;}
		}
	
		
		if myship.v.0.abs() > 10{
			if myship.v.0 < 0 { addx = 1; }
			else {addx = -1;}
		}
		
		if myship.v.1.abs() > 10{
			if myship.v.1 < 0 { addy = 1; }
			else {addy = -1;}
		}
	
		let mut shoot_flag = false;
		let mut accelerate_flag = false;
		let mut shooty = next_enemy[0];
		let mut shootx = next_enemy[1];
	
		let mut next_me = vec![myship.pos.0+myship.v.0+addx, myship.pos.1+myship.v.1+addy];
	
	
		if myship.heat <= myship.max_heat - 60 {shoot_flag = true;}
	
		let maxlen = (next_me[0]-next_enemy[0]).abs().max( (next_me[1]-next_enemy[1]).abs());
		let minlen = (next_me[0]-next_enemy[0]).abs().min( (next_me[1]-next_enemy[1]).abs());
	
		let terrible_angle = (maxlen * 2 / 10 <= minlen) && (maxlen * 8 / 10 >= minlen);
		let bad_angle = (maxlen * 1 / 10 <= minlen) && (maxlen * 9 / 10 >= minlen);
	
		if !bad_angle || (!terrible_angle && minlen + maxlen <= 35) {
			if addx == 0 && addy == 0 {
				if minlen + maxlen <= 70 && enemyship.max_heat - enemyship.heat >= 30 && enemyship.status.power >= 30 {
					let num = thread_rng().gen_range(0, 4);
					addx = num / 2 * 2 - 1;
					addy = num % 2 * 2 - 1;
				}
			}
		}
		else{
			shoot_flag = false;
		}
	
		eprintln!("debug {} {} {} {} {} {} {}", myship.status.energy, myship.pos.0, myship.pos.1, myship.v.0, myship.v.1, addx, addy);
	
	
		if addy != 0 || addx != 0 {accelerate_flag = true; }
		let mut ret = vec![];
	
		if shoot_flag{
			ret.push(Command::Shoot(*id, (shooty, shootx), 64));
		}
	
		if accelerate_flag {
			ret.push(Command::Accelerate(*id, (-addx, -addy)));
		}
		return ret;
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
