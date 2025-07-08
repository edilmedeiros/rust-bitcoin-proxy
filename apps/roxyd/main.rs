mod cli;
use clap::Parser;
use cli::{Action, Args};
use roxy::error::*;
mod actions;
mod rpc;
use crate::actions::proxy;
use crate::actions::test;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Args::parse();
    match cli.action {
        Action::Proxy => proxy::run(&cli).await,
        Action::Test => test::run(&cli).await.map(|_| ()),
    }
}
