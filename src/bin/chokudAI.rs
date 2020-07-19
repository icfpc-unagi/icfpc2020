use app::client::*;
use rand::prelude::*;

fn run(){

	//CREATE
	let server_url = std::env::args().nth(1).unwrap();
	eprintln!("{}", server_url);
	let mut client = Client::new(server_url);
	if std::env::args().len() == 2 {
		client.send("[1, 0]");
		return;
	}

	//JOIN
	let player_key = std::env::args().nth(2).unwrap();
	let join_resp = client.join(&player_key);

	//START
	//set firstなんとか
	
	let mut all = 448;
	if join_resp.info.role == 0 { all = 512; }
	let shoot = 64;
	let heal = 10;
	let life = 1;
	let energy = all - shoot * 4 - heal * 12 - life * 2;
	
	let mut resp = client.start(energy, shoot, heal, life);

	let id = resp.state.ships.iter().find_map(|s| if s.role == resp.info.role { Some(s.id) } else { None }).unwrap();
	let my_role = join_resp.info.role;
	//COMMANDS
	while resp.stage != 2 {

		resp = client.command(&chokud_ai(&resp, &id, &my_role));
		//dbg!(&resp);
	}



}

fn chokud_ai(resp: &Response, id: &i32, my_role: &i32) -> Vec<Command> {
	
	let mut myship = resp.state.ships[0].clone();
	let mut enemyship = resp.state.ships[0].clone();

	for i in 0..resp.state.ships.len() {
		let nowship = resp.state.ships[i].clone();
		if nowship.role == *my_role {myship = nowship; }
		else {enemyship = nowship;}
	}


	let mut next_enemy = vec![enemyship.pos.0 + enemyship.v.0, enemyship.pos.1 + enemyship.v.1];

	
	if resp.state.tick < 30 || rand::thread_rng().gen_range(0, 2) == 0 {

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

	let mut shoot_flag = false;
	let mut accelerate_flag = false;
	let mut shooty = next_enemy[0];
	let mut shootx = next_enemy[1];

	if addy != 0 || addx != 0 {accelerate_flag = true; }
	if myship.heat <= myship.max_heat - 60 {shoot_flag = true;}

	eprintln!("debug {} {} {} {} {} {} {}", myship.status.energy, myship.pos.0, myship.pos.1, myship.v.0, myship.v.1, addx, addy);


	let mut ret = vec![];

	if shoot_flag{
		ret.push(Command::Shoot(*id, (shooty, shootx), 64));
	}

	if accelerate_flag {
		ret.push(Command::Accelerate(*id, (-addx, -addy)));
	}
	return ret;
}

fn main() {
	let _ = ::std::thread::Builder::new()
		.name("run".to_string())
		.stack_size(32 * 1024 * 1024)
		.spawn(run)
		.unwrap()
		.join();
}