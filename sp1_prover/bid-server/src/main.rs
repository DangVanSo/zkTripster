#[macro_use] extern crate rocket;

use rocket::{Rocket, Build, State};
use rocket::http::Method;

use rocket::serde::Serialize;

use rocket::serde::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Guard, Responder};
use clap::Parser;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Resp {
    proof: String,
    address: String,
}

// return contets of proof
#[get("/proof")]
async fn proof(cors: Guard<'_>, args: &State<Args>) -> Responder<Json<Resp>> {
    let resp = Resp {
        proof: args.proof.clone(),
        address: args.address.clone(),
    };
    cors.responder(Json(resp))
}

/// Simple program to serve proof files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// path to proof
    #[arg(short, long)]
    proof: String,

    /// redemption address
    #[arg(short, long, default_value_t = (&"0x..").to_string())]
    address: String,
}

#[launch]
fn rocket() -> Rocket<Build> {

    let allowed_origins = AllowedOrigins::some_exact(&["https://zktripster.pages.dev"]);
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().unwrap();

    let args = Args::parse();
    rocket::build()
        .mount("/", routes![proof])
        .mount("/", rocket_cors::catch_all_options_routes())
        .manage(args)
        .manage(cors)
}
