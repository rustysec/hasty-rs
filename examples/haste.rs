extern crate hasty;

use hasty::{Config, Hasty, Request};

fn main() {
    let mut hasty = Hasty::new_with_config(Config::new().disable_https_security());

    match hasty.request("http://localhost:3000/basic_get", Request::new()) {
        Ok(resp) => {
            println!("Response: {}", String::from_utf8(resp.body()).unwrap());
        },
        Err(e) => println!("Error: {}", e.to_string())
    }
}