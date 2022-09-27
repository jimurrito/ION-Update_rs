// To-do list
// [X] - finish error handling (Reqwest + General)
// [*] - add ability to pull configuration from cli arguments. (Using Env instead)
// [0] - boot config dump and logo :)

mod config;
mod errors;
mod req_temp;

use config::{get_ip, Configuration};
use errors::{ERR3, ERR5, ERR6};
use log::{debug, error, info, warn};
use req_temp::{ZConfig, ZDump};
use simple_logger::SimpleLogger;
use std::{env, thread, time};

// App Constants
const ZONE_BASE: &str = "https://api.hosting.ionos.com/dns/v1/zones/";

fn main() {
    //Logo
    logo();
    // Dynamic Log Level
    let raw_lglvl = get_env_var("LOG_LEVEL");
    // "info" or "debug"
    if raw_lglvl == "info" {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Info)
            .init()
            .unwrap();
    } else if raw_lglvl == "debug" {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Debug)
            .init()
            .unwrap();
    } else {
        panic!("{ERR6}")
    }
    // Pulls IONOS Parameters from the docker set Env vars + Error handling
    // Scope
    let scope = make_vec_string(get_env_var("SCOPE"));
    env_val_vec(&scope, "SCOPE", "unset");
    // Pub key
    let pub_key = get_env_var("PUBKEY");
    env_val_str(&pub_key, "PUBKEY", "");
    // Prv key
    let prv_key = get_env_var("PRVKEY");
    env_val_str(&prv_key, "PRVKEY", "");
    'mainloop: loop {
        // MAIN APP
        info!("Initalization");
        // Creates Configuration type object
        // Attempts to get current IP before creating configuration object
        let config = Configuration::new(
            scope.to_owned(),
            pub_key.to_owned(),
            prv_key.to_owned(),
            find_ip(),
        );
        info!(
            "Scope: {:?} | Current Ip {}",
            &config.scope, &config.curr_ip
        );
        debug!("{:#?}", &config);
        // gets API zones && Deserializes API response, making it iterable
        let response_raw: String = match config.ionos_get(ZONE_BASE) {
            // [EH] Error handling for Ionos Get
            Option::Some(o) => o,
            Option::None => {
                error!("{}", errors::ERR1);
                sleep(30);
                continue 'mainloop;
            }
        };
        // Deserialize json response, from get request, into a struct
        let response_parse = match serde_json::from_str::<ZDump>(&response_raw) {
            // [EH] Deserialization error handling
            Result::Ok(o) => o,
            Result::Err(_) => {
                error!("{}", errors::ERR2);
                sleep(30);
                continue 'mainloop;
            }
        };
        // Iterates through Zones found in Dump
        for i in &response_parse {
            // Skips iteration if Zone is not in scope
            if !&config.scope.contains(&i.name) {
                continue;
            }
            // From here, domain is in-scope for update
            info!("Checking records for {}", &i.name);
            //Get DNS Zone config
            let zone_url = String::from(ZONE_BASE) + &i.id;
            debug!("{}", &zone_url);
            // get zone config
            let zresponse_raw = match config.ionos_get(&zone_url) {
                // [EH] Error handling for Ionos Get
                Option::Some(o) => o,
                Option::None => {
                    error!("{}", errors::ERR1);
                    sleep(30);
                    continue 'mainloop;
                }
            };
            let zone_config = match serde_json::from_str::<ZConfig>(&zresponse_raw)
        // [EH] Deserialzation error handling
        {
            Result::Ok(o) => o,
            Result::Err(_) => {
                error!("{}", errors::ERR2);
                sleep(30);
                continue 'mainloop;
            }
        };
            // find and updates OOD wildcard records
            for mut r in zone_config.records {
                // Skips it if not wildcard or not A Rec (Filters out IPV6) or IPs matches
                if !r.is_wildcard() {
                    continue;
                }
                // Skips iter when IP is already up to date
                if &config.curr_ip == &r.content {
                    info!("Zone: {} is already up to date", &i.name);
                    continue;
                }
                debug!("{:#?}", r);
                // Sets current IP into object
                r.content = config.curr_ip.clone();
                // Serialize {r} back into json for put request
                let rec_json = match serde_json::to_string(&r) {
                    Result::Ok(o) => o,
                    Result::Err(_) => {
                        error!("{}", errors::ERR4);
                        sleep(30);
                        continue 'mainloop;
                    }
                };
                // Pushing update
                info!("Updating wildcard record for {}", &i.name);
                // Generates record url
                let rec_url = { zone_url.clone() + "/records/" + &r.id };
                debug!("{}", rec_url);
                // Put update of record into IONOS
                let put_response = match config.ionos_put(rec_url.as_str(), rec_json) {
                    // Error handling for
                    Option::Some(o) => o,
                    Option::None => {
                        error!("{}", errors::ERR1);
                        sleep(30);
                        continue 'mainloop;
                    }
                };
                debug!("{:#?}", put_response);
                info!("Zone: {} has been successfully updated", &i.name);
            }
        }
        info!("In-scope zones have been updated Successfully. Entering idel state for 4hrs.");
        // Idel sleep for 4hrs (Placeholder time)
        sleep(14400);
    }
}

fn find_ip() -> String {
    let mut count = 0;
    loop {
        count += 1;
        match get_ip() {
            Option::Some(o) => {
                return o;
            }
            Option::None => {
                error!("{}", errors::ERR0);
                //Sleeps 30s if IP is not available
                thread::sleep(time::Duration::from_secs(30));
                warn!("Re-attempt #{} to get current IP", count);
            }
        }
    }
}

fn sleep(sec: u64) {
    thread::sleep(time::Duration::from_secs(sec));
}

fn get_env_var(key: &str) -> String {
    match env::var_os(&key) {
        Some(val) => match val.into_string() {
            Result::Ok(o) => o,
            Result::Err(o) => {
                panic!("{o:#?}");
            }
        },
        None => {
            panic!("{key}: {ERR3}");
        }
    }
}

fn env_val_vec(vec: &Vec<String>, key: &str, val: &str) {
    if vec.contains(&val.to_string()) {
        panic!("{key} was not set in docker. {}", ERR5);
    }
}

fn env_val_str(str: &str, key: &str, unset: &str) {
    if str == unset {
        panic!("{key} was not set in docker. {}", ERR5);
    }
}

fn make_vec_string(input: String) -> Vec<String> {
    input.split(",").map(str::to_string).collect()
}

fn logo() {
    println!(
        r#"
    ________  _   __      __  __          __      __                     
   /  _/ __ \/ | / /     / / / /___  ____/ /___ _/ /____       __________
   / // / / /  |/ /_____/ / / / __ \/ __  / __ `/ __/ _ \     / ___/ ___/
 _/ // /_/ / /|  /_____/ /_/ / /_/ / /_/ / /_/ / /_/  __/    / /  (__  ) 
/___/\____/_/ |_/      \____/ .___/\__,_/\__,_/\__/\___/____/_/  /____/  
                           /_/                        /_____/            
    
    Build: {}
    Built by: Jimurrito - https://github.com/jimurrito/
    https://hub.docker.com/r/jimurrito/ionupdate_rs

    *-----------------------------------------*
    |Rebuilt in Rust; as all things should be!|
    *----------- -----------------------------*
        _~^~^~_ V
    \) /  o o  \ (/
      '_   -   _'
      / '-----' \

    "#,
        get_env_var("VER")
    )
}
