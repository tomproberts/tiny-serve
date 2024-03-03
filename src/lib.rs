mod parse;

use std::error::Error;
use std::fs;
use tiny_http::{Server, Response, Header};
use crate::ServeContent::{PageContent, HtmlContent, RawContent};

// Configuration definition
pub struct ServeConfig {
    content: ServeContent,
    port: u16,
    serve_files: bool,
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

fn page_closure(pages: Vec<String>) -> ContentClosure {
    Box::new(move |path: &str| -> Response<_> {
        let requested = String::from(&path[1..]);
        let mut response = Response::from_string("");
        let mut status = 200;
        for page in &pages {
            if &requested == page || page == "." {
                match fs::read(&requested) {
                    Ok(file) => response = Response::from_data(file),
                    Err(_) => status = 404
                };
                break;
            }
        }

        let html_header: Header = "Content-Type: text/html".parse().unwrap();
        response.with_header(html_header).with_status_code(status)
    })
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
        // Run callback
        let response = closure(request.url());

        // Log
        println!("{:?}: {:?} {:?}",
                 response.status_code(),
                 request.method(),
                 request.url()
        );

        // Send request
        request.respond(response).expect("Failed to respond to request");
    }

    Ok(())
}