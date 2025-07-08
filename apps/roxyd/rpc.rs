use crate::cli::{Args, Network};
use roxy::error::*;
use std::process::exit;

pub fn read_cookie(datadir_path: &str) -> (String, String) {
    let mut full_path: String = "/regtest/.cookie".to_owned();
    full_path.insert_str(0, datadir_path);
    let (user, pass) = match std::fs::read_to_string(&full_path)
        .map_err(Error::from)
        .and_then(|s| {
            s.split_once(":")
                .map(|(s1, s2)| (s1.to_string(), s2.to_string()))
                .ok_or(Error::Err("Can't parse cookie file".to_string()))
        }) {
        Ok(t) => t,
        Err(Error::IO(e)) => {
            println!(
                "{}: {} (Are you sure you are running the bitcoin node?)",
                full_path, e
            );
            exit(-1);
        }
        Err(Error::Err(e)) => {
            println!("{}", e);
            exit(-2);
        }
        _ => {
            println! {"Unexpected error..."};
            exit(-3);
        }
    };
    (user, pass)
}

pub fn get_rpc_port(cli: &Args) -> u16 {
    match cli.rpc_port {
        None => match cli.network {
            Network::Mainnet => 8332,
            Network::Regtest => 18443,
            Network::Testnet => 38332,
            Network::Signet => 38332,
        },
        Some(port) => port,
    }
}
