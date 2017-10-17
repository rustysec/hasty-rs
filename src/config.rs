use std::net::TcpStream;

use url::Url;

pub trait Connection {
    fn connect(url: &Url) -> Result<TcpStream, String>;
}

#[derive(Clone)]
pub struct Config {
    allow_insecure_https: bool,
}

impl Config {
    /// Returns a standard client configuration, only allowing http and valid https
    pub fn new() -> Config {
        Config {
            allow_insecure_https: false,
        }
    }

    /// Disables SSL/TLS certificate validation during requests
    ///
    /// # Example
    ///
    /// ```
    /// use hasty::{Config,Hasty};
    /// let conf = Config::new().disable_https_security();
    /// let cli = Hasty::new_with_config(conf);
    /// ```
    pub fn disable_https_security(mut self) -> Self {
        self.allow_insecure_https = true;
        self
    }
    
    /// Returns whether or not the configuration allows insecure https connections
    ///
    /// # Example
    ///
    /// ```
    /// use hasty::Config;
    ///
    /// // default is to validate security
    /// let conf = Config::new();
    /// assert_eq!(conf.allows_insecure_https(), false);
    ///
    /// // turn off https security
    /// let conf = Config::new().disable_https_security();
    /// assert_eq!(conf.allows_insecure_https(), true);
    /// ```
    pub fn allows_insecure_https(&self) -> bool {
        self.allow_insecure_https
    }
}

#[test]
pub fn config_default() {
    let config = Config::new();
    assert_eq!(config.allows_insecure_https(), false);
}

#[test]
pub fn config_insecure_https() {
    let config = Config::new().disable_https_security();
    assert_eq!(config.allows_insecure_https(), true);
}