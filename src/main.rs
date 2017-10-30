extern crate mysql;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate clap;

mod config;
mod server;

use config::Config;
use server::Server;
use clap::{Arg, App};

fn main() {

    let matches = App::new("MySql RSU Tool")
        .version("1.0")
        .author("Tim Glabisch. <mysql-rsu@tim.ainfach.de>")
        .about("MySQL RSU Tool")
        .arg(Arg::with_name("query")
            .help("query")
            .long("query")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("db")
            .help("db")
            .long("db")
            .required(true)
            .takes_value(true))
        .get_matches();


    let config = Config::from_file("config.toml").expect("could not parse config.toml");

    println!("config: {:#?}", &config);

    let query = matches.value_of("query").expect("query not given").to_string();
    let db = matches.value_of("db").expect("db not given").to_string();

    let mut servers = config.server.iter().map(|c| {
        Server::new(c.clone(), c.get_name()).expect("could not create server from config")
    }).collect::<Vec<Server>>();

    if servers.len() < 3 {
        println!("need at least 3 servers.");
        return;
    }

    println!("servers: {}", servers.len());

    for server in servers.iter_mut() {
        
        server.query(&format!("USE {};", &db)).expect("could select db");
        server.query("SET SESSION wsrep_OSU_method=RSU;").expect("query_failed");
        
        let foo : Vec<usize> = server.conn().prep_exec("show variables like 'wsr%'", ()).map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                println!("{:?}", row);
                1
            }).collect()
        }).unwrap();
        //server.query(&query).expect("query failed.");

    }


    println!("Hello, world!");
}
