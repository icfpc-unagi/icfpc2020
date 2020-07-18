use http_body::Body as _;
use hyper::{Body, Client, Method, Request, StatusCode};
use hyper_tls::HttpsConnector;
use std::env;
use tokio::runtime::Runtime;

async fn send_async(s: String) -> String {
	let https = HttpsConnector::new();
	let client = Client::builder().build::<_, hyper::Body>(https);
	let req = Request::builder()
		.method(Method::POST)
		.uri(format!(
			"{}{}",
			"https://icfpc2020-api.testkontur.ru/aliens/send?apiKey=",
			env::var("ICFPC_API_KEY").expect("ICFPC_API_KEY must be specified")
		))
		.body(Body::from(s))
		.unwrap();
	match client.request(req).await {
		Ok(mut res) => {
			let mut cs = vec![];
			while let Some(chunk) = res.body_mut().data().await {
				match chunk {
					Ok(c) => {
						cs.append(&mut c.to_vec());
					}
					Err(e) => panic!("{}", e),
				}
			}
			let s = String::from_utf8(cs).unwrap();
			match res.status() {
				StatusCode::OK => return s,
				_ => panic!("HTTP code: {}\nBody: {}", res.status(), s),
			}
		}
		Err(err) => panic!("Unexpected server response:\n{}", err),
	}
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
