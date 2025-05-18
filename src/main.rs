use std::{error::Error, f32::consts::E};

use reqwest::{self};


fn main() {
    run().unwrap()
}

fn run() -> Result<(), Box<dyn Error>> {

    let req = reqwest::blocking::get("http://127.0.0.1:8685/json.cgi")?
    .text()?;
    
    println!("{req}");

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


