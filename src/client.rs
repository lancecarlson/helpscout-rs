use std::thread;
use std::io::Read;
use std::time::Duration;

use reqwest::{self, StatusCode, Method, Url};
use reqwest::header::Headers;
use serde_json::{self, Value};

use error::HelpScoutError;

/// The HelpScout API Rust client.
///
/// This will allow the rest of this library to interact with the HelpScout API!
///
#[derive(Debug)]
pub struct Client {
    /// Configure the client to retry the request `retry_count` number of times
    /// when the service is unavailable.
    pub retry_count: u8,

    /// Duration of time to wait between retry attempts.
    pub retry_wait: u16,

    api_url: String,
    api_key: String,
    reqwest: reqwest::Client,
}

/// Status message returned by every API request.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub code: Option<i32>,
    pub error: String,
}

impl Client {
    /// Create a new client to the HelpScout service.
    pub fn new(api_key: &str) -> Client {
        Client {
            retry_count: 3,
            retry_wait: 250,
            api_url: "https://api.helpscout.net/v1".into(),
            api_key: api_key.into(),
            reqwest: reqwest::Client::new().expect("A reqwest client"),
        }
    }

    /// Send a `get` request to the HelpScout service. This is intended to be used
    /// by the library and not the user.
    pub fn get(&self, prefix: &str, path: &str, url_params: Option<Vec<(String, String)>>) -> Result<(Status, Value), HelpScoutError> {
        self.request(Method::Get, self.url(prefix, path, url_params), None)
    }

    /// Send a `post` request to the HelpScout service. This is intended to be used
    /// by the library and not the user.
    pub fn post(&self, prefix: &str, path: &str, url_params: Option<Vec<(String, String)>>, post_params: Option<Vec<(String, String)>>) -> Result<(Status, Value), HelpScoutError> {
        self.request(Method::Post, self.url(prefix, path, url_params), post_params)
    }

    fn url(&self, prefix: &str, path: &str, params: Option<Vec<(String, String)>>) -> Url {
        let base = format!("{api_url}/{prefix}/{path}",
                           api_url = self.api_url,
                           prefix = prefix,
                           path = path);
        match params {
            Some(params) => Url::parse_with_params(&base, params),
            None => Url::parse(&base),
        }.expect("Url to be valid")
    }

    //T: What is going on at the start here?
    fn request(&self, method: Method, url: Url, params: Option<Vec<(String, String)>>) -> Result<(Status, Value), HelpScoutError> {
        let mut count = self.retry_count;
        loop {
            let url = url.clone();
            let mut headers = Headers::new();
            headers.set_raw("X-HelpScout-API-Key", self.api_key.clone());
            let mut res = match params.clone() {
                Some(p) => self.reqwest.request(method.clone(), url)?.headers(headers).form(&p)?.send()?,
                None => self.reqwest.request(method.clone(), url)?.headers(headers).send()?,
            };

            let mut body = String::new();
            res.read_to_string(&mut body)?;

            // I wish could just check the content type but authy mixes json
            // and html content types when returning valid json.
            match serde_json::from_str::<Value>(&body) {
                Ok(mut value) => {
                    let status: Status = serde_json::from_value(value.clone())?;

                    match res.status() {
                        StatusCode::Ok => return Ok((status, value)),
                        StatusCode::BadRequest => return Err(HelpScoutError::BadRequest(status)),
                        StatusCode::Unauthorized => return Err(HelpScoutError::UnauthorizedKey(status)),
                        StatusCode::Forbidden => return Err(HelpScoutError::Forbidden(status)),
                        StatusCode::TooManyRequests => return Err(HelpScoutError::TooManyRequests(status)),
                        StatusCode::NotFound => return Err(HelpScoutError::UserNotFound(status)),
                        StatusCode::InternalServerError => return Err(HelpScoutError::InternalServerError(status)),
                        s => panic!("Status code not covered in HelpScout REST specification: {}", s),
                    };
                },
                Err(_) => {
                    match res.status() {
                        StatusCode::ServiceUnavailable => {
                            count -= 1;
                            if count == 0 {
                                return Err(HelpScoutError::ServiceUnavailable);
                            }
                                else {
                                    thread::sleep(Duration::from_millis(self.retry_wait.into()));
                                    continue;
                                }
                        },
                        _ => return Err(HelpScoutError::InvalidServerResponse),
                    }
                },
            };
        }
    }
}
