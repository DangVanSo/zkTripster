use rocket::http::Method;
use rocket::{Build, Rocket, State};

use rocket::serde::Serialize;

use clap::Parser;
use rocket::serde::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Guard, Responder};

struct Runtime {
    // tx: mpsc::Sender<VendorRequest>,
    proof: String,
    price: f64,
}

// pub enum VendorRequest {
//     Swap {
//         vendor_bid: f64,
//         resp_tx: oneshot::Sender<anyhow::Result<SwapMsg>>,
//     },
// }

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Resp {
    proof: String,
    // address: String,
}

// return contets of proof
#[get("/proof")]
async fn proof(cors: Guard<'_>, args: &State<Runtime>) -> Responder<Json<Resp>> {
    let resp = Resp {
        proof: args.proof.clone(),
        // address: args.address.clone(),
    };
    cors.responder(Json(resp))
}

pub fn rocket(proof_bytes: Vec<u8>, price: f64) -> Rocket<Build> {
    let allowed_origins = AllowedOrigins::some_exact(&["https://zktripster.pages.dev"]);
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    // let addr: SocketAddr = addr.parse().expect("valid address");
    // let mut config = rocket::Config::default();
    // config.address = addr.ip();
    // config.port = addr.port();
    // config.shutdown.ctrlc = true;
    // config.shutdown.force = true;

    rocket::build()
        .mount("/", routes![proof])
        .mount("/", rocket_cors::catch_all_options_routes())
        .manage(Runtime { proof: hex::encode(proof_bytes), price })
        .manage(cors)
}
