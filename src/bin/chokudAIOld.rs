
fn main() {
	let _ = ::std::thread::Builder::new()
		.name("run_chokudai".to_string())
		.stack_size(32 * 1024 * 1024)
		.spawn(app::chokudAIOld::run_chokudai)
		.unwrap()
		.join();
}