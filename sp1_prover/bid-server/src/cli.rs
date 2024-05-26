#[macro_use]
extern crate rocket;

use std::future::Future;
use std::path::Path;
use std::sync::Arc;

use ethers::prelude::*;
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::utils::{format_units, parse_ether};

use inquire::{Password, Select, Text};
use serde::Deserialize;
use serde_json::json;

use args::{Options, ProverArgs, SetupArgs};
use clap::Parser;
mod args;
mod server;
mod utils;
use anyhow::*;

use utils::*;

abigen!(
    ProofOfExploitMarketplace,
    "../../frontend/contracts/lib/PoEMarketplace_abi.json",
);

async fn app(options: Options) -> anyhow::Result<()> {
    match options.command {
        args::Command::Setup(args) => setup(args).await,
        args::Command::Prover(args) => prover(args).await,
    }
}

#[tokio::main]
async fn main() {
    let args = Options::parse();
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
        0 => keypair_gen()?,
        1 => keypair_from_hex(&Text::new("Paste hex here:").prompt().unwrap())?,
        // 2 => keypair_from_bip39(&Text::new("Mnemonic phrase:").prompt().unwrap())?.0,
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

    let client =
        Arc::new(Provider::new(Http::new(args.network.get_endpoint())).with_signer(wallet));

    let platform_contract = Address::from_slice(&hex::decode(args.platform_contract).unwrap());
    // .map_err(|e| anyhow!("error parsing target address: {e}"))?;

    let (fixture, proof_bytes) = zkpoex::prove(args.args)?;

    let bounty_eth = args.ask_bounty;

    let bounty_gwei =
        parse_ether(args.ask_bounty).map_err(|e| anyhow!("error parsing ether: {e}"))?;
    let contract = ProofOfExploitMarketplace::new(platform_contract, client);

    // let proof_bytes = vec![0u8; 32];
    // let key_hash = [0u8; 32];

    contract
        .post_exploit(
            "got 'em".to_string(),
            bounty_gwei,
            hex::decode(fixture.key_hash).unwrap().try_into().unwrap(),
        )
        .await?;

    tokio::spawn(async move {
        server::rocket(proof_bytes, bounty_eth)
            .launch()
            .await
            .expect("expect server to run");
    });

    let event = contract.events::<ExploitRedeemed>().unwrap();


    //  stream.

    Ok(())
}
