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
    title: String,
    task_type: String,
    ticket_number: String,
    description: String
}

impl Ticket {
    pub fn new(json_ticket: serde_json::Value) -> Ticket {
        Ticket {
            title: json_ticket["fields"]["summary"].to_string(),
            task_type: json_ticket["fields"]["issuetype"]["name"].to_string(),
            ticket_number: json_ticket["key"].to_string(),
            description: json_ticket["fields"]["description"].to_string()
        }
    }

    pub fn print_ticket(&self) {
        println!("Ticket name: {}", &self.title.trim_matches('"'));
        println!("Ticket number: {}", &self.ticket_number.trim_matches('"'));
        println!("Ticket type: {}", &self.task_type.trim_matches('"'));
        println!("Description");
        println!("{}", &self.description.trim_matches('"'));
    }
}

