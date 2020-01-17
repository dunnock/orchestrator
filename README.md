Execute and orchestrate command line utils.

Based on io streams and optionally ipc-channel orchestrator is intended for starting and orchestrating programs.

This repo cannot be used for running production critical tasks, rather it is for local dev or non critical orchestration.

Working on linux and Mac OS X, Windows is not supported due to dependency on ipc_channels.

# Use case
```
use tokio::process::{Command};
use ipc_orchestrator::orchestrator;

tokio::runtime::Runtime::new().unwrap().block_on(async {
	let mut orchestrator = orchestrator().ipc(false);
    orchestrator.start("start", &mut Command::new("echo"));
    orchestrator.connect().await
});
```