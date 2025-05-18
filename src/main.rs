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
    created: VnStatDateCreated,
}

#[derive(Deserialize, Debug)]
struct VnStatDateCreated {
    date: VnStatDate,
    timestamp: u32,
}

#[derive(Deserialize, Debug)]
struct VnStatDate {
    day: u32,
    month: u32,
    year: u32,
}

fn main() {
    run().unwrap()
}

fn run() -> Result<(), Box<dyn Error>> {
    let req: VnStats = reqwest::blocking::get("http://127.0.0.1:8685/json.cgi")?.json()?;

    println!("{req:?}");
    println!("The vnstatversion is: {}", req.vnstatversion);
    println!("The jsonversion is: {}", req.jsonversion);
    let num_interfaces = req.interfaces.len();
    println!("The number of interfaces is: {}", num_interfaces);
    println!("{req:#?}");
    /*
    println!(
        "An interface {:?}\n, name: {}\n, alias: {}\n. Created: {}/{}/{}\n Timestamp: {} \n. ",
        req.interfaces[0],
        req.interfaces[0].name,
        req.interfaces[0].alias,
        req.interfaces[0].created.date.day,
        req.interfaces[0].created.date.month,
        req.interfaces[0].created.date.year,
        req.interfaces[0].created.timestamp
    );
    */

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
