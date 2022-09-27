use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

#[derive(Debug)]
pub struct Configuration {
    pub scope: Vec<String>,
    api_key: String,
    pub curr_ip: String, //keep as Option result and not string so we can error handle for none varient.
}

impl Configuration {
    // New Struct
    pub fn new(scope: Vec<String>, pubky: String, prvky: String, c_ip: String) -> Self {
        Self {
            scope: scope,
            api_key: pubky + "." + &prvky, // + operator needs a owned var on the very right side.
            curr_ip: c_ip,
        }
    }

    // get request to IONOS
    pub fn ionos_get(&self, url: &str) -> Option<String> {
        // Methods must use &self, or else parent object will be dropped.
        match reqwest::blocking::Client::new()
            .get(url)
            .headers(self.make_header().to_owned())
            .send()
        {
            Result::Ok(o) => Some(o.text().unwrap()),
            Result::Err(_) => None,
        }
    }

    // get request to IONOS
    pub fn ionos_put(&self, url: &str, input: String) -> Option<String> {
        // Methods must use &self, or else parent object will be dropped.
        match reqwest::blocking::Client::new()
            .put(url)
            .headers(self.make_header().to_owned())
            .body(input)
            .send()
        {
            Result::Ok(e) => Some(e.text().unwrap()),
            Result::Err(_) => None,
        }
    }

    // Generates header for Reqwest
    fn make_header(&self) -> HeaderMap {
        let mut map = HeaderMap::new();
        map.insert(
            HeaderName::from_static("x-api-key"),
            HeaderValue::from_str(self.api_key.as_str()).unwrap(),
        );
        map.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("application/json"),
        );
        map.insert(
            HeaderName::from_static("user-agent"),
            HeaderValue::from_static("reqwest/0.11"),
        );
        map
    }
}

pub fn get_ip() -> Option<String> {
    // gets Public IP, returns Option's (Some, None)
    match reqwest::blocking::get("https://ifconfig.me") {
        Result::Ok(e) => Some(e.text().unwrap()),
        Result::Err(_) => None,
    }
}


