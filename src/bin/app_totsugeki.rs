use app::client::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

const SIZE_OUTER: i32 = 128;
const SIZE_INNER: i32 = 16;
const MAX_V: i32 = 16;
const STEP_LIMIT: i32 = 5;

fn clip_int(x: i32, limit: i32) -> i32 {
	x.signum() * x.abs().min(limit)
}

fn clip_pos(x: i32) -> i32 {
	clip_int(x, SIZE_OUTER - 1)
}

fn clip_vel(x: i32) -> i32 {
	clip_int(x, MAX_V - 1)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Copy, Clone, Debug)]
struct PosVel {
	x: i32,
	y: i32,
	vx: i32,
	vy: i32,
}

impl PosVel {
	pub fn new(x: i32, y: i32, vx: i32, vy: i32) -> Self {
		Self {
			x,
			y,
			vx,
			vy
		}
	}

	pub fn new_empty() -> Self {
		Self {
			x: i32::MIN,
			y: i32::MIN,
			vx: i32::MIN,
			vy: i32::MIN,
		}
	}

	pub fn is_empty(&self) -> bool {
		self.x == i32::MIN && self.y == i32::MIN && self.vx == i32::MIN && self.vy == i32::MIN
	}

	pub fn get_gravity(&self) -> (i32, i32) {
		let apply_x = self.x.abs() >= self.y.abs();
		let apply_y = self.x.abs() <= self.y.abs();

		let gx =  {
			if apply_x {
				-self.x.signum()
			} else {
				0
			}
		};
		let gy = {
			if apply_y {
				-self.y.signum()
			} else {
				0
			}
		};

		(gx, gy)
	}

	pub fn apply_gravity(&self) -> Self {
		let (gx, gy) = self.get_gravity();
		Self {
			x: self.x,
			y: self.y,
			vx: self.vx + gx,
			vy: self.vy + gy,
		}
	}

	pub fn accelerate_and_move(&self, dvx: i32, dvy: i32) -> Self {
		let vx = self.vx + dvx;
		let vy = self.vy + dvy;
		Self {
			x: self.x + vx,
			y: self.y + vy,
			vx,
			vy,
		}
	}

	pub fn is_in_valid_area(&self) -> bool {
		if SIZE_OUTER <= self.x.abs() || SIZE_OUTER <= self.y.abs()  {
			return false;
		}
		if self.x.abs() <= SIZE_INNER && self.y.abs() <= SIZE_INNER {
			return false;
		}
		if MAX_V <= self.vx.abs() || MAX_V <= self.vy.abs() {
			return false;
		}
		true
	}

	pub fn hypot_to(&self, mut x: i32, mut y: i32) -> i32 {
		x -= self.x;
		y -= self.y;
		x * x + y * y
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Copy, Clone, Debug)]
struct BinaryHeapState {
	cst: i32,
	pv: PosVel,
}

impl PartialOrd for BinaryHeapState {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		other.cst.partial_cmp(&self.cst)
	}
}

impl Eq for BinaryHeapState {}

impl Ord for BinaryHeapState {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.partial_cmp(other).unwrap()
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

struct Router {
	mem: Vec<Vec<Vec<Vec<(usize, (i32, PosVel))>>>>,
	uninitialized: (i32, PosVel),
	ver: usize,
}

impl Router {
	fn new() -> Self {
		let uninitialized = (i32::MAX, PosVel::new_empty());
		Self {
			mem: vec![vec![vec![vec![(usize::MAX, uninitialized); (SIZE_OUTER * 2) as usize]; (SIZE_OUTER * 2) as usize]; (MAX_V * 2) as usize]; (MAX_V * 2) as usize],
			ver: 0,
			uninitialized,
		}
	}

	fn get(&self, s: &PosVel) -> &(i32, PosVel) {
		let m = &self.mem[(s.vy + MAX_V) as usize][(s.vx + MAX_V) as usize][(s.y + SIZE_OUTER) as usize][(s.x + SIZE_OUTER) as usize];
		if m.0 == self.ver {
			&m.1
		} else {
			&self.uninitialized
		}
	}

	fn set(&mut self, s: &PosVel, value: (i32, PosVel)) {
		self.mem[(s.vy + MAX_V) as usize][(s.vx + MAX_V) as usize][(s.y + SIZE_OUTER) as usize][(s.x + SIZE_OUTER) as usize] = (self.ver, value);
	}

