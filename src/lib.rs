mod parse;

use std::error::Error;
use tiny_http::{Server, Response, Header};
use crate::ServeContent::{PageContent, HtmlContent, RawContent};

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
    PageContent(Vec<String>),
}

type ContentClosure = Box<dyn Fn(&str) -> Response<std::io::Cursor<Vec<u8>>>>;

//
fn raw_string_closure(text: String) -> ContentClosure {
    Box::new(move |_: &str| -> Response<_> {
        Response::from_string(&text)
    })
}

fn html_string_closure(html: String) -> ContentClosure {
    Box::new(move |_: &str| -> Response<_> {
        let html_header: Header = "Content-Type: text/html".parse().unwrap();
        Response::from_string(&html).with_header(html_header)
    })
}

fn page_closure(_content: Vec<String>) -> ContentClosure {
    html_string_closure(String::from("Unimplemented"))
}

// Server main
pub fn run(config: ServeConfig) -> Result<(), Box<dyn Error>> {
    // Start server
    let server = Server::http(config.listening_addr()).expect("Failed to bind to port");

    // Closure
    let closure = match config.content {
        RawContent(text) => raw_string_closure(text),
        HtmlContent(html) => html_string_closure(html),
        PageContent(pages) => page_closure(pages),
    };

    // Listen
    for request in server.incoming_requests() {
        println!("received request! method: {:?}, url: {:?}",
                 request.method(),
                 request.url()
        );

        // Run callback
        let response = closure(request.url());

        // Send request
        request.respond(response).expect("Failed to respond to request");
    }

    Ok(())
}