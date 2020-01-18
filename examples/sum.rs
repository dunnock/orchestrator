use ipc_orchestrator::{message::Message};
use ipc_orchestrator::connect_ipc_server;
use std::convert::TryFrom;
use std::time::Instant;

fn main() -> anyhow::Result<()> {
	let channel = connect_ipc_server()
		.expect("failed to connect to server");
	let (tx, rx) = channel.split()
		.expect("failed to split channel");
	
	let start = Instant::now();

	let mut sum = 0.0;
	while let Ok(msg) = rx.recv() {
		let num = f64::from_le_bytes( <[u8;8]>::try_from(&msg.data[..])? );

		sum += num;
		tx.send(Message { topic: "sum".to_string(), data: sum.to_le_bytes().to_vec() })
			.expect("failed to send message");
	};

	let ms = start.elapsed().as_millis();
	println!("total sum {} in {}ms", sum, ms);

	Ok(())	
}
