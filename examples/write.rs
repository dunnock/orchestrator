use ipc_orchestrator::connect_ipc_server;
use std::convert::TryFrom;
use std::time::Instant;

fn main() -> anyhow::Result<()> {
	let channel = connect_ipc_server()
		.expect("failed to connect to server");
	let (_tx, rx) = channel.split()
		.expect("failed to split channel");
	
	let start = Instant::now();

	let mut i = 0;
	while let Ok(msg) = rx.recv() {
		let sum = f64::from_le_bytes( <[u8;8]>::try_from(&msg.data[..])? );

		if i % 10_000 == 0 {
			println!("{}", sum)
		};
		i += 1;
	};

	let ms = start.elapsed().as_millis();
	println!("final write in {}ms from start", ms);
	Ok(())	
}
