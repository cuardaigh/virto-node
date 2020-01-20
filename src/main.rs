#![warn(missing_docs)]
#![warn(unused_extern_crates)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;

pub use sc_cli::{error, IntoExit, VersionInfo};

fn main() -> Result<(), cli::error::Error> {
    let version = VersionInfo {
        name: "POC Node",
        commit: env!("VERGEN_SHA_SHORT"),
        version: env!("CARGO_PKG_VERSION"),
        executable_name: "node",
        author: "olanod",
        description: "Valibre POC Node",
        support_url: "https://github.com/valibre-org/substrate-poc/issues",
    };

    cli::run(std::env::args(), cli::Exit, version)
}