mod parse;

use std::error::Error;
use tiny_http::{Server, Response, Header};
use crate::ServeContent::{HtmlContent, RawContent};

// Configuration definition
pub struct ServeConfig {
    content: ServeContent,
    port: u16,
}

pub const DEFAULT_PORT: u16 = 3000;
impl ServeConfig {
    pub fn listening_addr(&self) -> String {
        let mut addr = String::from("0.0.0.0:");
        addr.push_str(&self.port.to_string());
        addr
    }
}

#[derive(Debug, PartialEq)]
pub enum ServeContent {
    RawContent(String),
    HtmlContent(String),
}

// Server main
pub fn run(config: ServeConfig) -> Result<(), Box<dyn Error>> {
    // Start server
    let server = Server::http(config.listening_addr()).expect("Failed to bind to port");

    // Listen
    for request in server.incoming_requests() {
        println!("received request! method: {:?}, url: {:?}",
                 request.method(),
                 request.url()
        );

        let mut response;
        match &config.content {
            RawContent(content) => {
                response = Response::from_string(content);
            }
            HtmlContent(content) => {
                let html_header: Header = "Content-Type: text/html".parse().unwrap();
                response = Response::from_string(content);
                response.add_header(html_header);
            }
        }

        // Send request
        request.respond(response).expect("Failed to respond to request");
    }

    Ok(())
}