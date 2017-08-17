extern crate reqwest;
extern crate env_logger;

use reqwest::{Url, Method, header};
use std::time::SystemTime;


struct TestClient {
    client: reqwest::Client,
}


impl TestClient {
    fn new() -> Result<Self, reqwest::Error> {
        Ok(TestClient {
            client: reqwest::Client::new()?
        })
    }

    fn exec(&self) -> Result<reqwest::StatusCode, reqwest::Error> {
        let url = Url::parse("https://api.easypost.com/health/ok").unwrap();
        let response = self.client.request(Method::Get, url)?
            .header(header::Date(header::HttpDate::from(SystemTime::now())))
            .header(header::UserAgent::new("reqwest-panic-demo/1.0"))
            .send()?;
        Ok(response.error_for_status()?.status())
    }
}


fn main() {
    env_logger::init().unwrap();
    let client = TestClient::new().expect("should be able to construct result");
    match client.exec() {
        Ok(s) => println!("done: HTTP status is {:?}", s),
        Err(e) => eprintln!("error: {:?}", e)
    };
}
