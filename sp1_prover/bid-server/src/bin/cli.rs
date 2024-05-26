#![feature(async_closure)]
#[macro_use]
extern crate rocket;

use std::future::Future;
use std::path::Path;

use ethers::prelude::*;
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::utils::{format_units, parse_ether};

use inquire::{Password, Select, Text};
use serde::Deserialize;
use serde_json::json;


use crate::{cli::spec_app, rpc::run_rpc};
use args::{Cli, Options, ProverArgs, SetupArgs};
use clap::Parser;
use utils::utils_cli;
mod args;

async fn app(options: Options) -> anyhow::Result<()> {
    match options.command {
        args::Command::Setup(_) => todo!(),
        args::Command::Prover(_) => todo!(),
    }
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    tracing_subscriber::fmt::init();
    app(args).await.unwrap();
}


async fn setup(args: SetupArgs) -> anyhow::Result<()> {
    let options = vec![
        "Generate new",
        "Recover from hex",
        "Recover from BIP39 mnemonic",
    ];
    let picked = Select::new("Wallet source?", options.clone())
        .prompt()
        .unwrap();
    let sk = match options
        .iter()
        .position(|e| *e == picked)
        .expect("unexpected option")
    {
        0 => keypair_gen().0,
        1 => keypair_from_hex(&Text::new("Paste hex here:").prompt().unwrap())?.0,
        2 => keypair_from_bip39(&Text::new("Mnemonic phrase:").prompt().unwrap())?.0,
        _ => panic!("unexpected option"),
    };

    let name = Text::new("Wallet name:").prompt().unwrap();
    let password = Password::new("Password:").prompt().unwrap();

    write_to_keystore(sk, args.keystore_dir, name, password)
}

async fn prover(args: ProverArgs) -> anyhow::Result<()> {
    let name = args
        .wallet_name
        .unwrap_or_else(|| Text::new("Wallet name:").prompt().unwrap());
    let password = args
        .password
        .unwrap_or_else(|| Password::new("Password:").prompt().unwrap());
    let keystore = Path::new(&args.keystore_dir).join(name);
    let wallet = read_from_keystore(keystore, password)?;
    info!("Our address: {}", wallet.address());

    let eth_provider = Ethereum::new(&args.network).await?;

    let target_address = Address::from_str(&args.secondary_address)
        .map_err(|e| anyhow!("error parsing target address: {e}"))?;

    let time_lock = match args.vtc.method {
        VTCMethod::HTLP => VariableVTC::new_htlp(SECURITY_BITS, args.vtc.htlp_hardness),
        VTCMethod::TLock => {
            VariableVTC::new_tlock(&args.vtc.drand_network, &args.vtc.chain_hash).await?
        }
    };

    let (bob, to_alice) = Maker::new(
        eth_provider.clone(),
        wallet,
        target_address,
        time_lock,
        args.vtc.refund_duration.into(),
    )
    .unwrap();

    tokio::spawn(async {
        bob.run().await;
    });

    server::serve(to_alice, args.server_address).await;

    Ok(())
}
