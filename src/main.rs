use reqwest::{self};
use std::error::Error;
use types::VnStats;

mod types;

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
