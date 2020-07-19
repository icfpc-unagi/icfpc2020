use std::env;
use tokio::runtime::Runtime;
use reqwest;

async fn send_async(s: String) -> String {
	let client = reqwest::Client::new();
	let res = client.post(&format!(
		"{}{}",
		"https://icfpc2020-api.testkontur.ru/aliens/send?apiKey=",
		env::var("ICFPC_API_KEY").expect("ICFPC_API_KEY must be specified")
	))
		.body(s.clone())
		.send()
		.await;
	let resp = res.unwrap();
	if resp.status() != reqwest::StatusCode::OK {
		panic!("Unexpected respose: {:?}", resp);
	}
	resp.text().await.unwrap()
}

#[allow(dead_code)]
pub fn send(s: &str) -> String {
	let s = String::from(s);
	Runtime::new()
		.expect("Failed to create tokio runtime")
		.block_on(send_async(s))
}

#[test]
#[ignore]
fn test_send() {
	assert_eq!(send("010"), "1101000");
}
