use crate::client::*;
use rand::prelude::*;
use crate::wata::*;

struct EnemyData{
	pub pattern: Vec<Vec<i32>>,
	pub go_near: bool, 

}

pub fn run_chokudai() {

	// //CREATE
	// let server_url = std::env::args().nth(1).unwrap();
	// eprintln!("{}", server_url);
	// let mut client = Client::new(server_url);
	// if std::env::args().len() == 2 {
	// 	client.send("[1, 0]");
	// 	return;
	// }

	// //JOIN
	// let player_key = std::env::args().nth(2).unwrap();
	// let join_resp = client.join(&player_key);

	// run(client, join_resp);
}

pub fn run(client: Client, join_resp: Response, prep: Preprocess){


	let mut router = super::routing::Router::new();

	let all = join_resp.info.ability.potential;
	let shoot = 64;
	let mut heal = 10;
	let life = 1;
	if join_resp.info.role == 0 { heal = 16;}
	let energy = all - shoot * 4 - heal * 12 - life * 2;
	
	let mut resp = client.start(energy, shoot, heal, life);

	let mut e_data = EnemyData{
		pattern: vec![vec![0;5];5],
		go_near: true 
	};

	let id = resp.state.ships.iter().find_map(|s| if s.role == resp.info.role { Some(s.id) } else { None }).unwrap();
	let my_role = join_resp.info.role;


	//COMMANDS
	while resp.stage != 2 {

		resp = client.command(&chokud_ai(&resp, &id, &my_role, &mut e_data, &prep, &mut router));
		//dbg!(&resp);
	}
}


fn dist(a: &Ship, b: &Ship) -> i32{
	return (a.pos.0 - b.pos.0).abs() + (a.pos.1 - b.pos.1).abs(); 
}

fn chokud_ai(resp: &Response, id: &i32, my_role: &i32, e_data: &mut EnemyData, prep: &Preprocess, router: &mut super::routing::Router) -> Vec<Command> {
	
	let mut myship = resp.state.ships[0].clone();
	let mut enemyship = resp.state.ships[0].clone();
	let mut enemyshipflag = false;

	let mut px = 0;
	let mut py = 0;

	for i in 0..resp.state.ships.len() {
		let nowship = resp.state.ships[i].clone();
		if nowship.role == *my_role {myship = nowship; }
	}

	let mut enemy_cnt = 0;

	for i in 0..resp.state.ships.len() {
		let nowship = resp.state.ships[i].clone();
		if nowship.role != *my_role {
			enemy_cnt += 1;
			if !enemyshipflag || enemyship.status.energy < nowship.status.energy
			|| (enemyship.status.energy == nowship.status.energy  
				&& dist(&enemyship, &myship) > dist(&nowship, &myship)) {
				enemyship = nowship;
				enemyshipflag = true;
				for c in enemyship.commands.iter(){
					if let Command::Accelerate(_, v) = c {
						px = -v.0;
						py = -v.1;
						let ppx = 2 + px;
						let ppy = 2 + py;
						e_data.pattern[ppx as usize][ppy as usize] += 1;
					}
				}
			}
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
	
	//next_enemy[0] += px;
	//next_enemy[1] += py;	

	let mut enemy_move_x = 0 as i32;
	let mut enemy_move_y = 0 as i32;
	let mut enemyCnt = 0 as i32;

	for x in 0..5 {
		for y in 0..5 {
			if e_data.pattern[x][y] > enemyCnt {
				enemyCnt = e_data.pattern[x][y];
				enemy_move_x = x as i32 - 2;
				enemy_move_y = y as i32 - 2;
			}
		}
	}	

	next_enemy[0] += px;
	next_enemy[1] += py;

	//next_enemy[0] += enemy_move_x;
	//next_enemy[1] += enemy_move_y;

	let mut addx = 0;
	let mut addy = 0;

	if myship.heat <= 20 && myship.status.energy >= 20{
		e_data.go_near = true;
	}

	if myship.heat >= myship.max_heat - 50 || myship.status.energy <= 20 || (enemy_cnt >= 10 && enemyship.status.energy < 5){
		e_data.go_near = false;
	}

	if e_data.go_near {
		eprintln!("Go Near!");
		let (dvx, dvy) = router.doit(&myship, &enemyship);
		addx = dvx;
		addy = dvy;
	}
	else {
		eprintln!("Loop!");
		let r = to_orbit(myship.pos.0, myship.pos.1, myship.v.0, myship.v.1, 200, prep);
		addx = r.0;
		addy = r.1;
	}

	let mut shoot_flag = false;
	let mut accelerate_flag = false;
	let shooty = next_enemy[0];
	let shootx = next_enemy[1];

	let next_me = vec![myship.pos.0+myship.v.0+addx, myship.pos.1+myship.v.1+addy];


	if myship.heat <= myship.max_heat - 45 {shoot_flag = true;}

	let maxlen = (next_me[0]-next_enemy[0]).abs().max( (next_me[1]-next_enemy[1]).abs());
	let minlen = (next_me[0]-next_enemy[0]).abs().min( (next_me[1]-next_enemy[1]).abs());

	let terrible_angle = (maxlen * 2 / 10 <= minlen) && (maxlen * 8 / 10 >= minlen);
	let bad_angle = (maxlen * 1 / 10 <= minlen) && (maxlen * 9 / 10 >= minlen);

	if !bad_angle || (!terrible_angle && minlen + maxlen <= 35) {
		if addx == 0 && addy == 0 {
			if minlen + maxlen <= 70 && enemyship.max_heat - enemyship.heat >= 30 && enemyship.status.power >= 30 && myship.status.energy >= 10 {
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
		ret.push(Command::Shoot(*id, (shooty, shootx), 64, None));
	}

	if accelerate_flag {
		ret.push(Command::Accelerate(*id, (-addx, -addy)));
	}
	return ret;
}
