use reqwest::blocking as reqwest;
use parser::*;
use app::*;

struct Client {
	server_url: String,
	client: reqwest::Client
}

impl Client {
	pub fn new(server_url: String) -> Self {
		Self {
			server_url,
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
	pub fn join_request(&self, player_key: &str) {
		let resp = self.send(&format!("[2, {}, [192496425430, 103652820]]", player_key));
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
	client.join_request(&player_key);
}

fn main() {
	let _ = ::std::thread::Builder::new()
		.name("run".to_string())
		.stack_size(32 * 1024 * 1024)
		.spawn(run)
		.unwrap()
		.join();
}
