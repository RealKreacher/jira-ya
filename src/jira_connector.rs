use serde_json::{self, Value};

pub struct Connector {
    url: String,
    api_key: String,
    user: String,
}

impl Connector {

    pub fn new(url: String, api_key: String, user: String) -> Connector {
       Connector {
           url,
           api_key,
           user
       }
    }

    pub fn download_ticket(&self) -> Value {
        let client = reqwest::blocking::Client::new();

        let res = client.get(&self.url)
            .basic_auth(&self.user, Some(&self.api_key))
            .send()
            .expect("Couldn't receive a response")
            .json::<serde_json::Value>()
            .expect("Couldn't parse the response");

        res
    }
}

pub struct Ticket {
    title: String
}

impl Ticket {
    pub fn new(json_ticket: serde_json::Value) -> Ticket {
        Ticket {
            title: json_ticket["key"].to_string()
        }
    }

    pub fn to_string(&self) -> String {
        self.title.clone()
    }
}

