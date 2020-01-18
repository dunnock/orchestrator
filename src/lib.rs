#![allow(clippy::unnecessary_mut_passed)]
//! Opinionated orchestrator for services which communicate via IPC and are not expected to exit
//! It allows to start and control processes, handling all the necessary boilerplate:
//! - Running within async runtime
//! - Uses tokio::process::Command with predefined params
//!   to execute commands
//! - Uses log with info+ levels to
//! - Uses ipc-channel to establish communication from and to processes
//! ```
//! use tokio::process::{Command};
//! use ipc_orchestrator::orchestrator;
//! // from within async runtime:
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//!     let mut orchestrator = orchestrator().ipc(false);
//!     orchestrator.start("start", &mut Command::new("echo"));
//!     orchestrator.connect().await
//! # });
//! ```

mod channel;
mod connected;
mod logger;
mod macros;
pub mod message;
mod orchestrator;

pub use ipc_channel::ipc::{IpcReceiver, IpcSender};
use tokio::process::Child;

pub use orchestrator::{orchestrator, Orchestrator};

/// Channel for duplex communication via IPC
pub type Channel = channel::Channel<message::Message>;
/// IPC Sender for Message
pub type Sender = IpcSender<message::Message>;
/// IPC Receiver for Message
pub type Receiver = IpcReceiver<message::Message>;

pub struct Process {
    name: String,
    child: Child,
}
impl std::fmt::Debug for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Process {{ {} }}", self.name)
    }
}

/// Communication channel for module `name`
#[derive(Debug)]
pub struct Bridge {
    pub channel: Channel,
    pub name: String,
}

pub const IPC_SERVER_ENV_VAR: &'static str = "IPC_SERVER";

/// This is helper function for implementing child processes
/// Child process will automatically connect to the IPC server
/// passed in the env var "IPC_SERVER".
/// This env var is injected by orchestrator.
/// Execution blocks until connected
///
/// TODO: move to separate client library or set features to exclude all the other unnecessary code
pub fn connect_ipc_server() -> anyhow::Result<Channel> {
    let ipc_output = std::env::var(IPC_SERVER_ENV_VAR)?;
    println!("Connecting to server: {}", ipc_output);
    let tx = IpcSender::connect(ipc_output.clone())?;
    let (ch1, ch2) = Channel::duplex()?;
    println!("Connected, sending Channel to server: {}", ipc_output);
    tx.send(ch1)?;
    Ok(ch2)
}
