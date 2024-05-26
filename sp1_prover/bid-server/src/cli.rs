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
    let (wallet, local_sk) = read_from_keystore(keystore, password)?;
    info!("Our address: {}", wallet.address());
    // let chain_id = {
    //     let provider = Provider::new(Http::new(args.network.get_endpoint()));

    //     provider
    //         .get_chainid()
    //         .await
    //         .map_err(|_e| anyhow!("error making request to the specified Ethereum RPC address"))?
    // };

    let chain_id: u64 = args.network.get_chainid().parse().unwrap();
    let client = Arc::new(
        Provider::new(Http::new(args.network.get_endpoint()))
            .with_signer(wallet.with_chain_id(chain_id)),
    );

    let block = client.clone().get_block_number().await?;

    let platform_contract = Address::from_slice(&hex::decode(args.platform_contract).unwrap());
    // .map_err(|e| anyhow!("error parsing target address: {e}"))?;

    // let (fixture, proof_bytes) = zkpoex::prove(args.args)?;

    let bounty_eth = args.ask_bounty;

    let bounty_gwei =
        parse_ether(args.ask_bounty).map_err(|e| anyhow!("error parsing ether: {e}"))?;
    let contract = ProofOfExploitMarketplace::new(platform_contract, client.clone());

    let proof_bytes = vec![0u8; 32];
    let key_hash = [0u8; 32];

    let vuln_id: U256 = {
        let vuln_id = contract
            .post_exploit(
                "got 'em".to_string(),
                bounty_gwei,
                key_hash, //hex::decode(&fixture.key_hash).unwrap().try_into().unwrap(),
            )
            .await?;
        let tx = contract.post_exploit(
            "got 'em".to_string(),
            bounty_gwei,
            key_hash, //hex::decode(fixture.key_hash).unwrap().try_into().unwrap(),
        );

        let pending_tx = tx.send().await?;
        let _mined_tx = pending_tx.await?;

        vuln_id
    };

    println!("vuln id: {}", vuln_id);

    tokio::spawn(async move {
        server::rocket(proof_bytes, bounty_eth, vuln_id.to_string(), args.local_pk.clone())
            .launch()
            .await
            .expect("expect server to run");
    });

    // for testing
    {
        let c = contract.clone();
        tokio::spawn(async move {
            let tx = c.purchase_token(vuln_id).value(bounty_gwei);

            let pending_tx = tx.send().await.unwrap();
            let _mined_tx = pending_tx.await.unwrap();
            println!("Token purchased");
        });
    }

    let events = contract
        .event::<TokenPurchasedFilter>()
        .from_block(block - 1);
    let mut stream = events.stream().await?;

    //     let erc20_transfer_filter =
    //     Filter::new().from_block(0).event("TokenPurchased(uint256, address)");

    // let mut stream = client.subscribe_logs(&erc20_transfer_filter).await?.take(1);

    // let erc20_transfer_filter = Filter::new()
    //     .from_block(block - 25)
    //     .event("Transfer(address,address,uint256)");

    // let mut stream = client.subscribe_logs(&erc20_transfer_filter).await?.take(2);

    // wasn't able to get buyer pk from event so need to pass it into CL
    let vendor_pk_bytes = hex::decode(args.vendor_pk).unwrap();

    println!("Waiting for exploit redemption");
    while let Some(e) = stream.next().await {
        println!("RedeemedFilter event: {e:?}");

        let (_, proof, public_inputs) = zkecdh::prove(
            &local_sk,
            &vendor_pk_bytes,
            Some(Path::new(&format!(
                "./data/vuln_keyenc.bincode"
            ))),
        )?;

        let tx = contract.redeem_exploit(vuln_id, proof.into(), public_inputs.into());

        let pending_tx = tx.send().await.unwrap();
        let _mined_tx = pending_tx.await.unwrap();
    }

    Ok(())
}
