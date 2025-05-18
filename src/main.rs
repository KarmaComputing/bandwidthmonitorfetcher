use reqwest::{self};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct VnStats {
    vnstatversion: String,
    jsonversion: String,
    interfaces: Vec<VnStatInterface>,
}

#[derive(Deserialize, Debug)]
struct VnStatInterface {
    name: String,
    alias: String,
}

fn main() {
    run().unwrap()
}

fn run() -> Result<(), Box<dyn Error>> {
    let req: VnStats = reqwest::blocking::get("http://127.0.0.1:8685/json.cgi")?.json()?;

    println!("{req:?}");
    println!("The vnstatversion is: {}", req.vnstatversion);
    println!("The jsonversion is: {}", req.jsonversion);
    println!(
        "An interface {:?}, name: {}, alias: {}",
        req.interfaces[0], req.interfaces[0].name, req.interfaces[0].alias
    );

    Ok(())
}

/*
    // The question operator is a shorthand for the match Ok/Err verbose approach (which is useful) but
    // not always needed (too noisy!)

    let contents = match req {
        Ok(jsonData) => jsonData,
        Err(e) => return Err(e) ,
    }

*/
