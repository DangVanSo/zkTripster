#[macro_use] extern crate rocket;

use rocket::{Rocket, Build, State};
use clap::Parser;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// return contets of proof
#[get("/proof")]
async fn proof(args: &State<Args>) -> String {
    args.proof.clone()
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
    let args = Args::parse();
    rocket::build()
        // .mount("/", routes![hello, hello]) // uncomment this to get an error
        // .mount("/", routes![unmanaged]) // uncomment this to get a sentinel error
        .mount("/", routes![index])
        .mount("/", routes![proof])
        .manage(args)
}


