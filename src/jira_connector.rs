use std::{collections::HashMap, io::Error};

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

    pub fn download_ticket(&self, ticket_number: &str) -> String {
        let client = reqwest::blocking::Client::new();
        let res = client.get(&self.url)
            .basic_auth(&self.user, Some(&self.api_key))
            .send().unwrap();
    
        let json  = res.text();
        println!("{:?}", json);

        println!("{}", ticket_number);
        ticket_number.to_owned()
    }
}
