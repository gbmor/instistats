use std::env;
use std::fs;
use std::process;

use serde::{Deserialize, Serialize};
use serde_yaml;

const VERS: &str = "v0.1";
const CONF_PATH: &str = "instistats.yml";

#[derive(Debug, Deserialize, Serialize)]
struct Server {
    name: String,
    url: String,
    signup_url: String,
    user_count: Option<u32>,
    want_users: bool,
    admin_email: String,
    description: String,
    users: Option<Vec<User>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
    title: String,
    mtime: String,
}

fn main() {
    println!("instistats {}", VERS);
    println!("(c) 2019 Ben Morrison - ISC License");
    println!();
    let args = env::args().collect::<Vec<String>>();
    let out_path = match args[1].trim() {
        "-h" | "--help" => {
            println!("The only argument should be the path to save the tilde.json file.\nEx: /var/www/htdocs/tilde.json");
            process::exit(0);
        }
        out_path => {
            println!("Output Location: {}", out_path);
            out_path
        }
    };
    println!();

    let conf = fs::read_to_string(CONF_PATH).expect("Could not read config file");
    let conf_yaml: Server =
        serde_yaml::from_str(&conf).expect("Could not parse config data as YAML");

    println!("{:#?}", conf_yaml);
}
