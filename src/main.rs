use reqwest::{self};
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::process::exit;

#[derive(Deserialize, Debug)]
struct VnStats {
    vnstatversion: String,
    jsonversion: String,
    interfaces: Vec<VnStatInterface>,
}

#[derive(Deserialize, Debug, Clone)]
struct VnStatInterface {
    name: String,
    alias: String,
    created: VnStatDateCreated,
    updated: VnStatDateUpdated,
    traffic: VnStatTraffic,
}

#[derive(Deserialize, Debug, Clone)]
struct VnStatTraffic {
    total: VnStatTrafficTotal,
    fiveminute: Vec<VnStatTrafficFiveminute>,
}

#[derive(Deserialize, Debug, Clone)]
struct VnStatTrafficTotal {
    rx: u64,
    tx: u64,
}

#[derive(Deserialize, Debug, Clone)]
struct VnStatTrafficFiveminute {
    id: u32,
    date: VnStatDate,
    time: VnStatTime,
    timestamp: u32,
    rx: u64,
    tx: u64,
}

#[derive(Deserialize, Debug, Clone)]
struct VnStatDateCreated {
    date: VnStatDate,
    timestamp: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct VnStatDateUpdated {
    date: VnStatDate,
    time: VnStatTime,
    timestamp: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct VnStatTime {
    hour: u32,
    minute: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct VnStatDate {
    day: u32,
    month: u32,
    year: u32,
}

impl VnStats {
    fn fiveminute_total(vnstat_traffic: VnStatTraffic) -> VnStatTrafficTotal {
        let mut tx: u64 = 0;
        let mut rx: u64 = 0;
        for entry in vnstat_traffic.fiveminute {
            tx += entry.tx;
            rx += entry.rx;
        }
        VnStatTrafficTotal { tx, rx }
    }

    fn get_vnstats() -> Result<VnStats, reqwest::Error> {
        reqwest::blocking::get("http://127.0.0.1:8685/json.cgi")?.json()
    }

    fn get_interface(interface_name: &str) -> Result<Option<VnStatInterface>, Box<dyn Error>> {
        // Locate a given interface from the vnstats json output
        let vnstats: VnStats = Self::get_vnstats()?;

        let interface = vnstats
            .interfaces
            .iter()
            .find(|interface| interface.name == interface_name)
            .cloned();

        Ok(interface)
    }
}

fn main() {
    run().unwrap();
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);

    let help_message = "Help\nlist - List interfaces known to vnstat";
    if *&args.len() == 1 {
        println!("{help_message:?}");
    }

    if *&args.len() == 2 {
        if args[1] == "help" || args[1] == "-h" {
            println!("{help_message:?}");
        }

        if args[1] == "list" {
            let vnstats = VnStats::get_vnstats();
            match vnstats {
                Ok(vnstats) => {
                    for interface in vnstats.interfaces {
                        println!("{}", interface.name)
                    }
                }
                Err(e) => {
                    eprint!("Error getting vnstats: {}", e)
                }
            }
            exit(0);
        } else {
            // Assume wants stats for a given interface name
            let interface_name = &args[1];
            let vnstats: Result<Option<VnStatInterface>, Box<dyn Error + 'static>> =
                VnStats::get_interface(&interface_name);

            match vnstats {
                Ok(Some(interface)) => {
                    let traffic_usage = VnStats::fiveminute_total(interface.traffic);
                    let total_in_out_bytes: u64 = traffic_usage.tx + traffic_usage.rx;
                    let total_in_out_mb: f64 = total_in_out_bytes as f64 / 1_000_000.0;
                    let total_in_out_gb: f64 = total_in_out_bytes as f64/ 1_000_000_000.0;

                    println!(
                        "Last 5mins bandwidth usage for interface: '{}'\n\
                        tx: {} bytes\n\
                        rx: {} bytes\n\
                        Total in/out bytes: {} bytes\n\
                        Total in/out Mb: {:.2} Mb\n\
                        Total in/out Gb: {:.2} Gb",
                        interface.name,
                        traffic_usage.tx,
                        traffic_usage.rx,
                        total_in_out_bytes,
                        total_in_out_mb,
                        total_in_out_gb
                    );
                }
                Ok(None) => {
                    println!("No interface found with that name.");
                }
                Err(e) => {
                    eprintln!("Error getting interface traffic: {}", e);
                }
            }
        }
    }

    /*
     To handle:
     Result, could be an error
     Inside the Result is an Option, which could be None.
    */

    /*
    println!("{req:?}");


    println!("The vnstatversion is: {}", req.vnstatversion);
    println!("The jsonversion is: {}", req.jsonversion);
    let num_interfaces = req.interfaces.len();
    println!("The number of interfaces is: {}", num_interfaces);
    //println!("{req:#?}");
    let total = req.fiveminute_total();
    println!("fiveminuteTotal: {:?}", { total });
    */

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
