use std::{thread, time};

use public_ip;

mod config;
use config::*;

fn make_url(host: &str, domain_name: &str, ddns_password: &str, your_ip: &str) -> String {
    format!("https://dynamicdns.park-your-domain.com/update?host={host}&domain={domain_name}&password={ddns_password}&ip={your_ip}")
}

fn take_five(seconds: u64) {
    let wait_time = time::Duration::from_secs(seconds);
    thread::sleep(wait_time);
}

#[tokio::main]
async fn main() {
    // let mut recorded_ip = "".to_string();
    let mut recorded_ip = OLD_IP.to_string();

    loop {
        if let Some(current_ip) = public_ip::addr_v4().await {
            let current_ip = current_ip.to_string();

            if recorded_ip == current_ip {
                take_five(24 * 3600);
                continue
            }

            let url = make_url(HOST, DOMAIN_NAME, DDNS_PASSWORD, current_ip.as_str());

            let resp = reqwest::get(url)
                .await.expect("Oops, something went wrong 1")
                .text()
                .await.expect("oops, something went wrong 2");

            if resp.contains(&current_ip) {
                recorded_ip = current_ip;
            }

            println!("{:#?}", resp);
            take_five(24 * 3600);
        } else {
            println!("Something went terribly wrong!");
            take_five(3600);
        }
    }
}
