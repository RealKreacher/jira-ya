mod jira_connector;

use jira_connector::Ticket;
use crate::jira_connector::Connector;
use clap::Parser;

#[derive(Parser, Default, Debug)]
#[clap(author = "Kracher", version, about)]
/// Simple CLI program for working with jira
struct Arguments {
    /// Whole url of the ticket
    /// Example: https://project-name.atlassian.net/rest/api/latest/issue/ticket-number
    #[clap(short = 't', long = "ticket")]
    ticket_url: String,
    /// API key to use for the connection.
    /// Can be generated in jira web application
    #[clap(short = 'a', long = "api-key")]
    api_key: String,
    /// User name paired with the api key.
    /// Example: foo@bar.com
    #[clap(short = 'u', long = "user")]
    user: String,
}

fn main() {
    let args = Arguments::parse();

    println!("{:?}", args);
    let url = args.ticket_url;
    let api_key = args.api_key;
    let user = args.user;

    let connector = Connector::new(url, api_key, user);

    let json_ticket = connector.download_ticket();
    let ticket = Ticket::new(json_ticket);

    ticket.print_ticket(); 
    
}
