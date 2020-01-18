use ipc_orchestrator::connect_ipc_server;
use ipc_orchestrator::message::Message;
use rand::Rng;
use std::time::Instant;

fn main() {
    let channel = connect_ipc_server().expect("failed to connect to server");
    let (tx, _rx) = channel.split().expect("failed to split channel");

    let start = Instant::now();
    let mut rng = rand::thread_rng();
    const TOTAL: usize = 1_000_000;

    for _ in 0..TOTAL {
        let num = rng.gen::<f64>();
        tx.send(Message {
            topic: "generate".to_string(),
            data: num.to_le_bytes().to_vec(),
        })
        .expect("failed to send message");
    }

    let ms = start.elapsed().as_millis();
    println!(
        "sent {} numbers in {}ms rate {:.0}rps",
        TOTAL,
        ms,
        TOTAL as f64 / ms as f64 * 1000.0
    );
}
