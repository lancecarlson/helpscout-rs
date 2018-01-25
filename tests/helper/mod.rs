
use std::sync::{Once, ONCE_INIT};
use std::env;

use dotenv::dotenv;
use env_logger;

use super::helpscout::Client;

static INIT: Once = ONCE_INIT;

pub fn setup() -> Client {
    INIT.call_once(|| {
        dotenv().ok();
        env_logger::init();
    });
    let api_key = env::var("API_KEY").expect("to have API_KEY set");
    Client::new(&api_key)
}
