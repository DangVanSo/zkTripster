use clap::{Args, Parser};

use ethers::prelude::*;

use strum::EnumString;
use url::Url;
use zkpoex::zkPoExArgs;

#[derive(Clone, Parser)]
pub struct Options {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, clap::Subcommand)]
pub enum Command {
    #[command(about = "Setup wallet")]
    Setup(SetupArgs),
    #[command(about = "Deploy prover server")]
    Prover(ProverArgs),
}

#[derive(Clone, Args)]
pub struct SetupArgs {
    #[clap(
        short,
        long,
        default_value = "./keys",
        help = "path to keystore location"
    )]
    pub keystore_dir: String,
}

#[derive(Clone, Args)]
pub struct ProverArgs {
    #[clap(
        short,
        long,
        default_value = "./keys",
        help = "path to keystore location"
    )]
    pub keystore_dir: String,

    #[clap(short, long, help = "wallet name")]
    pub wallet_name: Option<String>,

    #[clap(short, long, help = "wallet password")]
    pub password: Option<String>,

    #[clap(short, long, default_value = "local", help = "Ethereum network")]
    pub network: Network,

    #[clap(index = 1, help = "platform address", default_value = "0x7C3c3cEFAde338Bb4461d365ed5B1955A944F2cD")]
    pub platform_contract: String,

    #[clap(index = 2, help = "ask bounty amount (in ETH)")]
    pub ask_bounty: f64,

    #[clap(
        short = 'a',
        default_value = "127.0.0.1:8000",
        long,
        help = "server address"
    )]
    pub server_address: String,

    #[clap(flatten)]
    pub args: zkPoExArgs,
}

#[derive(Clone, Debug, PartialEq, EnumString)]
pub enum Network {
    #[strum(serialize = "mainnet")]
    Mainnet,
    #[strum(serialize = "sepolia")]
    Sepolia,
    #[strum(serialize = "local")]
    Local,
}

impl Network {
    pub fn get_endpoint(&self) -> Url {
        match self {
            Network::Mainnet => {
                Url::parse("https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27").unwrap()
            }
            Network::Sepolia => {
                Url::parse("https://goerli.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27").unwrap()
            }
            Network::Local => Url::parse("http://localhost:8545").unwrap(),
        }
    }

    pub fn get_chainid(&self) -> String {
        match self {
            Network::Mainnet => "1".to_string(),
            Network::Sepolia => "11155111".to_string(),
            Network::Local => "31337".to_string(),
        }
    }
}
