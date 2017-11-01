extern crate hasty;

use hasty::{Config, Hasty, Request, HttpMethods};

fn main() {
    let mut hasty = Hasty::new_with_config(Config::new().disable_https_security());
    let mut request = Request::from_url("https://localhost:3001/basic_post".parse().unwrap());
    request.with_method(HttpMethods::Post);
    request.with_body(Some("snerple".to_owned().as_bytes().to_vec()));

    match hasty.request(request) {
        Ok(resp) => {
            println!("Response: {}", String::from_utf8(resp.body()).unwrap());
        },
        Err(e) => println!("Error: {}", e.to_string())
    }
}
