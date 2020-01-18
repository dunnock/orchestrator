Execute and orchestrate command line utils.

Based on io streams and optionally ipc-channel orchestrator is intended for starting and orchestrating programs.

This repo cannot be used for running production critical tasks, rather it is for local dev or non critical orchestration.

Working on linux and Mac OS X, Windows is not supported due to dependency on ipc_channels.


# Basic example

```rust
use tokio::process::{Command};
use ipc_orchestrator::orchestrator;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut orchestrator = orchestrator().ipc(false);
    orchestrator.start("start", &mut Command::new("echo"));
    orchestrator.connect().await
}
```

# IPC routing example

```shell
cargo run --example=orchestrate
```

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut orchestrator = orchestrator().ipc(true);

    // Start pipeline: generate random f64 [0;1) -> sum -> write to stdout every 10_000 times
    let mut cmd = Command::new("cargo");
    orchestrator.start("generate", cmd.arg("run").arg("--example=generate"))
        .expect("failed to start generate");
    let mut cmd = Command::new("cargo");
    orchestrator.start("sum", cmd.arg("run").arg("--example=sum"))
        .expect("failed to start sum");
    let mut cmd = Command::new("cargo");
    orchestrator.start("mul", cmd.arg("run").arg("--example=write"))
        .expect("failed to start mul");

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
```


# Custom logger

```rust
use tokio::process::{Command, ChildStdout};
use ipc_orchestrator::Orchestrator;
use std::sync::atomic::{AtomicBool, Ordering};
static CALLED: AtomicBool = AtomicBool::new(false);
use tokio::io::{AsyncBufReadExt, BufReader};

// custom logs processor
async fn mock_log_handler(reader: ChildStdout, name: String) -> anyhow::Result<()> {
   let mut reader = BufReader::new(reader).lines();
   assert_eq!(reader.next_line().await?.unwrap(), "testbed");
   CALLED.store(true, Ordering::Relaxed);
   Ok(())
}

#[tokio::main]
async fn main() {
    let mut orchestrator = Orchestrator::from_handlers(mock_log_handler).ipc(false);
    let mut cmd = Command::new("echo");
    cmd.arg("testbed");
    orchestrator.start("start", &mut cmd);
    let orchestra = orchestrator.connect().await.unwrap();
    // it supposes never existing processes
    // hence it will give error on when any process exit or stdout was closed
    orchestra.run().await.unwrap_err();
    assert!(CALLED.load(Ordering::Relaxed));
}
```