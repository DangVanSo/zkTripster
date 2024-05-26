#[macro_use] extern crate rocket;

use rocket::{Rocket, Build};

struct Runtime {
    tx: mpsc::Sender<VendorRequest>,
}

pub enum VendorRequest {
    Swap {
        vendor_bid: f64,
        resp_tx: oneshot::Sender<anyhow::Result<SwapMsg>>,
    },
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[derive(Parser)]
#[group(skip)]
struct Options {
    /// File to read
    #[clap(long, env, default_value = "")]
    proofPath: PathBuf,
}


async fn app(options: Options) -> Result<()> {
    let mut proof = File::open(options.proofPath).await?;
    // verify
    Ok(())
}

#[allow(unused_must_use)]
pub async fn serve(to_runtime: mpsc::Sender<maker::MakerRequest>, addr: String) {
    let addr: SocketAddr = addr.parse().expect("valid address");
    let mut config = rocket::Config::default();
    config.address = addr.ip();
    config.port = addr.port();
    config.shutdown.ctrlc = true;
    config.shutdown.force = true;

    rocket::build()
        .manage(Runtime { tx: to_runtime })
        .mount("/", routes![setup, lock, swap])
        .launch()
        .await
        .expect("expect server to run");
}
