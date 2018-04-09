use std::thread;
use std::io::Read;
use std::time::Duration;

use reqwest::{self, StatusCode, Method, Url};
use reqwest::header::{Headers, Authorization, Basic, ContentType};
use serde;
use serde_json::{self, Value};
use serde_url_params;

// TODO: Maybe optionally compile these two?
use std::env;
use dotenv::dotenv;

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
    ///
    /// Example:
    ///
    /// ```rust
    /// use helpscout::Client;
    ///
    /// fn main() {
    ///     let api_key = env::var("HELPSCOUT_API_KEY").expect("to have HELPSCOUT_API_KEY set");
    ///     let client = Client::new(&api_key);
    ///     let mailboxes = mailboxes::list(&client);
    ///     assert!(mailboxes.items.len() > 0);
    /// }
    /// ```
    pub fn new(api_key: &str) -> Client {
        Client {
            retry_count: 3,
            retry_wait: 250,
            api_url: "https://api.helpscout.net/v1".into(),
            api_key: api_key.into(),
            reqwest: reqwest::Client::new(),
        }
    }

    #[doc(hidden)]
    pub fn example() -> Client {
        dotenv().ok();
        let api_key: String = env::var("HELPSCOUT_API_KEY").expect("to have HELPSCOUT_API_KEY set");
        Client::new(&api_key)
    }

    /// Send a `get` request to the HelpScout service. This is intended to be used
    /// by the library and not the user.
    pub fn get<T>(&self, path: &str, url_params: T) -> Result<Value, HelpScoutError>
        where T: serde::Serialize
    {
        self.request(Method::Get, self.url(path, url_params)?, None)
    }

    /// Send a `post` request to the HelpScout service. This is intended to be used
    /// by the library and not the user.
    pub fn post<T>(&self, path: &str, url_params: T, body: Option<String>) -> Result<Value, HelpScoutError>
        where T: serde::Serialize
    {
        self.request(Method::Post, self.url(path, url_params)?, body)
    }

    /// Send a `put` request to the HelpScout service. This is intended to be used
    /// by the library and not the user.
    pub fn put<T>(&self, path: &str, url_params: T, body: Option<String>) -> Result<Value, HelpScoutError>
        where T: serde::Serialize
    {
        self.request(Method::Put, self.url(path, url_params)?, body)
    }

    fn url<T>(&self, path: &str, params: T) -> Result<Url, HelpScoutError>
        where T: serde::Serialize
    {
        let mut base = format!("{api_url}/{path}",
                           api_url = self.api_url,
                           path = path);

        let encoded = serde_url_params::to_string(&params)?;
        base = format!("{}?{}", base, encoded);
        debug!("{}", base);
        Ok(Url::parse(&base)?)
    }

    fn request(&self, method: Method, url: Url, request_body: Option<String>) -> Result<Value, HelpScoutError> {
        let mut count = self.retry_count;
        loop {
            let url = url.clone();

            debug!("Attempting request - Method: {}. Url: {}", method, url);

            let mut headers = Headers::new();
            let credentials = Basic {
                username: self.api_key.clone(),
                password: Some("X".into()),
            };
            headers.set(Authorization(credentials));
            headers.set(ContentType::json());
            let mut res = match request_body.clone() {
                Some(b) => {
                    println!("Request body - {}", b);
                    self.reqwest.request(method.clone(), url).headers(headers).body(b).send()?
                },
                None => self.reqwest.request(method.clone(), url).headers(headers).send()?,
            };

            let mut body = String::new();
            res.read_to_string(&mut body)?;

            debug!("Response body: {}", body);
            debug!("Response status: {}", res.status());
            match serde_json::from_str::<Value>(&body) {
                Ok(value) => {
                    let status = serde_json::from_value(value.clone());

                    match res.status() {
                        StatusCode::Ok => return Ok(value),
                        StatusCode::Created => return Ok(value),
                        StatusCode::BadRequest => return Err(HelpScoutError::BadRequest(status?)),
                        StatusCode::Unauthorized => return Err(HelpScoutError::UnauthorizedKey(status?)),
                        StatusCode::Forbidden => return Err(HelpScoutError::Forbidden(status?)),
                        StatusCode::TooManyRequests => return Err(HelpScoutError::TooManyRequests(status?)),
                        StatusCode::NotFound => return Err(HelpScoutError::UserNotFound(status?)),
                        StatusCode::InternalServerError => return Err(HelpScoutError::InternalServerError(status?)),
                        s => panic!("Status code not covered in HelpScout REST specification: {}", s),
                    };
                },
                Err(_) => {
                    debug!("Response status: {}",res.status());
                    if res.status()==StatusCode::Ok||res.status()==StatusCode::Created {
                        return Ok(serde_json::Value::String("Ok".into()));
                    } else {
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
                    }
                },
            };
        }
    }
}
