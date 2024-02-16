use std::error::Error;
use tiny_http::{Server, Response, Header};
use crate::ServeContent::{HtmlContent, RawContent};

// Configuration definition
pub struct ServeConfig {
    content: ServeContent,
    port: String,
}

pub enum ServeContent {
    RawContent(String),
    HtmlContent(String),
}

impl ServeConfig {
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<ServeConfig, &'static str> {
        args.next();
        let port = String::from("0.0.0.0:3000");
        match args.next() {
            None => Err("No content"),
            Some(content) => Ok(ServeConfig { content: RawContent(content), port })
        }
    }
}

// Server main
pub fn run(config: ServeConfig) -> Result<(), Box<dyn Error>> {
    // Start server
    let server = Server::http(&config.port).expect("Failed to bind to port");

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