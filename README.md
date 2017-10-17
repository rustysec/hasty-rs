# Hasty
A completely naive HTTP library without async, but with XP support!

## Description
If you're looking for a high performance modern http library, you probably want [hyper](https://www.hyper.rs). However, currently [mio](https://github.com/carllerche/mio) (and subsequently [tokio](https://github.com/tokio-rs/tokio)) rely on some windows api's under the hood that are not present on XP, thus will not be compatible with those platforms. Hasty is a very simple library designed to solve very simple requirements.

## Usage
*This api is not currently stablized and will probably change in the future.*

#### Configuration
The connection configuration allows you to set certain options, currently only https certificate bypass is implemented:

Standard configuration with https validation enabled:
```rust
extern crate hasty;
use hasty::Config;
let config = Config::new();
```

To disable:
```rust
extern crate hasty;
use hasty::Config;
let config = Config::new();
config.disable_https_security();
```

#### Requests
A request object stores the remote host, url, path, headers, and body information the client will send. 

```rust
extern crate hasty;
use hasty::{Config,Request};
let url = url::Url::parse("http://www.rust-lang.org").unwrap();
let req = Request::from_url(url);
```

#### Responses
A response object parses the data returned by a the server for further consumption.

```rust
extern crate hasty;
use hasty::{Config,Request,Response,request};
let url = url::Url::parse("http://www.rust-lang.org").unwrap();
let req = Request::from_url(url);
let res = request("http://www.rust-lang.org".parse().unwrap(), req);
println!("{}", String::from_utf8(res.body()).unwrap());
```

## Examples
The examples folder contains getting started code which can be easily adapted for your needs!

## Issues
Please feel free to open up issues around any breakages you find. I don't intend for Hasty to take over the world, but I do want it to work for as many people as see value in it.