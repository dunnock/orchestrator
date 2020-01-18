use ipc_orchestrator::orchestrator;
use tokio::process::Command;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_log_engine();

    let mut orchestrator = orchestrator().ipc(true);

    // Start pipeline: generate random f64 [0;1) -> sum -> write to stdout every 10_000 times
    let mut cmd = Command::new("cargo");
    orchestrator
        .start("generate", cmd.arg("run").arg("--example=generate"))
        .expect("failed to start generate");
    let mut cmd = Command::new("cargo");
    orchestrator
        .start("sum", cmd.arg("run").arg("--example=sum"))
        .expect("failed to start sum");
    let mut cmd = Command::new("cargo");
    orchestrator
        .start("write", cmd.arg("run").arg("--example=write"))
        .expect("failed to start write");

    // Connect log handlers and IPC handlers
    let mut orchestra = match orchestrator.connect().await {
        Err(_) => std::process::exit(1),
        Ok(o) => o,
    };

    // Route IPC messages
    orchestra.pipe_bridges("generate", "sum")?;
    orchestra.pipe_bridges("sum", "write")?;

    // Killing it hard since some spawned futures might still run
    match orchestra.run().await {
        Err(_) => std::process::exit(1),
        _ => Ok(()),
    }
}

fn init_log_engine() {
    let mut builder = pretty_env_logger::formatted_timed_builder();
    builder
        .filter_level(log::LevelFilter::Info)
        .default_format_module_path(true);
    builder.init();
}