	/// 次にするべき加速を返す
	///
	/// TODO: memを毎回初期化するのをやめる
	/// TODO: a starにする
	/// TODO: 早くなったらvelocity上限あげたい
	fn get_next_move(&mut self, sx: i32, sy: i32, vx: i32, vy: i32, tx: i32, ty: i32) -> ((i32, i32), i32) {
		// できればこれが起こるべきではない（外側でこういうパターンに対してケアされているべき）がout of boundsで死ぬよりよい
		let sx = clip_pos(sx);
		let sy = clip_pos(sy);
		let vx = clip_int(vx, MAX_V);
		let vy = clip_int(vy, MAX_V);

		// self.mem = vec![vec![vec![vec![(i32::MAX, PosVel::new_empty()); (SIZE_OUTER * 2) as usize]; (SIZE_OUTER * 2) as usize]; (MAX_V * 2) as usize]; (MAX_V * 2) as usize];
		self.ver += 1;  // これが事実上の配列クリアや！

		let mut que = std::collections::BinaryHeap::new();
		let pv = PosVel::new(sx, sy, vx, vy);
		self.set(&pv, (0, PosVel::new_empty()));
		que.push(BinaryHeapState {
			cst: 0,
			pv,
		});

		let mut best_entry = (i32::MAX, i32::MAX, PosVel::new_empty());
		while let Some(s) = que.pop() {
			let hypot = s.pv.hypot_to(tx, ty);
			if s.cst > 0 && (hypot, s.cst) < (best_entry.0, best_entry.1) {
				// s.cst == 0を除外するのは、これを入れちゃうと、離れるしかないときにすぐ虚無になっちゃうから
				best_entry = (hypot, s.cst, s.pv);
			}

			if s.cst >= STEP_LIMIT {
				continue;
			}

			for dvx in -2..2 {
				for dvy in -2..2 {
					let cst = s.cst + 1;
					let pv = s.pv.apply_gravity().accelerate_and_move(dvx, dvy);

					if !pv.is_in_valid_area() || self.get(&pv).0 <= cst {
						continue;
					}

					self.set(&pv, (cst, s.pv));
					que.push(BinaryHeapState { cst, pv });
				}
			}
		}

		// 復元するよー
		let mut posvels = vec![];
		let mut last_posvel = best_entry.2;
		while !last_posvel.is_empty() {
			posvels.push(last_posvel);
			last_posvel = self.get(&last_posvel).1;
		}
		posvels.reverse();
		// dbg!(&posvels);

		let dvx;
		let dvy;
		if posvels.len() < 2 {
			dvx = 0;
			dvy = 0;
		} else {
			dvx = posvels[1].vx - posvels[0].vx;
			dvy = posvels[1].vy - posvels[0].vy;
		}
		let (gx, gy) = posvels[0].get_gravity();

		((dvx - gx, dvy - gy), best_entry.1)
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl From<&Ship> for PosVel {
	fn from(s: &Ship) -> Self {
		Self {
			x: s.pos.0,
			y: s.pos.1,
			vx: s.v.0,
			vy: s.v.1,
		}
	}
}

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
	let mut router = Router::new();
	/*
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
		let my_ship = get_ship(&resp, my_id);
		let en_ship = get_ship(&resp, en_id);

		println!("TICK = {}, DISTANCE {}", resp.state.tick, (PosVel::from(my_ship).hypot_to(en_ship.pos.0, en_ship.pos.1) as f64).sqrt());

		// dbg!(my_ship);
		// dbg!(en_ship);

		let tx = clip_pos(en_ship.pos.0);
		let ty = clip_pos(en_ship.pos.1);
		let (_, n_steps) = router.get_next_move(my_ship.pos.0, my_ship.pos.1, my_ship.v.0, my_ship.v.1, tx, ty);

		let mut tpv = PosVel::from(en_ship);
		for _ in 0..n_steps {
			tpv = tpv.apply_gravity().accelerate_and_move(0, 0);
		}
		let ((dvx, dvy), _) = router.get_next_move(my_ship.pos.0, my_ship.pos.1, my_ship.v.0, my_ship.v.1, tpv.x, tpv.y);
		let mut commands = vec![Command::Accelerate(my_id, (-dvx, -dvy))];

		// 次ステップのポジで重なるなら爆発！
		let myp = PosVel::from(my_ship).apply_gravity().accelerate_and_move(dvx, dvy);
		let enp = PosVel::from(en_ship).apply_gravity().accelerate_and_move(0, 0);
		if myp.x == enp.x && myp.y == enp.y {
			eprintln!("{}", "BOMB!!!!!!!!".repeat(10));
			commands.push(Command::Detonate(my_id));
		}

		// dbg!((dvx, dvy));
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
